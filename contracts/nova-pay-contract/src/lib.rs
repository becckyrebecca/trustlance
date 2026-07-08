#![no_std]

use soroban_sdk::{
    contract, contractimpl, token, Address, Bytes, BytesN, Env, Symbol, String, Vec,
};

mod types;
mod error;
mod storage;
mod helper;

#[cfg(test)]
mod test;

use types::{
    GlobalStats, Merchant, MerchantCategory, MultisigPayment, MultisigStatus, Order,
    PaymentFilter, PaymentRecord, PaymentStatus, PaymentStatusFilter, RefundRequest, RefundStatus,
    SortField, SortOrder,
};
use error::ContractError;

fn query_payment_history(
    e: &Env,
    order_ids: Vec<String>,
    cursor: Option<String>,
    limit: u32,
    filter: Option<PaymentFilter>,
    sort_field: SortField,
    sort_order: SortOrder,
) -> Vec<PaymentRecord> {
    let mut filtered_payments = Vec::new(e);

    // 1. Retrieve all records and apply filters
    for id in order_ids.iter() {
        if let Some(record) = storage::get_payment(e, &id) {
            let mut matches = true;

            if let Some(ref f) = filter {
                if let Some(min) = f.amount_min {
                    if record.amount < min {
                        matches = false;
                    }
                }
                if let Some(max) = f.amount_max {
                    if record.amount > max {
                        matches = false;
                    }
                }
                if let Some(ref tok) = f.token {
                    if record.token != *tok {
                        matches = false;
                    }
                }
                if let Some(start) = f.date_start {
                    if record.paid_at < start {
                        matches = false;
                    }
                }
                if let Some(end) = f.date_end {
                    if record.paid_at > end {
                        matches = false;
                    }
                }
                match f.status {
                    PaymentStatusFilter::Any => {}
                    PaymentStatusFilter::Completed => {
                        if record.status != PaymentStatus::Completed {
                            matches = false;
                        }
                    }
                    PaymentStatusFilter::PartiallyRefunded => {
                        if record.status != PaymentStatus::PartiallyRefunded {
                            matches = false;
                        }
                    }
                    PaymentStatusFilter::FullyRefunded => {
                        if record.status != PaymentStatus::FullyRefunded {
                            matches = false;
                        }
                    }
                }
            }

            if matches {
                filtered_payments.push_back(record);
            }
        }
    }

    // 2. Sort the filtered payments using a simple bubble sort
    let len = filtered_payments.len();
    if len > 1 {
        for i in 0..len {
            for j in 0..len - i - 1 {
                let p1 = filtered_payments.get(j).unwrap();
                let p2 = filtered_payments.get(j + 1).unwrap();
                let should_swap = match sort_field {
                    SortField::Date => {
                        if sort_order == SortOrder::Ascending {
                            p1.paid_at > p2.paid_at
                        } else {
                            p1.paid_at < p2.paid_at
                        }
                    }
                    SortField::Amount => {
                        if sort_order == SortOrder::Ascending {
                            p1.amount > p2.amount
                        } else {
                            p1.amount < p2.amount
                        }
                    }
                };
                if should_swap {
                    filtered_payments.set(j, p2);
                    filtered_payments.set(j + 1, p1);
                }
            }
        }
    }

    // 3. Paginate based on cursor and limit
    let mut paginated = Vec::new(e);
    let mut start_idx = 0;

    if let Some(cursor_id) = cursor {
        let mut found = false;
        for i in 0..filtered_payments.len() {
            if filtered_payments.get(i).unwrap().order_id == cursor_id {
                start_idx = i + 1;
                found = true;
                break;
            }
        }
        if !found {
            return paginated;
        }
    }

    let end_idx = core::cmp::min(start_idx + limit, filtered_payments.len());
    for i in start_idx..end_idx {
        paginated.push_back(filtered_payments.get(i).unwrap());
    }

    paginated
}

#[contract]
pub struct NovaPayContract;

