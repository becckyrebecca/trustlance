use soroban_sdk::{contracttype, Address, Env, String, Vec};
use crate::types::{Merchant, PaymentRecord, RefundRequest, MultisigPayment, GlobalStats};

pub const LEDGERS_PER_DAY: u32 = 17280;
pub const BUMP_THRESHOLD: u32 = 30 * LEDGERS_PER_DAY;
pub const BUMP_LIMIT: u32 = 365 * LEDGERS_PER_DAY;

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    Admin,
    CleanupPeriod,
    Merchant(Address),
    MerchantList,
    Payment(String),
    MerchantPayments(Address),
    PayerPayments(Address),
    GlobalStats,
    Refund(String),
    Multisig(String),
    AllPayments,
}

pub fn has_admin(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Admin)
}

pub fn get_admin(e: &Env) -> Option<Address> {
    e.storage().instance().get(&DataKey::Admin)
}

pub fn set_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&DataKey::Admin, admin);
    e.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_LIMIT);
}

pub fn get_cleanup_period(e: &Env) -> u64 {
    e.storage().instance().get(&DataKey::CleanupPeriod).unwrap_or(90 * 24 * 60 * 60) // default 90 days in seconds
}

pub fn set_cleanup_period(e: &Env, period: u64) {
    e.storage().instance().set(&DataKey::CleanupPeriod, &period);
    e.storage().instance().extend_ttl(BUMP_THRESHOLD, BUMP_LIMIT);
}

pub fn has_merchant(e: &Env, address: &Address) -> bool {
    let key = DataKey::Merchant(address.clone());
    let has = e.storage().persistent().has(&key);
    if has {
        e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
    }
    has
}

#[allow(dead_code)]
pub fn get_merchant(e: &Env, address: &Address) -> Option<Merchant> {
    let key = DataKey::Merchant(address.clone());
    if e.storage().persistent().has(&key) {
        e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
        e.storage().persistent().get(&key)
    } else {
        None
    }
}

pub fn set_merchant(e: &Env, address: &Address, merchant: &Merchant) {
    let key = DataKey::Merchant(address.clone());
    e.storage().persistent().set(&key, merchant);
    e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
}

pub fn get_merchant_list(e: &Env) -> Vec<Address> {
    let key = DataKey::MerchantList;
    if e.storage().persistent().has(&key) {
        e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
        e.storage().persistent().get(&key).unwrap()
    } else {
        Vec::new(e)
    }
}

pub fn set_merchant_list(e: &Env, list: &Vec<Address>) {
    let key = DataKey::MerchantList;
    e.storage().persistent().set(&key, list);
    e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
}

pub fn get_payment(e: &Env, order_id: &String) -> Option<PaymentRecord> {
    let key = DataKey::Payment(order_id.clone());
    if e.storage().persistent().has(&key) {
        e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
        e.storage().persistent().get(&key)
    } else {
        None
    }
}

pub fn set_payment(e: &Env, order_id: &String, record: &PaymentRecord) {
    let key = DataKey::Payment(order_id.clone());
    e.storage().persistent().set(&key, record);
    e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
}

pub fn remove_payment(e: &Env, order_id: &String) {
    let key = DataKey::Payment(order_id.clone());
    if e.storage().persistent().has(&key) {
        e.storage().persistent().remove(&key);
    }
}

pub fn get_merchant_payments(e: &Env, merchant: &Address) -> Vec<String> {
    let key = DataKey::MerchantPayments(merchant.clone());
    if e.storage().persistent().has(&key) {
        e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
        e.storage().persistent().get(&key).unwrap()
    } else {
        Vec::new(e)
    }
}

pub fn set_merchant_payments(e: &Env, merchant: &Address, list: &Vec<String>) {
    let key = DataKey::MerchantPayments(merchant.clone());
    e.storage().persistent().set(&key, list);
    e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
}

pub fn get_payer_payments(e: &Env, payer: &Address) -> Vec<String> {
    let key = DataKey::PayerPayments(payer.clone());
    if e.storage().persistent().has(&key) {
        e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
        e.storage().persistent().get(&key).unwrap()
    } else {
        Vec::new(e)
    }
}

pub fn set_payer_payments(e: &Env, payer: &Address, list: &Vec<String>) {
    let key = DataKey::PayerPayments(payer.clone());
    e.storage().persistent().set(&key, list);
    e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
}

pub fn get_all_payments(e: &Env) -> Vec<String> {
    let key = DataKey::AllPayments;
    if e.storage().persistent().has(&key) {
        e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
        e.storage().persistent().get(&key).unwrap()
    } else {
        Vec::new(e)
    }
}

pub fn set_all_payments(e: &Env, list: &Vec<String>) {
    let key = DataKey::AllPayments;
    e.storage().persistent().set(&key, list);
    e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
}

pub fn get_global_stats(e: &Env) -> GlobalStats {
    let key = DataKey::GlobalStats;
    if e.storage().persistent().has(&key) {
        e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
        e.storage().persistent().get(&key).unwrap()
    } else {
        GlobalStats {
            total_payments_count: 0,
            total_volume_processed: 0,
            total_refunds_count: 0,
            total_refunded_volume: 0,
            active_merchants_count: 0,
        }
    }
}

pub fn set_global_stats(e: &Env, stats: &GlobalStats) {
    let key = DataKey::GlobalStats;
    e.storage().persistent().set(&key, stats);
    e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
}

pub fn get_refund(e: &Env, refund_id: &String) -> Option<RefundRequest> {
    let key = DataKey::Refund(refund_id.clone());
    if e.storage().persistent().has(&key) {
        e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
        e.storage().persistent().get(&key)
    } else {
        None
    }
}

pub fn set_refund(e: &Env, refund_id: &String, request: &RefundRequest) {
    let key = DataKey::Refund(refund_id.clone());
    e.storage().persistent().set(&key, request);
    e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
}

pub fn get_multisig(e: &Env, multisig_id: &String) -> Option<MultisigPayment> {
    let key = DataKey::Multisig(multisig_id.clone());
    if e.storage().persistent().has(&key) {
        e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
        e.storage().persistent().get(&key)
    } else {
        None
    }
}

pub fn set_multisig(e: &Env, multisig_id: &String, multisig: &MultisigPayment) {
    let key = DataKey::Multisig(multisig_id.clone());
    e.storage().persistent().set(&key, multisig);
    e.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_LIMIT);
}
