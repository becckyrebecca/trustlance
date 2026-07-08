#![cfg(test)]

extern crate std;
use std::vec;

use crate::{NovaPayContract, NovaPayContractClient};
use crate::types::{
    MerchantCategory, Order, PaymentStatus,
};
use soroban_sdk::{
    token, Address, Bytes, BytesN, Env, String, Vec,
};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::testutils::Ledger as _;
use soroban_sdk::xdr::ToXdr;
use ed25519_dalek::{SigningKey, Signer};

fn setup_test_env<'a>() -> (Env, NovaPayContractClient<'a>, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, NovaPayContract);
    let client = NovaPayContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let merchant = Address::generate(&env);
    let payer = Address::generate(&env);

    // Deploy mock token
    let token_admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract_v2(token_admin.clone()).address();
    
    // Set admin
    client.set_admin(&admin);

    (env, client, admin, merchant, payer, token_address)
}

#[test]
fn test_register_merchant() {
    let (env, client, _admin, merchant, _payer, _token) = setup_test_env();

    client.register_merchant(
        &merchant,
        &String::from_str(&env, "Nova Pharmacy"),
        &String::from_str(&env, "Clinical Pharmacy Services"),
        &String::from_str(&env, "pharmacy@nova.org"),
        &MerchantCategory::Services,
    );

    // Try registering duplicate
    let result = client.try_register_merchant(
        &merchant,
        &String::from_str(&env, "Nova Pharmacy Duplicate"),
        &String::from_str(&env, "Desc"),
        &String::from_str(&env, "Contact"),
        &MerchantCategory::Services,
    );
    assert!(result.is_err());
}

#[test]
fn test_process_payment_with_signature() {
    let (env, client, _admin, merchant, payer, token_address) = setup_test_env();

    // Register merchant
    client.register_merchant(
        &merchant,
        &String::from_str(&env, "Nova Pharmacy"),
        &String::from_str(&env, "Clinical Pharmacy Services"),
        &String::from_str(&env, "pharmacy@nova.org"),
        &MerchantCategory::Services,
    );

    // Mint tokens to payer
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&payer, &1000);

    let token_client = token::Client::new(&env, &token_address);
    assert_eq!(token_client.balance(&payer), 1000);

    // Generate Order
    let order = Order {
        merchant_address: merchant.clone(),
        order_id: String::from_str(&env, "order_123"),
        amount: 250,
        token: token_address.clone(),
        nonce: 0,
    };

    // Sign order with merchant's private key (mocked locally using ed25519-dalek)
    let signing_key = SigningKey::from_bytes(&[2u8; 32]);
    let order_xdr = order.clone().to_xdr(&env);
    let mut order_bytes = vec![0u8; order_xdr.len() as usize];
    order_xdr.copy_into_slice(&mut order_bytes);
    let signature_bytes = signing_key.sign(&order_bytes).to_bytes();
    let signature = Bytes::from_slice(&env, &signature_bytes);
    let merchant_public_key = BytesN::from_array(&env, &signing_key.verifying_key().to_bytes());

    // Process payment
    client.process_payment_with_signature(
        &payer,
        &order,
        &signature,
        &merchant_public_key,
    );

    // Verify balances
    assert_eq!(token_client.balance(&payer), 750);
    assert_eq!(token_client.balance(&merchant), 250);

    // Verify payment record
    let record = client.get_payment_by_id(&payer, &String::from_str(&env, "order_123"));
    assert_eq!(record.order_id, String::from_str(&env, "order_123"));
    assert_eq!(record.payer, payer);
    assert_eq!(record.merchant_address, merchant);
    assert_eq!(record.amount, 250);
    assert_eq!(record.status, PaymentStatus::Completed);
}