#[contractimpl]
impl NovaPayContract {
    pub fn set_admin(e: Env, admin: Address) -> Result<(), ContractError> {
        if storage::has_admin(&e) {
            let current_admin = storage::get_admin(&e).unwrap();
            current_admin.require_auth();
        }
        storage::set_admin(&e, &admin);
        Ok(())
    }

    pub fn register_merchant(
        e: Env,
        merchant_address: Address,
        name: String,
        description: String,
        contact_info: String,
        category: MerchantCategory,
    ) -> Result<(), ContractError> {
        merchant_address.require_auth();

        if storage::has_merchant(&e, &merchant_address) {
            return Err(ContractError::MerchantAlreadyExists);
        }

        let merchant = Merchant {
            address: merchant_address.clone(),
            name,
            description,
            contact_info,
            category,
            registered_at: e.ledger().timestamp(),
        };

        storage::set_merchant(&e, &merchant_address, &merchant);

        // Add to merchant list
        let mut merchants = storage::get_merchant_list(&e);
        merchants.push_back(merchant_address.clone());
        storage::set_merchant_list(&e, &merchants);

        // Update global stats
        let mut stats = storage::get_global_stats(&e);
        stats.active_merchants_count += 1;
        storage::set_global_stats(&e, &stats);

        // Publish event
        e.events().publish(
            (Symbol::new(&e, "merchant_registered"), merchant_address),
            (),
        );

        Ok(())
    }

    pub fn process_payment_with_signature(
        e: Env,
        payer: Address,
        order: Order,
        signature: Bytes,
        merchant_public_key: BytesN<32>,
    ) -> Result<(), ContractError> {
        payer.require_auth();

        // Verify that merchant exists
        if !storage::has_merchant(&e, &order.merchant_address) {
            return Err(ContractError::MerchantDoesNotExist);
        }

        // Verify order has not been paid
        if storage::get_payment(&e, &order.order_id).is_some() {
            return Err(ContractError::PaymentAlreadyExists);
        }

        if order.amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        // Verify the signature
        helper::verify_signature(&e, &order, &signature, &merchant_public_key)?;

        // Perform transfer
        let token_client = token::Client::new(&e, &order.token);
        token_client.transfer(&payer, &order.merchant_address, &order.amount);

        // Create payment record
        let paid_at = e.ledger().timestamp();
        let cleanup_period = storage::get_cleanup_period(&e);
        let record = PaymentRecord {
            order_id: order.order_id.clone(),
            payer: payer.clone(),
            merchant_address: order.merchant_address.clone(),
            amount: order.amount,
            token: order.token.clone(),
            status: PaymentStatus::Completed,
            refunded_amount: 0,
            paid_at,
            expiry: paid_at + cleanup_period,
        };

        storage::set_payment(&e, &order.order_id, &record);

        // Indexing
        let mut all_payments = storage::get_all_payments(&e);
        all_payments.push_back(order.order_id.clone());
        storage::set_all_payments(&e, &all_payments);

        let mut merchant_payments = storage::get_merchant_payments(&e, &order.merchant_address);
        merchant_payments.push_back(order.order_id.clone());
        storage::set_merchant_payments(&e, &order.merchant_address, &merchant_payments);

        let mut payer_payments = storage::get_payer_payments(&e, &payer);
        payer_payments.push_back(order.order_id.clone());
        storage::set_payer_payments(&e, &payer, &payer_payments);

        // Global Stats
        let mut stats = storage::get_global_stats(&e);
        stats.total_payments_count += 1;
        stats.total_volume_processed += order.amount;
        storage::set_global_stats(&e, &stats);

        // Event
        e.events().publish(
            (Symbol::new(&e, "payment_processed"), order.order_id.clone()),
            (payer, order.merchant_address, order.amount),
        );

        Ok(())
    }

