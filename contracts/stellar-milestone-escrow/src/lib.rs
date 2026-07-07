#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token, Address, Env, String, Vec,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum MilestoneState {
    Pending = 0,
    Delivered = 1,
    Released = 2,
    Disputed = 3,
    Refunded = 4,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Milestone {
    pub amount: i128,
    pub state: MilestoneState,
    pub delivered_at: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Job {
    pub client: Address,
    pub freelancer: Address,
    pub arbiter: Address,
    pub token: Address,
    pub milestones: Vec<Milestone>,
    pub auto_release_window: u64,
    pub is_funded: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    Job = 0,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    EmptyMilestones = 3,
    InvalidMilestoneAmount = 4,
    ZeroAutoReleaseWindow = 5,
    ZeroAddress = 6,
    NotFunded = 7,
    AlreadyFunded = 8,
    InvalidMilestoneIndex = 9,
    InvalidMilestoneState = 10,
    Unauthorized = 11,
    AutoReleaseWindowNotMet = 12,
}

#[contract]
pub struct MilestoneEscrowContract;

#[contractimpl]
impl MilestoneEscrowContract {
    pub fn initialize(
        e: Env,
        client: Address,
        freelancer: Address,
        arbiter: Address,
        token: Address,
        milestone_amounts: Vec<i128>,
        auto_release_window: u64,
    ) -> Result<(), ContractError> {
        if e.storage().instance().has(&DataKey::Job) {
            return Err(ContractError::AlreadyInitialized);
        }

        // Validate zero addresses
        let zero_str = String::from_str(
            &e,
            "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF",
        );
        let zero_address = Address::from_string(&zero_str);
        if client == zero_address
            || freelancer == zero_address
            || arbiter == zero_address
            || token == zero_address
        {
            return Err(ContractError::ZeroAddress);
        }

        // Validate milestone amounts list
        if milestone_amounts.is_empty() {
            return Err(ContractError::EmptyMilestones);
        }

        let mut milestones = Vec::new(&e);
        for amount in milestone_amounts.iter() {
            if amount <= 0 {
                return Err(ContractError::InvalidMilestoneAmount);
            }
            milestones.push_back(Milestone {
                amount,
                state: MilestoneState::Pending,
                delivered_at: 0,
            });
        }

        // Validate auto release window
        if auto_release_window == 0 {
            return Err(ContractError::ZeroAutoReleaseWindow);
        }

        let job = Job {
            client,
            freelancer,
            arbiter,
            token,
            milestones,
            auto_release_window,
            is_funded: false,
        };

        e.storage().instance().set(&DataKey::Job, &job);
        Ok(())
    }

    pub fn fund(e: Env) -> Result<(), ContractError> {
        let mut job: Job = e
            .storage()
            .instance()
            .get(&DataKey::Job)
            .ok_or(ContractError::NotInitialized)?;

        if job.is_funded {
            return Err(ContractError::AlreadyFunded);
        }

        job.client.require_auth();

        let mut total_amount: i128 = 0;
        for milestone in job.milestones.iter() {
            total_amount += milestone.amount;
        }

        let token_client = token::Client::new(&e, &job.token);
        token_client.transfer(&job.client, &e.current_contract_address(), &total_amount);

        job.is_funded = true;
        e.storage().instance().set(&DataKey::Job, &job);
        Ok(())
    }

    pub fn mark_delivered(e: Env, milestone_index: u32) -> Result<(), ContractError> {
        let mut job: Job = e
            .storage()
            .instance()
            .get(&DataKey::Job)
            .ok_or(ContractError::NotInitialized)?;

        if !job.is_funded {
            return Err(ContractError::NotFunded);
        }

        job.freelancer.require_auth();

        if milestone_index >= job.milestones.len() {
            return Err(ContractError::InvalidMilestoneIndex);
        }

        let mut milestone = job.milestones.get(milestone_index).unwrap();
        if milestone.state != MilestoneState::Pending {
            return Err(ContractError::InvalidMilestoneState);
        }

        milestone.state = MilestoneState::Delivered;
        milestone.delivered_at = e.ledger().timestamp();

        job.milestones.set(milestone_index, milestone);
        e.storage().instance().set(&DataKey::Job, &job);
        Ok(())
    }

    pub fn approve_milestone(e: Env, milestone_index: u32) -> Result<(), ContractError> {
        let mut job: Job = e
            .storage()
            .instance()
            .get(&DataKey::Job)
            .ok_or(ContractError::NotInitialized)?;

        if !job.is_funded {
            return Err(ContractError::NotFunded);
        }

        if milestone_index >= job.milestones.len() {
            return Err(ContractError::InvalidMilestoneIndex);
        }

        let mut milestone = job.milestones.get(milestone_index).unwrap();
        if milestone.state != MilestoneState::Delivered {
            return Err(ContractError::InvalidMilestoneState);
        }

        let current_time = e.ledger().timestamp();
        let release_time = milestone.delivered_at + job.auto_release_window;

        if current_time >= release_time {
            // Auto-release window has passed, no client signature required.
        } else {
            // Client must sign to approve early
            job.client.require_auth();
        }

        milestone.state = MilestoneState::Released;

        let token_client = token::Client::new(&e, &job.token);
        token_client.transfer(
            &e.current_contract_address(),
            &job.freelancer,
            &milestone.amount,
        );

        job.milestones.set(milestone_index, milestone);
        e.storage().instance().set(&DataKey::Job, &job);
        Ok(())
    }

    pub fn raise_dispute(
        e: Env,
        caller: Address,
        milestone_index: u32,
    ) -> Result<(), ContractError> {
        let mut job: Job = e
            .storage()
            .instance()
            .get(&DataKey::Job)
            .ok_or(ContractError::NotInitialized)?;

        if !job.is_funded {
            return Err(ContractError::NotFunded);
        }

        caller.require_auth();

        if caller != job.client && caller != job.freelancer {
            return Err(ContractError::Unauthorized);
        }

        if milestone_index >= job.milestones.len() {
            return Err(ContractError::InvalidMilestoneIndex);
        }

        let mut milestone = job.milestones.get(milestone_index).unwrap();
        if milestone.state != MilestoneState::Pending
            && milestone.state != MilestoneState::Delivered
        {
            return Err(ContractError::InvalidMilestoneState);
        }

        milestone.state = MilestoneState::Disputed;

        job.milestones.set(milestone_index, milestone);
        e.storage().instance().set(&DataKey::Job, &job);
        Ok(())
    }

    pub fn resolve_dispute(
        e: Env,
        milestone_index: u32,
        favor_freelancer: bool,
    ) -> Result<(), ContractError> {
        let mut job: Job = e
            .storage()
            .instance()
            .get(&DataKey::Job)
            .ok_or(ContractError::NotInitialized)?;

        if !job.is_funded {
            return Err(ContractError::NotFunded);
        }

        job.arbiter.require_auth();

        if milestone_index >= job.milestones.len() {
            return Err(ContractError::InvalidMilestoneIndex);
        }

        let mut milestone = job.milestones.get(milestone_index).unwrap();
        if milestone.state != MilestoneState::Disputed {
            return Err(ContractError::InvalidMilestoneState);
        }

        let token_client = token::Client::new(&e, &job.token);
        if favor_freelancer {
            milestone.state = MilestoneState::Released;
            token_client.transfer(
                &e.current_contract_address(),
                &job.freelancer,
                &milestone.amount,
            );
        } else {
            milestone.state = MilestoneState::Refunded;
            token_client.transfer(
                &e.current_contract_address(),
                &job.client,
                &milestone.amount,
            );
        }

        job.milestones.set(milestone_index, milestone);
        e.storage().instance().set(&DataKey::Job, &job);
        Ok(())
    }

    pub fn get_job(e: Env) -> Result<Job, ContractError> {
        e.storage()
            .instance()
            .get(&DataKey::Job)
            .ok_or(ContractError::NotInitialized)
    }
}

mod test;
