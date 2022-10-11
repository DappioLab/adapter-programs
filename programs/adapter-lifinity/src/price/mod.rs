use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

pub fn get_lifinity_lp_price(
  pool_coin_token_account_info: AccountInfo,
  pool_pc_token_account_info: AccountInfo,
  lp_mint_account_info: AccountInfo,
  token_in_amount: u64,
  pool_direction: PoolDirectionP
) -> GetLifinityLpPriceWrapper {
  let slippage:u64 = 1;
  let slippage_percent: f64 = (100f64 - slippage as f64 ) / 100f64;

  let pool_coin_token_account = Account::<TokenAccount>::try_from(&pool_coin_token_account_info).unwrap();
  let pool_pc_token_account = Account::<TokenAccount>::try_from(&pool_pc_token_account_info).unwrap();
  let coin_balance = pool_coin_token_account.amount;
  let pc_balance = pool_pc_token_account.amount;

  let coin_to_pc_price = coin_balance as f64 / pc_balance as f64;
  let pc_to_coin_price = pc_balance as f64 / coin_balance as f64;
  let (pool_balance, amount_in) = match pool_direction {
      PoolDirectionP::Obverse => {(pc_balance, token_in_amount)}, // Obverse => pc
      PoolDirectionP::Reverse => {(coin_balance, token_in_amount)}, // Reverse => coin
  };
  
  let lp_mint_account = Account::<Mint>::try_from(&lp_mint_account_info).unwrap();
  let lp_supply = lp_mint_account.supply;
  let lp_amount = (lp_supply * amount_in) as f64 / (pool_balance) as f64 * slippage_percent;

  msg!("lp_amount: {}", lp_amount.to_string());

  return GetLifinityLpPriceWrapper {
      coin_balance,
      pc_balance,
      coin_to_pc_price,
      pc_to_coin_price,
      lp_amount,
  }
}

pub fn get_lifinity_withdraw_amount(
  pool_coin_token_account_info: AccountInfo,
  pool_pc_token_account_info: AccountInfo,
  lp_mint_account_info: AccountInfo,
  remove_lp_amount: u64,
) -> (u64, u64) {
  let slippage:u64 = 1;
  let slippage_percent: f64 = (100f64 + slippage as f64 ) / 100f64;

  let lp_mint_account = Account::<Mint>::try_from(&lp_mint_account_info).unwrap();
  let lp_supply = lp_mint_account.supply;

  let pool_coin_token_account = Account::<TokenAccount>::try_from(&pool_coin_token_account_info).unwrap();
  let pool_pc_token_account = Account::<TokenAccount>::try_from(&pool_pc_token_account_info).unwrap();
  let coin_balance = pool_coin_token_account.amount;
  let pc_balance = pool_pc_token_account.amount;

  let coin_out_amount = (coin_balance * remove_lp_amount) as f64 / slippage_percent / (lp_supply) as f64;
  let pc_out_amount = (pc_balance * remove_lp_amount) as f64 / slippage_percent / (lp_supply) as f64;
  
  msg!("remove_lp_amount: {}", remove_lp_amount.to_string());
  msg!("coin_out_amount: {}", coin_out_amount.to_string());
  msg!("pc_out_amount: {}", pc_out_amount.to_string());

  return (coin_out_amount as u64, pc_out_amount as u64);
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GetLifinityLpPriceWrapper {
    pub coin_balance: u64,
    pub pc_balance: u64,
    pub coin_to_pc_price: f64,
    pub pc_to_coin_price: f64,
    pub lp_amount: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum PoolDirectionP {
    Obverse,
    Reverse,
}
