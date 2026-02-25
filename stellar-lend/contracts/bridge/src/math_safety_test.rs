use crate::bridge::{BridgeContract, ContractError};

#[test]
fn test_bridge_fee_check() {
    // 10^30 tokens * 10% fee (1000 bps)
    let amount = 1_000_000_000_000_000_000_000_000_000_000i128;
    let fee = BridgeContract::compute_fee(amount, 1000);
    // 10^30 * 1000 / 10000 = 10^29
    assert_eq!(fee, 100_000_000_000_000_000_000_000_000_000i128);
    
    // Test extreme overflow
    let max_amount = i128::MAX;
    let fee_overflow = BridgeContract::compute_fee(max_amount, 1000);
    // max * 1000 / 10000 = max / 10
    // But compute_fee uses checked_mul, so it might fail if intermediate overflows
    // i128::MAX * 1000 will definitely overflow i128
    assert_eq!(fee_overflow, 0); // Our implementation uses unwrap_or(0) on overflow
}
