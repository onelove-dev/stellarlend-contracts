#![cfg(test)]

use super::*;
use crate::amm::*;
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, Symbol, Vec,
};

fn create_amm_contract<'a>(env: &Env) -> AmmContractClient<'a> {
    AmmContractClient::new(env, &env.register(AmmContract {}, ()))
}

fn create_test_protocol_config(env: &Env, protocol_addr: &Address) -> AmmProtocolConfig {
    let mut supported_pairs = Vec::new(env);
    supported_pairs.push_back(TokenPair {
        token_a: None, // Native XLM
        token_b: Some(Address::generate(env)), // Mock USDC
        pool_address: Address::generate(env),
    });

    AmmProtocolConfig {
        protocol_address: protocol_addr.clone(),
        protocol_name: Symbol::new(env, "TestAMM"),
        enabled: true,
        fee_tier: 30, // 0.3%
        min_swap_amount: 1000,
        max_swap_amount: 1_000_000_000,
        supported_pairs,
    }
}

#[test]
fn test_initialize_amm_settings() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);

    // Initialize AMM settings - this should not panic
    contract.initialize_amm_settings(
        &admin,
        &100,   // 1% default slippage
        &1000,  // 10% max slippage
        &10000, // 10000 auto-swap threshold
    );

    // Verify settings were stored
    let settings = contract.get_amm_settings();
    assert!(settings.is_some());
    let settings = settings.unwrap();
    assert_eq!(settings.default_slippage, 100);
    assert_eq!(settings.max_slippage, 1000);
    assert_eq!(settings.auto_swap_threshold, 10000);
    assert!(settings.swap_enabled);
    assert!(settings.liquidity_enabled);
}

#[test]
fn test_add_amm_protocol() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);
    let protocol_addr = Address::generate(&env);

    // Initialize first
    contract.initialize_amm_settings(&admin, &100, &1000, &10000);

    // Create protocol config
    let protocol_config = create_test_protocol_config(&env, &protocol_addr);

    // Add protocol - this should not panic
    contract.add_amm_protocol(&admin, &protocol_config);

    // Verify protocol was added
    let protocols = contract.get_amm_protocols();
    assert!(protocols.is_some());
    let protocols = protocols.unwrap();
    assert!(protocols.contains_key(protocol_addr.clone()));
}

#[test]
fn test_execute_swap_success() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let protocol_addr = Address::generate(&env);

    // Initialize
    contract.initialize_amm_settings(&admin, &100, &1000, &10000);

    // Add protocol
    let protocol_config = create_test_protocol_config(&env, &protocol_addr);
    contract.add_amm_protocol(&admin, &protocol_config);

    // Create swap parameters
    let swap_params = SwapParams {
        protocol: protocol_addr,
        token_in: None, // Native XLM
        token_out: Some(Address::generate(&env)), // Mock USDC
        amount_in: 100000,
        min_amount_out: 95000, // 5% slippage tolerance
        slippage_tolerance: 500, // 5%
        deadline: env.ledger().timestamp() + 300,
    };

    // Execute swap - this should return an amount
    let amount_out = contract.execute_swap(&user, &swap_params);
    assert!(amount_out >= swap_params.min_amount_out);
    assert!(amount_out <= swap_params.amount_in); // Should be less due to slippage
}

#[test]
#[should_panic(expected = "Swap error")]
fn test_execute_swap_invalid_params() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let protocol_addr = Address::generate(&env);

    // Initialize
    contract.initialize_amm_settings(&admin, &100, &1000, &10000);

    // Add protocol
    let protocol_config = create_test_protocol_config(&env, &protocol_addr);
    contract.add_amm_protocol(&admin, &protocol_config);

    // Test invalid amount (zero) - should panic
    let swap_params = SwapParams {
        protocol: protocol_addr,
        token_in: None,
        token_out: Some(Address::generate(&env)),
        amount_in: 0, // Invalid: zero amount
        min_amount_out: 95000,
        slippage_tolerance: 500,
        deadline: env.ledger().timestamp() + 300,
    };

    contract.execute_swap(&user, &swap_params);
}

#[test]
#[should_panic(expected = "Swap error")]
fn test_execute_swap_same_tokens() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let protocol_addr = Address::generate(&env);

    // Initialize
    contract.initialize_amm_settings(&admin, &100, &1000, &10000);

    // Add protocol
    let protocol_config = create_test_protocol_config(&env, &protocol_addr);
    contract.add_amm_protocol(&admin, &protocol_config);

    // Test same token in and out - should panic
    let swap_params = SwapParams {
        protocol: protocol_addr,
        token_in: None,
        token_out: None, // Same as token_in
        amount_in: 100000,
        min_amount_out: 95000,
        slippage_tolerance: 500,
        deadline: env.ledger().timestamp() + 300,
    };

    contract.execute_swap(&user, &swap_params);
}

#[test]
fn test_add_liquidity_success() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let protocol_addr = Address::generate(&env);

    // Initialize
    contract.initialize_amm_settings(&admin, &100, &1000, &10000);

    // Add protocol
    let protocol_config = create_test_protocol_config(&env, &protocol_addr);
    contract.add_amm_protocol(&admin, &protocol_config);

    // Create liquidity parameters
    let liquidity_params = LiquidityParams {
        protocol: protocol_addr,
        token_a: None, // Native XLM
        token_b: Some(Address::generate(&env)), // Mock USDC
        amount_a: 100000,
        amount_b: 100000,
        min_amount_a: 95000,
        min_amount_b: 95000,
        deadline: env.ledger().timestamp() + 300,
    };

    // Add liquidity
    let lp_tokens = contract.add_liquidity(&user, &liquidity_params);
    assert!(lp_tokens > 0);
}