    pub fn initiate_refund(
        e: Env,
        caller: Address,
        refund_id: String,
        order_id: String,
        amount: i128,
        reason: String,
    ) -> Result<(), ContractError> {
        caller.require_auth();

        // Retrieve payment record
        let payment = storage::get_payment(&e, &order_id).ok_or(ContractError::PaymentDoesNotExist)?;

        // Verify caller is either payer or merchant
        if caller != payment.payer && caller != payment.merchant_address {
            return Err(ContractError::Unauthorized);
        }

        // Check if refund already exists
        if storage::get_refund(&e, &refund_id).is_some() {
            return Err(ContractError::RefundAlreadyExists);
        }

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        // Check remaining refundable amount
        let remaining = payment.amount - payment.refunded_amount;
        if amount > remaining {
            return Err(ContractError::RefundAmountExceedsLimit);
        }

        // Check refund window (30 days)
        let refund_window = 30 * 24 * 60 * 60; // 30 days in seconds
        if e.ledger().timestamp() > payment.paid_at + refund_window {
            return Err(ContractError::RefundExpired);
        }

        // Create refund request
        let refund = RefundRequest {
            refund_id: refund_id.clone(),
            order_id,
            amount,
            reason,
            status: RefundStatus::Pending,
            initiated_at: e.ledger().timestamp(),
        };

        storage::set_refund(&e, &refund_id, &refund);

        e.events().publish(
            (Symbol::new(&e, "refund_initiated"), refund_id),
            (payment.payer, payment.merchant_address, amount),
        );

        Ok(())
    }

    pub fn approve_refund(e: Env, caller: Address, refund_id: String) -> Result<(), ContractError> {
        caller.require_auth();

        let mut refund = storage::get_refund(&e, &refund_id).ok_or(ContractError::RefundDoesNotExist)?;
        if refund.status != RefundStatus::Pending {
            return Err(ContractError::InvalidRefundStatus);
        }

        let payment = storage::get_payment(&e, &refund.order_id).ok_or(ContractError::PaymentDoesNotExist)?;

        // Merchant or Admin can approve
        let admin = storage::get_admin(&e);
        let is_admin = admin.is_some() && admin.unwrap() == caller;
        if caller != payment.merchant_address && !is_admin {
            return Err(ContractError::Unauthorized);
        }

        refund.status = RefundStatus::Approved;
        storage::set_refund(&e, &refund_id, &refund);

        e.events().publish(
            (Symbol::new(&e, "refund_approved"), refund_id),
            (),
        );

        Ok(())
    }

    pub fn reject_refund(e: Env, caller: Address, refund_id: String) -> Result<(), ContractError> {
        caller.require_auth();

        let mut refund = storage::get_refund(&e, &refund_id).ok_or(ContractError::RefundDoesNotExist)?;
        if refund.status != RefundStatus::Pending {
            return Err(ContractError::InvalidRefundStatus);
        }

        let payment = storage::get_payment(&e, &refund.order_id).ok_or(ContractError::PaymentDoesNotExist)?;

        // Merchant or Admin can reject
        let admin = storage::get_admin(&e);
        let is_admin = admin.is_some() && admin.unwrap() == caller;
        if caller != payment.merchant_address && !is_admin {
            return Err(ContractError::Unauthorized);
        }

        refund.status = RefundStatus::Rejected;
        storage::set_refund(&e, &refund_id, &refund);

        e.events().publish(
            (Symbol::new(&e, "refund_rejected"), refund_id),
            (),
        );

        Ok(())
    }

