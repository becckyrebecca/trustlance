use soroban_sdk::{contracttype, Address, String, Vec};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum MerchantCategory {
    Retail = 0,
    Services = 1,
    Digital = 2,
    Other = 3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Merchant {
    pub address: Address,
    pub name: String,
    pub description: String,
    pub contact_info: String,
    pub category: MerchantCategory,
    pub registered_at: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum PaymentStatus {
    Completed = 0,
    PartiallyRefunded = 1,
    FullyRefunded = 2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Order {
    pub merchant_address: Address,
    pub order_id: String,
    pub amount: i128,
    pub token: Address,
    pub nonce: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct PaymentRecord {
    pub order_id: String,
    pub payer: Address,
    pub merchant_address: Address,
    pub amount: i128,
    pub token: Address,
    pub status: PaymentStatus,
    pub refunded_amount: i128,
    pub paid_at: u64,
    pub expiry: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum RefundStatus {
    Pending = 0,
    Approved = 1,
    Rejected = 2,
    Completed = 3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct RefundRequest {
    pub refund_id: String,
    pub order_id: String,
    pub amount: i128,
    pub reason: String,
    pub status: RefundStatus,
    pub initiated_at: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum MultisigStatus {
    Pending = 0,
    Executed = 1,
    Cancelled = 2,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct MultisigPayment {
    pub multisig_id: String,
    pub payer: Address,
    pub order: Order,
    pub signers: Vec<Address>,
    pub approvals: Vec<Address>,
    pub threshold: u32,
    pub status: MultisigStatus,
    pub initiated_at: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum SortField {
    Date = 0,
    Amount = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum SortOrder {
    Ascending = 0,
    Descending = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum PaymentStatusFilter {
    Any = 0,
    Completed = 1,
    PartiallyRefunded = 2,
    FullyRefunded = 3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct PaymentFilter {
    pub amount_min: Option<i128>,
    pub amount_max: Option<i128>,
    pub token: Option<Address>,
    pub status: PaymentStatusFilter,
    pub date_start: Option<u64>,
    pub date_end: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct GlobalStats {
    pub total_payments_count: u64,
    pub total_volume_processed: i128,
    pub total_refunds_count: u64,
    pub total_refunded_volume: i128,
    pub active_merchants_count: u64,
}
