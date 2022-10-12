pub mod common;
pub mod decimal;
pub mod error;
pub mod rate;
pub mod uint;

use std::convert::TryInto;

pub fn calculate_underlying_to_withdraw(
    amount: u64,
    total_vlp_shares: u64,
    total_vault_balance: u64,
) -> u64 {
    ((amount as u128)
        .checked_mul(total_vault_balance as u128)
        .unwrap())
    .checked_div(total_vlp_shares as u128)
    .unwrap()
    .try_into()
    .unwrap()
}
pub fn calculate_shares_to_give(
    amount: u64,
    total_vlp_shares: u64,
    total_vault_balance: u64,
) -> u64 {
    ((amount as u128)
        .checked_mul(total_vlp_shares as u128)
        .unwrap())
    .checked_div(total_vault_balance as u128)
    .unwrap()
    .try_into()
    .unwrap()
}
