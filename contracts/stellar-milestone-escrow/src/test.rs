#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String, Vec,
};

fn setup_test_env(
    e: &Env,
) -> (
    Address,
    Address,
    Address,
    Address,
    MilestoneEscrowContractClient<'static>,
) {
    e.mock_all_auths();

    let contract_id = e.register_contract(None, MilestoneEscrowContract);
    let client = MilestoneEscrowContractClient::new(e, &contract_id);

    let client_addr = Address::generate(e);
    let freelancer_addr = Address::generate(e);
    let arbiter_addr = Address::generate(e);

    let token_admin = Address::generate(e);
    let sac = e.register_stellar_asset_contract_v2(token_admin);
    let token_id = sac.address();

    // Mint some tokens to the client
    let token_admin_client = token::StellarAssetClient::new(e, &token_id);
    token_admin_client.mint(&client_addr, &10000);

    (client_addr, freelancer_addr, arbiter_addr, token_id, client)
}

#[test]
fn test_successful_flow() {
    let e = Env::default();
    let (client_addr, freelancer_addr, arbiter_addr, token_id, client) = setup_test_env(&e);

    let mut milestone_amounts = Vec::new(&e);
    milestone_amounts.push_back(100);
    milestone_amounts.push_back(200);

    client.initialize(
        &client_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &milestone_amounts,
        &3600,
    );

    // Check initial state
    let job = client.get_job();
    assert_eq!(job.client, client_addr);
    assert_eq!(job.freelancer, freelancer_addr);
    assert_eq!(job.arbiter, arbiter_addr);
    assert_eq!(job.token, token_id);
    assert_eq!(job.milestones.len(), 2);
    assert_eq!(job.milestones.get(0).unwrap().amount, 100);
    assert_eq!(
        job.milestones.get(0).unwrap().state,
        MilestoneState::Pending
    );
    assert_eq!(job.milestones.get(1).unwrap().amount, 200);
    assert_eq!(
        job.milestones.get(1).unwrap().state,
        MilestoneState::Pending
    );
    assert!(!job.is_funded);

    // Fund
    client.fund();
    let job = client.get_job();
    assert!(job.is_funded);

    // Check balances
    let token_client = token::Client::new(&e, &token_id);
    assert_eq!(token_client.balance(&client_addr), 9700);
    assert_eq!(token_client.balance(&client.address), 300);

    // Mark first milestone delivered
    client.mark_delivered(&0);
    let job = client.get_job();
    assert_eq!(
        job.milestones.get(0).unwrap().state,
        MilestoneState::Delivered
    );

    // Approve first milestone (early approval by client)
    client.approve_milestone(&0);
    let job = client.get_job();
    assert_eq!(
        job.milestones.get(0).unwrap().state,
        MilestoneState::Released
    );
    assert_eq!(token_client.balance(&freelancer_addr), 100);
    assert_eq!(token_client.balance(&client.address), 200);
}

#[test]
fn test_auto_release() {
    let e = Env::default();
    let (client_addr, freelancer_addr, arbiter_addr, token_id, client) = setup_test_env(&e);

    let mut milestone_amounts = Vec::new(&e);
    milestone_amounts.push_back(100);

    client.initialize(
        &client_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &milestone_amounts,
        &3600,
    );
    client.fund();
    client.mark_delivered(&0);

    // Update ledger timestamp to simulate auto-release window expiry
    e.ledger().set_timestamp(4000);

    // Check that we can approve after auto-release window
    client.approve_milestone(&0);
    let job = client.get_job();
    assert_eq!(
        job.milestones.get(0).unwrap().state,
        MilestoneState::Released
    );

    let token_client = token::Client::new(&e, &token_id);
    assert_eq!(token_client.balance(&freelancer_addr), 100);
}

#[test]
fn test_dispute_and_arbitration_favor_freelancer() {
    let e = Env::default();
    let (client_addr, freelancer_addr, arbiter_addr, token_id, client) = setup_test_env(&e);

    let mut milestone_amounts = Vec::new(&e);
    milestone_amounts.push_back(150);

    client.initialize(
        &client_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &milestone_amounts,
        &3600,
    );
    client.fund();

    // Freelancer raises dispute
    client.raise_dispute(&freelancer_addr, &0);
    let job = client.get_job();
    assert_eq!(
        job.milestones.get(0).unwrap().state,
        MilestoneState::Disputed
    );

    // Arbiter resolves dispute favoring freelancer
    client.resolve_dispute(&0, &true);
    let job = client.get_job();
    assert_eq!(
        job.milestones.get(0).unwrap().state,
        MilestoneState::Released
    );

    let token_client = token::Client::new(&e, &token_id);
    assert_eq!(token_client.balance(&freelancer_addr), 150);
    assert_eq!(token_client.balance(&client_addr), 9850);
}

