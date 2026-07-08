use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    MerchantAlreadyExists = 4,
    MerchantDoesNotExist = 5,
    PaymentAlreadyExists = 6,
    PaymentDoesNotExist = 7,
    RefundAlreadyExists = 8,
    RefundDoesNotExist = 9,
    RefundExpired = 10,
    RefundAmountExceedsLimit = 11,
    InvalidRefundStatus = 12,
    InsufficientMerchantBalance = 13,
    InvalidSignature = 14,
    InvalidAmount = 15,
    InvalidThreshold = 16,
    MultisigPaymentDoesNotExist = 17,
    MultisigAlreadyApproved = 18,
    MultisigNotPending = 19,
    MultisigThresholdNotMet = 20,
    ExpiredPaymentCleanupNotMet = 21,
    InvalidCleanupPeriod = 22,
}
