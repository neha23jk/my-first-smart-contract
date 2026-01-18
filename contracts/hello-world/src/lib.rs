#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, token, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Vault,
}

#[derive(Clone)]
#[contracttype]
pub struct Vault {
    pub saver: Address,
    pub token: Address,
    pub goal_amount: i128,
    pub unlock_time: u64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[contracterror]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    GoalNotReached = 3,
    TooEarly = 4,
    InvalidAmount = 5, // Added for extra safety
}

#[contract]
pub struct AchievementVault;

#[contractimpl]
impl AchievementVault {
    /// Initializes the vault with a target goal, an asset token, and an unlock timestamp.
    /// This can only be called once.
    pub fn initialize(
        env: Env,
        saver: Address,
        token: Address,
        goal_amount: i128,
        unlock_time: u64,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Vault) {
            return Err(Error::AlreadyInitialized);
        }

        let vault = Vault {
            saver,
            token,
            goal_amount,
            unlock_time,
        };

        env.storage().instance().set(&DataKey::Vault, &vault);
        Ok(())
    }

    /// Deposits tokens into the vault. Anyone can call this to help the saver.
    pub fn deposit(env: Env, from: Address, amount: i128) -> Result<(), Error> {
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let vault: Vault = env.storage().instance()
            .get(&DataKey::Vault)
            .ok_or(Error::NotInitialized)?;
            
        from.require_auth();

        let client = token::Client::new(&env, &vault.token);
        client.transfer(&from, &env.current_contract_address(), &amount);

        Ok(())
    }

    /// Allows the saver to withdraw funds ONLY if the time has passed AND the goal is reached.
    pub fn withdraw(env: Env) -> Result<(), Error> {
        // Fetch vault data or return NotInitialized error efficiently
        let vault: Vault = env.storage().instance()
            .get(&DataKey::Vault)
            .ok_or(Error::NotInitialized)?;
            
        // Security: Ensure only the designated saver can call this
        vault.saver.require_auth();

        // Business Logic Check 1: Time Lock
        if env.ledger().timestamp() < vault.unlock_time {
            return Err(Error::TooEarly);
        }

        let client = token::Client::new(&env, &vault.token);
        let balance = client.balance(&env.current_contract_address());
        
        // Business Logic Check 2: Goal Threshold
        if balance < vault.goal_amount {
            return Err(Error::GoalNotReached);
        }

        // Execution: Transfer all funds to the saver
        client.transfer(&env.current_contract_address(), &vault.saver, &balance);
        
        Ok(())
    }
}

mod test;