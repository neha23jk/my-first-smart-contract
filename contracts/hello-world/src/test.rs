#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger, LedgerInfo}, Address, Env};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient as TokenAdminClient;

/// Helper function to setup the environment, accounts, and token
fn setup_test(env: &Env) -> (Address, Address, Address, TokenClient<'_>, TokenAdminClient<'_>, Address) {
    let admin = Address::generate(env);
    let saver = Address::generate(env);
    let friend = Address::generate(env);
    
    let token_address = env.register_stellar_asset_contract(admin.clone());
    let token = TokenClient::new(env, &token_address);
    let token_admin = TokenAdminClient::new(env, &token_address);

    (admin, saver, friend, token, token_admin, token_address)
}

#[test]
fn test_vault_flow() {
    let env = Env::default();
    env.mock_all_auths();

    // 1. Initial Ledger Setup
    let mut ledger_info = LedgerInfo {
        timestamp: 100,
        protocol_version: 20,
        sequence_number: 1,
        network_id: [0u8; 32],
        base_reserve: 10,
        min_persistent_entry_ttl: 4096,
        min_temp_entry_ttl: 16,
        max_entry_ttl: 6312000,
    };
    env.ledger().set(ledger_info.clone());

    let (_admin, saver, friend, token, token_admin, token_address) = setup_test(&env);
    token_admin.mint(&friend, &100);

    // 2. Initialize Vault
    let contract_id = env.register_contract(None, AchievementVault);
    let client = AchievementVaultClient::new(&env, &contract_id);

    let target_amount: i128 = 100;
    let unlock_time: u64 = 1000;

    client.initialize(&saver, &token_address, &target_amount, &unlock_time);

    // 3. Deposit logic check
    client.deposit(&friend, &50);
    assert_eq!(token.balance(&contract_id), 50);

    // 4. Test: Withdrawal before Time Lock (Should Fail)
    ledger_info.timestamp = 500;
    env.ledger().set(ledger_info.clone());
    
    let result = client.try_withdraw();
    assert_eq!(result, Err(Ok(Error::TooEarly))); // Specific error check

    // 5. Test: Withdrawal before Goal Reached (Should Fail)
    ledger_info.timestamp = 1001;
    env.ledger().set(ledger_info.clone());
    
    let result = client.try_withdraw();
    assert_eq!(result, Err(Ok(Error::GoalNotReached))); // Specific error check

    // 6. Test: Success Case (Time passed + Goal met)
    client.deposit(&friend, &50);
    assert_eq!(token.balance(&contract_id), 100);

    client.withdraw();
    assert_eq!(token.balance(&saver), 100);
    assert_eq!(token.balance(&contract_id), 0);
}