#[test]
fn test_refund_workflow() {
    let (env, client, _admin, merchant, payer, token_address) = setup_test_env();

    // Register merchant
    client.register_merchant(
        &merchant,
        &String::from_str(&env, "Nova Pharmacy"),
        &String::from_str(&env, "Clinical Pharmacy Services"),
        &String::from_str(&env, "pharmacy@nova.org"),
        &MerchantCategory::Services,
    );

    // Mint and pay
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&payer, &1000);

    let order = Order {
        merchant_address: merchant.clone(),
        order_id: String::from_str(&env, "order_123"),
        amount: 500,
        token: token_address.clone(),
        nonce: 0,
    };

    let signing_key = SigningKey::from_bytes(&[2u8; 32]);
    let order_xdr = order.clone().to_xdr(&env);
    let mut order_bytes = vec![0u8; order_xdr.len() as usize];
    order_xdr.copy_into_slice(&mut order_bytes);
    let signature_bytes = signing_key.sign(&order_bytes).to_bytes();
    let signature = Bytes::from_slice(&env, &signature_bytes);
    let merchant_public_key = BytesN::from_array(&env, &signing_key.verifying_key().to_bytes());

    client.process_payment_with_signature(
        &payer,
        &order,
        &signature,
        &merchant_public_key,
    );

    // Payer initiates refund
    client.initiate_refund(
        &payer,
        &String::from_str(&env, "refund_1"),
        &String::from_str(&env, "order_123"),
        &200,
        &String::from_str(&env, "Duplicate consultation fee charged"),
    );

    // Approve refund by merchant
    client.approve_refund(&merchant, &String::from_str(&env, "refund_1"));

    // Execute refund
    client.execute_refund(&String::from_str(&env, "refund_1"));

    // Verify balances
    let token_client = token::Client::new(&env, &token_address);
    assert_eq!(token_client.balance(&payer), 700); // 500 remaining initially + 200 refund
    assert_eq!(token_client.balance(&merchant), 300); // 500 initial payment - 200 refund

    // Verify payment status
    let record = client.get_payment_by_id(&payer, &String::from_str(&env, "order_123"));
    assert_eq!(record.status, PaymentStatus::PartiallyRefunded);
    assert_eq!(record.refunded_amount, 200);
}

#[test]
fn test_multisig_payment() {
    let (env, client, _admin, merchant, payer, token_address) = setup_test_env();

    // Register merchant
    client.register_merchant(
        &merchant,
        &String::from_str(&env, "Nova Pharmacy"),
        &String::from_str(&env, "Clinical Pharmacy Services"),
        &String::from_str(&env, "pharmacy@nova.org"),
        &MerchantCategory::Services,
    );

    let signer1 = Address::generate(&env);
    let signer2 = Address::generate(&env);
    let mut signers = Vec::new(&env);
    signers.push_back(signer1.clone());
    signers.push_back(signer2.clone());

    let order = Order {
        merchant_address: merchant.clone(),
        order_id: String::from_str(&env, "order_multi"),
        amount: 400,
        token: token_address.clone(),
        nonce: 0,
    };

    // Mint tokens to payer
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&payer, &1000);

    // Payer initiates multisig
    client.initiate_multisig_payment(&payer, &order, &signers, &2);

    // Signer 1 approves
    client.approve_multisig_payment(&signer1, &String::from_str(&env, "order_multi"));

    // Signer 2 approves (this should trigger execution)
    client.approve_multisig_payment(&signer2, &String::from_str(&env, "order_multi"));

    // Check balance
    let token_client = token::Client::new(&env, &token_address);
    assert_eq!(token_client.balance(&payer), 600);
    assert_eq!(token_client.balance(&merchant), 400);

    // Check payment record
    let record = client.get_payment_by_id(&payer, &String::from_str(&env, "order_multi"));
    assert_eq!(record.status, PaymentStatus::Completed);
}

#[test]
fn test_expired_cleanup() {
    let (env, client, admin, merchant, payer, token_address) = setup_test_env();

    // Register merchant
    client.register_merchant(
        &merchant,
        &String::from_str(&env, "Nova Pharmacy"),
        &String::from_str(&env, "Clinical Pharmacy Services"),
        &String::from_str(&env, "pharmacy@nova.org"),
        &MerchantCategory::Services,
    );

    // Mint tokens to payer
    let token_admin_client = token::StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&payer, &1000);

    let order = Order {
        merchant_address: merchant.clone(),
        order_id: String::from_str(&env, "order_cleanup"),
        amount: 100,
        token: token_address.clone(),
        nonce: 0,
    };

    let signing_key = SigningKey::from_bytes(&[2u8; 32]);
    let order_xdr = order.clone().to_xdr(&env);
    let mut order_bytes = vec![0u8; order_xdr.len() as usize];
    order_xdr.copy_into_slice(&mut order_bytes);
    let signature_bytes = signing_key.sign(&order_bytes).to_bytes();
    let signature = Bytes::from_slice(&env, &signature_bytes);
    let merchant_public_key = BytesN::from_array(&env, &signing_key.verifying_key().to_bytes());

    // Process payment
    client.process_payment_with_signature(
        &payer,
        &order,
        &signature,
        &merchant_public_key,
    );

    // Clean period setting
    client.set_payment_cleanup_period(&admin, &(40 * 24 * 60 * 60)); // 40 days

    // Advance ledger time past cleanup period (default is 90 days, now set to 40 days)
    env.ledger().set_timestamp(100 * 24 * 60 * 60);

    // Clean up
    client.cleanup_expired_payments(&admin);

    // Payment should no longer exist
    let result = client.try_get_payment_by_id(&payer, &String::from_str(&env, "order_cleanup"));
    assert!(result.is_err());
}