#[test]
fn test_remove_liquidity_success() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let protocol_addr = Address::generate(&env);

    // Initialize
    contract.initialize_amm_settings(&admin, &100, &1000, &10000);

    // Add protocol
    let protocol_config = create_test_protocol_config(&env, &protocol_addr);
    contract.add_amm_protocol(&admin, &protocol_config);

    // Remove liquidity
    let (amount_a, amount_b) = contract.remove_liquidity(
        &user,
        &protocol_addr,
        &None, // Native XLM
        &Some(Address::generate(&env)), // Mock USDC
        &50000, // LP tokens to burn
        &45000, // Min amount A
        &45000, // Min amount B
        &(env.ledger().timestamp() + 300),
    );

    assert!(amount_a >= 45000);
    assert!(amount_b >= 45000);
}

#[test]
fn test_auto_swap_for_collateral() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let protocol_addr = Address::generate(&env);

    // Initialize
    contract.initialize_amm_settings(&admin, &100, &1000, &10000);

    // Add protocol
    let protocol_config = create_test_protocol_config(&env, &protocol_addr);
    contract.add_amm_protocol(&admin, &protocol_config);

    // Test auto-swap
    let amount_out = contract.auto_swap_for_collateral(
        &user,
        &Some(Address::generate(&env)), // Target token
        &50000, // Amount above threshold
    );

    assert!(amount_out > 0);
}

#[test]
#[should_panic(expected = "Auto-swap error")]
fn test_auto_swap_below_threshold() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let protocol_addr = Address::generate(&env);

    // Initialize with high threshold
    contract.initialize_amm_settings(&admin, &100, &1000, &100000);

    // Add protocol
    let protocol_config = create_test_protocol_config(&env, &protocol_addr);
    contract.add_amm_protocol(&admin, &protocol_config);

    // Test auto-swap with amount below threshold - should panic
    contract.auto_swap_for_collateral(
        &user,
        &Some(Address::generate(&env)),
        &50000, // Amount below threshold (100000)
    );
}

#[test]
fn test_update_amm_settings() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);

    // Initialize
    contract.initialize_amm_settings(&admin, &100, &1000, &10000);

    // Update settings
    let new_settings = AmmSettings {
        default_slippage: 200,
        max_slippage: 2000,
        swap_enabled: false,
        liquidity_enabled: true,
        auto_swap_threshold: 20000,
    };

    contract.update_amm_settings(&admin, &new_settings);

    // Verify settings were updated
    let settings = contract.get_amm_settings().unwrap();
    assert_eq!(settings.default_slippage, 200);
    assert_eq!(settings.max_slippage, 2000);
    assert!(!settings.swap_enabled);
    assert!(settings.liquidity_enabled);
    assert_eq!(settings.auto_swap_threshold, 20000);
}

#[test]
#[should_panic(expected = "Add AMM protocol error")]
fn test_unauthorized_add_protocol() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let protocol_addr = Address::generate(&env);

    // Initialize with admin
    contract.initialize_amm_settings(&admin, &100, &1000, &10000);

    // Try to add protocol with non-admin - should panic
    let protocol_config = create_test_protocol_config(&env, &protocol_addr);
    contract.add_amm_protocol(&non_admin, &protocol_config);
}

#[test]
fn test_get_swap_history() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let protocol_addr = Address::generate(&env);

    // Initialize
    contract.initialize_amm_settings(&admin, &100, &1000, &10000);

    // Add protocol
    let protocol_config = create_test_protocol_config(&env, &protocol_addr);
    contract.add_amm_protocol(&admin, &protocol_config);

    // Execute a swap to create history
    let swap_params = SwapParams {
        protocol: protocol_addr,
        token_in: None,
        token_out: Some(Address::generate(&env)),
        amount_in: 100000,
        min_amount_out: 95000,
        slippage_tolerance: 500,
        deadline: env.ledger().timestamp() + 300,
    };

    contract.execute_swap(&user, &swap_params);

    // Get swap history
    let history = contract.get_swap_history(&Some(user.clone()), &10);
    assert!(history.is_some());
    let history = history.unwrap();
    assert_eq!(history.len(), 1);

    // Get all swap history
    let all_history = contract.get_swap_history(&None, &10);
    assert!(all_history.is_some());
    let all_history = all_history.unwrap();
    assert_eq!(all_history.len(), 1);
}

#[test]
fn test_get_liquidity_history() {
    let env = Env::default();
    env.mock_all_auths();

    let contract = create_amm_contract(&env);
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let protocol_addr = Address::generate(&env);

    // Initialize
    contract.initialize_amm_settings(&admin, &100, &1000, &10000);

    // Add protocol
    let protocol_config = create_test_protocol_config(&env, &protocol_addr);
    contract.add_amm_protocol(&admin, &protocol_config);

    // Add liquidity to create history
    let liquidity_params = LiquidityParams {
        protocol: protocol_addr,
        token_a: None,
        token_b: Some(Address::generate(&env)),
        amount_a: 100000,
        amount_b: 100000,
        min_amount_a: 95000,
        min_amount_b: 95000,
        deadline: env.ledger().timestamp() + 300,
    };

    contract.add_liquidity(&user, &liquidity_params);

    // Get liquidity history
    let history = contract.get_liquidity_history(&Some(user.clone()), &10);
    assert!(history.is_some());
    let history = history.unwrap();
    assert_eq!(history.len(), 1);

    // Get all liquidity history
    let all_history = contract.get_liquidity_history(&None, &10);
    assert!(all_history.is_some());
    let all_history = all_history.unwrap();
    assert_eq!(all_history.len(), 1);
}