    pub fn execute_refund(e: Env, refund_id: String) -> Result<(), ContractError> {
        let mut refund = storage::get_refund(&e, &refund_id).ok_or(ContractError::RefundDoesNotExist)?;
        if refund.status != RefundStatus::Approved {
            return Err(ContractError::InvalidRefundStatus);
        }

        let mut payment = storage::get_payment(&e, &refund.order_id).ok_or(ContractError::PaymentDoesNotExist)?;

        // Double check remaining amount
        let remaining = payment.amount - payment.refunded_amount;
        if refund.amount > remaining {
            return Err(ContractError::RefundAmountExceedsLimit);
        }

        // Check merchant token balance
        let token_client = token::Client::new(&e, &payment.token);
        let merchant_balance = token_client.balance(&payment.merchant_address);
        if merchant_balance < refund.amount {
            return Err(ContractError::InsufficientMerchantBalance);
        }

        // Require merchant's auth for executing transfer
        payment.merchant_address.require_auth();
        
        token_client.transfer(&payment.merchant_address, &payment.payer, &refund.amount);

        // Update payment record
        payment.refunded_amount += refund.amount;
        if payment.refunded_amount == payment.amount {
            payment.status = PaymentStatus::FullyRefunded;
        } else {
            payment.status = PaymentStatus::PartiallyRefunded;
        }
        storage::set_payment(&e, &refund.order_id, &payment);

        // Update refund status
        refund.status = RefundStatus::Completed;
        storage::set_refund(&e, &refund_id, &refund);

        // Update global stats
        let mut stats = storage::get_global_stats(&e);
        stats.total_refunds_count += 1;
        stats.total_refunded_volume += refund.amount;
        storage::set_global_stats(&e, &stats);

        e.events().publish(
            (Symbol::new(&e, "refund_executed"), refund_id),
            (payment.merchant_address, payment.payer, refund.amount),
        );

        Ok(())
    }

    pub fn get_merchant_payment_history(
        e: Env,
        merchant: Address,
        cursor: Option<String>,
        limit: u32,
        filter: Option<PaymentFilter>,
        sort_field: SortField,
        sort_order: SortOrder,
    ) -> Vec<PaymentRecord> {
        merchant.require_auth();
        let order_ids = storage::get_merchant_payments(&e, &merchant);
        query_payment_history(&e, order_ids, cursor, limit, filter, sort_field, sort_order)
    }

    pub fn get_payer_payment_history(
        e: Env,
        payer: Address,
        cursor: Option<String>,
        limit: u32,
        filter: Option<PaymentFilter>,
        sort_field: SortField,
        sort_order: SortOrder,
    ) -> Vec<PaymentRecord> {
        payer.require_auth();
        let order_ids = storage::get_payer_payments(&e, &payer);
        query_payment_history(&e, order_ids, cursor, limit, filter, sort_field, sort_order)
    }

    pub fn get_payment_by_id(
        e: Env,
        caller: Address,
        order_id: String,
    ) -> Result<PaymentRecord, ContractError> {
        caller.require_auth();
        let record = storage::get_payment(&e, &order_id).ok_or(ContractError::PaymentDoesNotExist)?;

        let admin = storage::get_admin(&e);
        let is_admin = admin.is_some() && admin.unwrap() == caller;

        if caller != record.payer && caller != record.merchant_address && !is_admin {
            return Err(ContractError::Unauthorized);
        }

        Ok(record)
    }

    pub fn get_global_payment_stats(
        e: Env,
        admin: Address,
        date_start: Option<u64>,
        date_end: Option<u64>,
    ) -> Result<GlobalStats, ContractError> {
        let stored_admin = storage::get_admin(&e).ok_or(ContractError::NotInitialized)?;
        if admin != stored_admin {
            return Err(ContractError::Unauthorized);
        }
        admin.require_auth();

        if date_start.is_none() && date_end.is_none() {
            return Ok(storage::get_global_stats(&e));
        }

        // Compute filtered stats
        let all_ids = storage::get_all_payments(&e);
        let mut filtered_volume = 0;
        let mut filtered_count = 0;
        let mut filtered_refunds_count = 0;
        let mut filtered_refunded_volume = 0;

        let mut active_merchants = Vec::new(&e);

        for id in all_ids.iter() {
            if let Some(record) = storage::get_payment(&e, &id) {
                let mut match_date = true;
                if let Some(start) = date_start {
                    if record.paid_at < start {
                        match_date = false;
                    }
                }
                if let Some(end) = date_end {
                    if record.paid_at > end {
                        match_date = false;
                    }
                }
                if match_date {
                    filtered_count += 1;
                    filtered_volume += record.amount;
                    if record.refunded_amount > 0 {
                        filtered_refunds_count += 1;
                        filtered_refunded_volume += record.refunded_amount;
                    }
                    if !active_merchants.contains(&record.merchant_address) {
                        active_merchants.push_back(record.merchant_address.clone());
                    }
                }
            }
        }

        Ok(GlobalStats {
            total_payments_count: filtered_count,
            total_volume_processed: filtered_volume,
            total_refunds_count: filtered_refunds_count,
            total_refunded_volume: filtered_refunded_volume,
            active_merchants_count: active_merchants.len() as u64,
        })
    }

