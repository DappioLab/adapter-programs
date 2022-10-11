use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

pub fn get_raydium_lp_price(
  pool_open_orders: AccountInfo,
  pool_id: AccountInfo,
  pool_coin_token_account_info: AccountInfo,
  pool_pc_token_account_info: AccountInfo,
  lp_mint_account_info: AccountInfo,
  token_in_amount: u64,
  pool_direction: PoolDirectionP
) -> GetRaydiumLpPriceWrapper {
    let data = &**pool_open_orders.try_borrow_data().unwrap();
    let serum_market_state = decode_open_orders(data);

    let pool_data = &**pool_id.try_borrow_data().unwrap();
    let pool_info = decode_pool_info(pool_data);

    let pool_coin_token_account = Account::<TokenAccount>::try_from(&pool_coin_token_account_info).unwrap();
    let pool_pc_token_account = Account::<TokenAccount>::try_from(&pool_pc_token_account_info).unwrap();
    let coin_balance = serum_market_state.base_token_total + pool_coin_token_account.amount - pool_info.need_take_pnl_coin;
    let pc_balance = serum_market_state.quote_token_total + pool_pc_token_account.amount - pool_info.need_take_pnl_pc;

    let coin_to_pc_price = coin_balance as f64 / pc_balance as f64;
    let pc_to_coin_price = pc_balance as f64 / coin_balance as f64 ;
    let (pool_balance, amount_in) = match pool_direction {
        PoolDirectionP::Obverse => (pc_balance, token_in_amount), // Obverse => pc
        PoolDirectionP::Reverse => (coin_balance, token_in_amount), // Reverse => coin
    };
    let lp_mint_account = Account::<Mint>::try_from(&lp_mint_account_info).unwrap();
    let lp_supply = lp_mint_account.supply;
    let lp_amount = (lp_supply * amount_in) as f64 / (amount_in + pool_balance) as f64;

    msg!("lp_amount: {}", lp_amount.to_string());

    return GetRaydiumLpPriceWrapper {
        coin_balance,
        pc_balance,
        coin_to_pc_price,
        pc_to_coin_price,
        lp_amount,
    }
}

#[derive(Accounts)]
pub struct GetRaydiumLpPrice<'info> {
    /// CHECK: Safe
    pub pool_id: AccountInfo<'info>,
    /// CHECK: Safe
    pub pool_open_orders: AccountInfo<'info>,
    pub pool_coin_token_account: Box<Account<'info, TokenAccount>>,
    pub pool_pc_token_account: Box<Account<'info, TokenAccount>>,
    pub lp_mint: Box<Account<'info, Mint>>,
}

#[derive(Accounts)]
pub struct GetRaydiumStake<'info> {
    /// CHECK: Safe
    pub stake_account_info: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SerumMarketStateLayoutV2 {
    pub empty_1: u32,
    pub empty_2: u8,
    pub account_flags: u64,
    pub own_address: Pubkey,
    pub vault_signer_nonce: u64,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub base_vault: Pubkey,
    pub base_deposits_total: u64,
    pub base_fees_accrued: u64,
    pub quote_vault: Pubkey,
    pub quote_deposits_total: u64,
    pub quote_fees_accrued: u64,
    pub quote_dust_threshold: u64,
    pub request_queue: Pubkey,
    pub event_queue: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub fee_rate_bps: u64,
    pub referrer_rebates_accrued: u64,
    pub empty_3: u32,
    pub empty_4: u16,
    pub empty_5: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SerumOpenOrdersLayoutV2 {
    pub empty_1: u32,
    pub empty_2: u8,
    pub account_flags: u64,
    pub market: Pubkey,
    pub owner: Pubkey,
    pub base_token_free: u64,
    pub base_token_total: u64,
    pub quote_token_free: u64,
    pub quote_token_total: u64,
    pub free_slot_bits: u128,
    pub is_bid_bits: u128,
    // pub orders: [u128; 128],
    // pub client_ids: [u64; 128],
    // pub referrer_rebates_accrued: u64,
    // pub empty_3: u32,
    // pub empty_4: u16,
    // pub empty_5: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GetRaydiumLpPriceWrapper {
    pub coin_balance: u64,
    pub pc_balance: u64,
    pub coin_to_pc_price: f64,
    pub pc_to_coin_price: f64,
    pub lp_amount: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RaydiumStakeInfoLayoutV4 {
    pub discriminator: u64,
    pub state: u64,
    pub nonce: u64,
    pub pool_lp_token_account: Pubkey,
    pub pool_reward_token_account: Pubkey,
    pub total_reward: u64,
    pub per_share: u128,
    pub per_block: u64,
    pub option: u8,
    pub pool_reward_token_account_b: Pubkey,
    pub empty_1: u32,
    pub empty_2: u16,
    pub empty_3: u8,
    pub total_reward_b: u64,
    pub per_share_b: u128,
    pub per_block_b: u64,
    pub last_block: u64,
    pub owner: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct PoolLayoutV4 {
    pub status: u64,
    pub nonce: u64,
    pub order_num: u64,
    pub depth: u64,
    pub coin_decimals: u64,
    pub pc_decimals: u64,
    pub state: u64,
    pub reset_flag: u64,
    pub min_size: u64,
    pub vol_max_cut_ratio: u64,
    pub amount_wave_ratio: u64,
    pub coin_lot_size: u64,
    pub pc_lot_size: u64,
    pub min_price_multiplier: u64,
    pub max_price_multiplier: u64,
    pub system_decimals_value: u64,
    // Fees
    pub min_separate_numerator: u64,
    pub min_separate_denominator: u64,
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub pnl_numerator: u64,
    pub pnl_denominator: u64,
    pub swap_fee_numerator: u64,
    pub swap_fee_denominator: u64,
    // OutputData
    pub need_take_pnl_coin: u64,
    pub need_take_pnl_pc: u64,
    pub total_pnl_pc: u64,
    pub total_pnl_coin: u64,
    pub pool_total_deposit_pc: u64,
    pub pool_total_deposit_coin: u64,
    pub swap_coin_in_amount: u64,
    pub swap_pc_out_amount: u64,
    pub swap_coin_to_pc_fee: u64,
    pub swap_pc_in_amount: u64,
    pub swap_coin_out_amount: u64,
    pub swap_pc_to_coin_fee: u64,
    pub pool_coin_token_account: Pubkey,
    pub pool_pc_token_account: Pubkey,
    pub coin_mint_address: Pubkey,
    pub pc_mint_address: Pubkey,
    pub lp_mint_address: Pubkey,
    pub amm_open_orders: Pubkey,
    pub serum_market: Pubkey,
    pub serum_program_id: Pubkey,
    pub amm_target_orders: Pubkey,
    pub pool_withdraw_queue: Pubkey,
    pub pool_temp_lp_token_account: Pubkey,
    pub amm_owner: Pubkey,
    pub pnl_owner: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum ZapDirectionP {
    In,
    Out,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum PoolDirectionP {
    Obverse,
    Reverse,
}

#[inline(never)]
pub fn decode_open_orders(mut open_orders: &[u8]) -> SerumOpenOrdersLayoutV2 {
    SerumOpenOrdersLayoutV2::deserialize(&mut open_orders).unwrap()
}

#[inline(never)]
pub fn decode_pool_info(mut pool_info: &[u8]) -> PoolLayoutV4 {
    PoolLayoutV4::deserialize(&mut pool_info).unwrap()
}