#[test]
fn test_dispute_and_arbitration_favor_client() {
    let e = Env::default();
    let (client_addr, freelancer_addr, arbiter_addr, token_id, client) = setup_test_env(&e);

    let mut milestone_amounts = Vec::new(&e);
    milestone_amounts.push_back(150);

    client.initialize(
        &client_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &milestone_amounts,
        &3600,
    );
    client.fund();

    // Client raises dispute
    client.raise_dispute(&client_addr, &0);
    let job = client.get_job();
    assert_eq!(
        job.milestones.get(0).unwrap().state,
        MilestoneState::Disputed
    );

    // Arbiter resolves dispute favoring client (refund)
    client.resolve_dispute(&0, &false);
    let job = client.get_job();
    assert_eq!(
        job.milestones.get(0).unwrap().state,
        MilestoneState::Refunded
    );

    let token_client = token::Client::new(&e, &token_id);
    assert_eq!(token_client.balance(&freelancer_addr), 0);
    assert_eq!(token_client.balance(&client_addr), 10000);
}

#[test]
fn test_initialization_failures() {
    let e = Env::default();
    let (client_addr, freelancer_addr, arbiter_addr, token_id, client) = setup_test_env(&e);

    let zero_str = String::from_str(
        &e,
        "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF",
    );
    let zero_addr = Address::from_string(&zero_str);

    // Zero address client
    let mut milestone_amounts = Vec::new(&e);
    milestone_amounts.push_back(100);
    let res = client.try_initialize(
        &zero_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &milestone_amounts,
        &3600,
    );
    assert_eq!(res.unwrap_err().unwrap(), ContractError::ZeroAddress);

    // Empty milestone list
    let empty_milestones = Vec::new(&e);
    let res = client.try_initialize(
        &client_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &empty_milestones,
        &3600,
    );
    assert_eq!(res.unwrap_err().unwrap(), ContractError::EmptyMilestones);

    // Negative/Zero milestone amount
    let mut bad_milestone_amounts = Vec::new(&e);
    bad_milestone_amounts.push_back(0);
    let res = client.try_initialize(
        &client_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &bad_milestone_amounts,
        &3600,
    );
    assert_eq!(
        res.unwrap_err().unwrap(),
        ContractError::InvalidMilestoneAmount
    );

    // Zero auto-release window
    let res = client.try_initialize(
        &client_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &milestone_amounts,
        &0,
    );
    assert_eq!(
        res.unwrap_err().unwrap(),
        ContractError::ZeroAutoReleaseWindow
    );

    // Initialize successfully once
    client.initialize(
        &client_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &milestone_amounts,
        &3600,
    );

    // Try double initialization
    let res = client.try_initialize(
        &client_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &milestone_amounts,
        &3600,
    );
    assert_eq!(res.unwrap_err().unwrap(), ContractError::AlreadyInitialized);
}

#[test]
fn test_unauthorized_dispute() {
    let e = Env::default();
    let (client_addr, freelancer_addr, arbiter_addr, token_id, client) = setup_test_env(&e);

    let mut milestone_amounts = Vec::new(&e);
    milestone_amounts.push_back(100);

    client.initialize(
        &client_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &milestone_amounts,
        &3600,
    );
    client.fund();

    // Random user tries to raise dispute
    let random_user = Address::generate(&e);
    let res = client.try_raise_dispute(&random_user, &0);
    assert_eq!(res.unwrap_err().unwrap(), ContractError::Unauthorized);
}

#[test]
fn test_action_on_unfunded_job() {
    let e = Env::default();
    let (client_addr, freelancer_addr, arbiter_addr, token_id, client) = setup_test_env(&e);

    let mut milestone_amounts = Vec::new(&e);
    milestone_amounts.push_back(100);

    client.initialize(
        &client_addr,
        &freelancer_addr,
        &arbiter_addr,
        &token_id,
        &milestone_amounts,
        &3600,
    );

    let res = client.try_mark_delivered(&0);
    assert_eq!(res.unwrap_err().unwrap(), ContractError::NotFunded);
}