    pub fn update_payment_status(
        e: Env,
        caller: Address,
        order_id: String,
        refunded_amount: i128,
    ) -> Result<(), ContractError> {
        caller.require_auth();

        let mut record = storage::get_payment(&e, &order_id).ok_or(ContractError::PaymentDoesNotExist)?;

        if caller != record.merchant_address {
            return Err(ContractError::Unauthorized);
        }

        if refunded_amount < 0 || refunded_amount > record.amount {
            return Err(ContractError::RefundAmountExceedsLimit);
        }

        record.refunded_amount = refunded_amount;
        if refunded_amount == record.amount {
            record.status = PaymentStatus::FullyRefunded;
        } else if refunded_amount > 0 {
            record.status = PaymentStatus::PartiallyRefunded;
        } else {
            record.status = PaymentStatus::Completed;
        }

        storage::set_payment(&e, &order_id, &record);
        Ok(())
    }

    pub fn archive_payment_record(e: Env, admin: Address, order_id: String) -> Result<(), ContractError> {
        let stored_admin = storage::get_admin(&e).ok_or(ContractError::NotInitialized)?;
        if admin != stored_admin {
            return Err(ContractError::Unauthorized);
        }
        admin.require_auth();

        if storage::get_payment(&e, &order_id).is_none() {
            return Err(ContractError::PaymentDoesNotExist);
        }

        storage::remove_payment(&e, &order_id);

        e.events().publish(
            (Symbol::new(&e, "payment_archived"), order_id),
            (),
        );

        Ok(())
    }

    pub fn cleanup_expired_payments(e: Env, admin: Address) -> Result<(), ContractError> {
        let stored_admin = storage::get_admin(&e).ok_or(ContractError::NotInitialized)?;
        if admin != stored_admin {
            return Err(ContractError::Unauthorized);
        }
        admin.require_auth();

        let all_ids = storage::get_all_payments(&e);
        let mut active_ids = Vec::new(&e);
        let current_time = e.ledger().timestamp();

        for id in all_ids.iter() {
            if let Some(record) = storage::get_payment(&e, &id) {
                if current_time > record.expiry {
                    storage::remove_payment(&e, &id);
                } else {
                    active_ids.push_back(id);
                }
            }
        }

        storage::set_all_payments(&e, &active_ids);
        Ok(())
    }

    pub fn set_payment_cleanup_period(
        e: Env,
        admin: Address,
        period: u64,
    ) -> Result<(), ContractError> {
        let stored_admin = storage::get_admin(&e).ok_or(ContractError::NotInitialized)?;
        if admin != stored_admin {
            return Err(ContractError::Unauthorized);
        }
        admin.require_auth();

        let min_period = 30 * 24 * 60 * 60;
        if period < min_period {
            return Err(ContractError::InvalidCleanupPeriod);
        }

        storage::set_cleanup_period(&e, period);
        Ok(())
    }

