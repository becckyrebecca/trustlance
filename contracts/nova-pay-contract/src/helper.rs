use soroban_sdk::{xdr::ToXdr, Bytes, BytesN, Env};
use crate::types::Order;
use crate::error::ContractError;

pub fn verify_signature(
    e: &Env,
    order: &Order,
    signature: &Bytes,
    merchant_public_key: &BytesN<32>,
) -> Result<(), ContractError> {
    let signature_n: BytesN<64> = match BytesN::try_from(signature.clone()) {
        Ok(s) => s,
        Err(_) => return Err(ContractError::InvalidSignature),
    };

    let msg_buf = order.clone().to_xdr(e);

    // Verify signature using the crypto module
    // This will panic/abort the transaction if signature is invalid, which is standard on-chain behavior.
    e.crypto().ed25519_verify(merchant_public_key, &msg_buf, &signature_n);

    Ok(())
}