    pub fn initiate_multisig_payment(
        e: Env,
        payer: Address,
        order: Order,
        signers: Vec<Address>,
        threshold: u32,
    ) -> Result<String, ContractError> {
        payer.require_auth();

        if threshold == 0 || threshold > signers.len() {
            return Err(ContractError::InvalidThreshold);
        }

        if order.amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        if storage::get_multisig(&e, &order.order_id).is_some() {
            return Err(ContractError::PaymentAlreadyExists);
        }

        let multisig = MultisigPayment {
            multisig_id: order.order_id.clone(),
            payer: payer.clone(),
            order: order.clone(),
            signers,
            approvals: Vec::new(&e),
            threshold,
            status: MultisigStatus::Pending,
            initiated_at: e.ledger().timestamp(),
        };

        storage::set_multisig(&e, &order.order_id, &multisig);

        e.events().publish(
            (Symbol::new(&e, "multisig_initiated"), order.order_id.clone()),
            (payer, order.merchant_address, order.amount),
        );

        Ok(order.order_id)
    }

    pub fn approve_multisig_payment(
        e: Env,
        caller: Address,
        multisig_id: String,
    ) -> Result<(), ContractError> {
        caller.require_auth();

        let mut multisig = storage::get_multisig(&e, &multisig_id).ok_or(ContractError::MultisigPaymentDoesNotExist)?;

        if multisig.status != MultisigStatus::Pending {
            return Err(ContractError::MultisigNotPending);
        }

        let mut is_signer = false;
        for signer in multisig.signers.iter() {
            if signer == caller {
                is_signer = true;
                break;
            }
        }
        if !is_signer {
            return Err(ContractError::Unauthorized);
        }

        for approval in multisig.approvals.iter() {
            if approval == caller {
                return Err(ContractError::MultisigAlreadyApproved);
            }
        }

        multisig.approvals.push_back(caller.clone());

        e.events().publish(
            (Symbol::new(&e, "multisig_approved"), multisig_id.clone()),
            (caller,),
        );

        if multisig.approvals.len() >= multisig.threshold {
            let token_client = token::Client::new(&e, &multisig.order.token);
            let allowance = token_client.allowance(&multisig.payer, &e.current_contract_address());

            if allowance >= multisig.order.amount {
                token_client.transfer_from(
                    &e.current_contract_address(),
                    &multisig.payer,
                    &multisig.order.merchant_address,
                    &multisig.order.amount,
                );
            } else {
                multisig.payer.require_auth();
                token_client.transfer(
                    &multisig.payer,
                    &multisig.order.merchant_address,
                    &multisig.order.amount,
                );
            }

            let paid_at = e.ledger().timestamp();
            let cleanup_period = storage::get_cleanup_period(&e);
            let record = PaymentRecord {
                order_id: multisig.order.order_id.clone(),
                payer: multisig.payer.clone(),
                merchant_address: multisig.order.merchant_address.clone(),
                amount: multisig.order.amount,
                token: multisig.order.token.clone(),
                status: PaymentStatus::Completed,
                refunded_amount: 0,
                paid_at,
                expiry: paid_at + cleanup_period,
            };

            storage::set_payment(&e, &multisig.order.order_id, &record);

            let mut all_payments = storage::get_all_payments(&e);
            all_payments.push_back(multisig.order.order_id.clone());
            storage::set_all_payments(&e, &all_payments);

            let mut merchant_payments = storage::get_merchant_payments(&e, &multisig.order.merchant_address);
            merchant_payments.push_back(multisig.order.order_id.clone());
            storage::set_merchant_payments(&e, &multisig.order.merchant_address, &merchant_payments);

            let mut payer_payments = storage::get_payer_payments(&e, &multisig.payer);
            payer_payments.push_back(multisig.order.order_id.clone());
            storage::set_payer_payments(&e, &multisig.payer, &payer_payments);

            let mut stats = storage::get_global_stats(&e);
            stats.total_payments_count += 1;
            stats.total_volume_processed += multisig.order.amount;
            storage::set_global_stats(&e, &stats);

            multisig.status = MultisigStatus::Executed;

            e.events().publish(
                (Symbol::new(&e, "multisig_executed"), multisig_id.clone()),
                (multisig.payer.clone(), multisig.order.merchant_address.clone(), multisig.order.amount),
            );
        }

        storage::set_multisig(&e, &multisig_id, &multisig);
        Ok(())
    }
}
