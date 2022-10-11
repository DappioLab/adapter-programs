use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

pub mod price;
use crate::price::{PoolDirectionP, get_raydium_lp_price};

declare_id!("ADPT1q4xG8F9m64cQyjqGe11cCXQq6vL4beY5hJavhQ5");

#[program]
pub mod adapter_raydium {
    use super::*;

    pub fn swap(
        ctx: Context<Action>,
    ) -> Result<()> {
        let swap_ix: u8 = 9;

        // Use remaining accounts

        let dest_token_account_info = ctx.remaining_accounts[16].clone();
        let mut dest_token_account = Account::<TokenAccount>::try_from(&dest_token_account_info)?;
        let dest_token_amount_before = dest_token_account.amount;

        let swap_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new(ctx.remaining_accounts[12].key(), false),
            AccountMeta::new(ctx.remaining_accounts[13].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[14].key(), false),
            AccountMeta::new(ctx.remaining_accounts[15].key(), false),
            AccountMeta::new(ctx.remaining_accounts[16].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[17].key(), true),
        ];

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let swap_in_amount = gateway_state.payload_queue[current_index as usize];

        let mut swap_data = vec![];
        swap_data.append(&mut swap_ix.try_to_vec()?);
        swap_data.append(&mut swap_in_amount.try_to_vec()?);
        swap_data.append(&mut gateway_state.swap_min_out_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: swap_accounts,
            data: swap_data,
        };

        invoke(
            &ix,
            ctx.remaining_accounts,
        )?;

        dest_token_account.reload()?;

        let dest_token_amount_after = dest_token_account.amount;
        let out_amount = dest_token_amount_after - dest_token_amount_before;

        msg!("out_amount: {}", out_amount.to_string());

        // Return Result
        let swap_result = SwapResultWrapper {
            out_amount
        };
        let mut buffer: Vec<u8> = Vec::new();
        swap_result.serialize(&mut buffer).unwrap();

        anchor_lang::solana_program::program::set_return_data(&buffer);

        Ok(())
    }

    pub fn add_liquidity<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let add_lp_ix: u8 = 3;

        // Use remaining accounts

        let lp_token_account_info = ctx.remaining_accounts[11].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        let price_pool_direction = match gateway_state.pool_direction {
            // Obverse
            0 => PoolDirectionP::Obverse,
            // Reverse
            1 => PoolDirectionP::Reverse,
            _ => return Err(ErrorCode::UnsupportedPoolDirection.into())
        };

        // Get the data from payload queue
        let token_in_amount = gateway_state.payload_queue[current_index as usize];

        // Get Price
        let raydium_lp_price_wrapper = get_raydium_lp_price(
            ctx.remaining_accounts[3].clone(),
            ctx.remaining_accounts[1].clone(),
            ctx.remaining_accounts[6].clone(),
            ctx.remaining_accounts[7].clone(),
            ctx.remaining_accounts[5].clone(),
            token_in_amount,
            price_pool_direction
        );

        let (
            add_lp_coin_amount,
            add_lp_pc_amount,
            add_lp_fixed_coin,
        ) = match gateway_state.pool_direction {
            // Obverse
            0 => {
                (
                    (token_in_amount as f64 * raydium_lp_price_wrapper.coin_to_pc_price) as u64,
                    token_in_amount,
                    0 as u64,
                )
            },
            // Reverse
            1 => {
                (
                    token_in_amount,
                    (token_in_amount as f64 * raydium_lp_price_wrapper.pc_to_coin_price) as u64,
                    1 as u64,
                )
            },
            _ => return Err(ErrorCode::UnsupportedPoolDirection.into())
        };

        let add_lp_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false), // user_coin_token_account
            AccountMeta::new(ctx.remaining_accounts[10].key(), false), // user_pc_token_account
            AccountMeta::new(ctx.remaining_accounts[11].key(), false), // user_lp_token_account
            AccountMeta::new_readonly(ctx.remaining_accounts[12].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[13].key(), false),
        ];

        let mut add_lp_data = vec![];
        add_lp_data.append(&mut add_lp_ix.try_to_vec()?);
        add_lp_data.append(&mut add_lp_coin_amount.try_to_vec()?);
        add_lp_data.append(&mut add_lp_pc_amount.try_to_vec()?);
        add_lp_data.append(&mut add_lp_fixed_coin.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: add_lp_accounts,
            data: add_lp_data,
        };

        invoke(
            &ix,
            ctx.remaining_accounts,
        )?;

        lp_token_account.reload()?;
        let lp_token_amount_after = lp_token_account.amount;
        let lp_amount = lp_token_amount_after - lp_token_amount_before;

        msg!("lp_amount: {}", lp_amount.to_string());

        // Return Result
        let result = AddLiquidityResultWrapper {
            lp_amount
        };
        let mut buffer: Vec<u8> = Vec::new();
        result.serialize(&mut buffer).unwrap();

        anchor_lang::solana_program::program::set_return_data(&buffer);

        Ok(())
    }

    pub fn remove_liquidity<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let remove_lp_ix: u8 = 4;

        // Use remaining accounts

        let coin_token_account_info = ctx.remaining_accounts[16].clone();
        let mut coin_token_account = Account::<TokenAccount>::try_from(&coin_token_account_info)?;
        let coin_token_amount_before = coin_token_account.amount;

        let pc_token_account_info = ctx.remaining_accounts[17].clone();
        let mut pc_token_account = Account::<TokenAccount>::try_from(&pc_token_account_info)?;
        let pc_token_amount_before = pc_token_account.amount;

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let amount_in = gateway_state.payload_queue[current_index as usize];

        let remove_lp_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new(ctx.remaining_accounts[12].key(), false),
            AccountMeta::new(ctx.remaining_accounts[13].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[14].key(), false),
            AccountMeta::new(ctx.remaining_accounts[15].key(), false),
            AccountMeta::new(ctx.remaining_accounts[16].key(), false),
            AccountMeta::new(ctx.remaining_accounts[17].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[18].key(), true),
            AccountMeta::new(ctx.remaining_accounts[19].key(), false),
            AccountMeta::new(ctx.remaining_accounts[20].key(), false),
            AccountMeta::new(ctx.remaining_accounts[21].key(), false),
        ];

        let mut remove_lp_data = vec![];
        remove_lp_data.append(&mut remove_lp_ix.try_to_vec()?);
        remove_lp_data.append(&mut amount_in.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: remove_lp_accounts,
            data: remove_lp_data,
        };

        invoke(
            &ix,
            ctx.remaining_accounts,
        )?;

        let swap_in_amount = match gateway_state.pool_direction {
            // Obverse
            0 => {
                coin_token_account.reload()?;
                let coin_token_amount_after = coin_token_account.amount;
                coin_token_amount_after - coin_token_amount_before
            },
            // Reverse
            1 => {
                pc_token_account.reload()?;
                let pc_token_amount_after = pc_token_account.amount;
                pc_token_amount_after - pc_token_amount_before
            },
            _ => return Err(ErrorCode::UnsupportedPoolDirection.into())
        };

        // Return Result
        let result = RemoveLiquidityResultWrapper {
            swap_in_amount,
        };
        let mut buffer: Vec<u8> = Vec::new();
        result.serialize(&mut buffer).unwrap();

        anchor_lang::solana_program::program::set_return_data(&buffer);

        Ok(())
    }

    pub fn stake<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);

        let current_index = gateway_state.current_index;
        let version = gateway_state.version_queue[current_index as usize];

        // Get the data from payload queue
        let lp_amount = gateway_state.payload_queue[current_index as usize];

        let stake_ix = match version {
            3 => 10,
            5 => 11,
            _ => {
                return Err(ErrorCode::UnsupportedVersion.into());
            }
        };

        invoke_stake(&ctx, stake_ix, lp_amount)?;

        // Return Result
        let result = StakeResultWrapper {
            lp_amount,
        };
        let mut buffer: Vec<u8> = Vec::new();
        result.serialize(&mut buffer).unwrap();

        anchor_lang::solana_program::program::set_return_data(&buffer);

        Ok(())
    }

    pub fn unstake<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);

        let current_index = gateway_state.current_index;
        let version = gateway_state.version_queue[current_index as usize];

        // Get the data from payload queue
        let share_amount = gateway_state.payload_queue[current_index as usize];

        let unstake_ix = match version {
            3 => 11,
            5 => 12,
            _ => {
                return Err(ErrorCode::UnsupportedVersion.into());
            }
        };

        invoke_stake(&ctx, unstake_ix, share_amount)?;

        // Return Result
        let result = UnstakeResultWrapper {
            share_amount,
        };
        let mut buffer: Vec<u8> = Vec::new();
        result.serialize(&mut buffer).unwrap();

        anchor_lang::solana_program::program::set_return_data(&buffer);

        Ok(())
    }

    pub fn harvest(
        ctx: Context<Action>,
    ) -> Result<()> {
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);

        let current_index = gateway_state.current_index;
        let version = gateway_state.version_queue[current_index as usize];

        let unstake_ix = match version {
            3 => 11,
            5 => 12,
            _ => {
                return Err(ErrorCode::UnsupportedVersion.into());
            }
        };

        invoke_stake(&ctx, unstake_ix, 0)
    }
}

fn get_gateway_state(gateway_state_info: &AccountInfo) -> GatewayStateWrapper {
    let mut gateway_state_data = &**gateway_state_info.try_borrow_data().unwrap();
    GatewayStateWrapper::deserialize(&mut gateway_state_data).unwrap()
}

pub fn invoke_stake(
    ctx: &Context<Action>,
    ix: u8,
    amount: u64
) -> Result<()> {
    let mut stake_accounts = vec![
        AccountMeta::new(ctx.remaining_accounts[0].key(), false),
        AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
        AccountMeta::new(ctx.remaining_accounts[2].key(), false),
        AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), true),
        AccountMeta::new(ctx.remaining_accounts[4].key(), false),
        AccountMeta::new(ctx.remaining_accounts[5].key(), false),
        AccountMeta::new(ctx.remaining_accounts[6].key(), false),
        AccountMeta::new(ctx.remaining_accounts[7].key(), false),
        AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
        AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
    ];

    // Check if the remaining_accounts are left (nornally 10 accounts)
    if ctx.remaining_accounts.len() > 10 {
        stake_accounts.push(AccountMeta::new(ctx.remaining_accounts[10].key(), false));
        stake_accounts.push(AccountMeta::new(ctx.remaining_accounts[11].key(), false));
    }

    let mut stake_data = vec![];
    stake_data.append(&mut ix.try_to_vec()?);
    stake_data.append(&mut amount.try_to_vec()?);

    let stake_ix = Instruction {
        program_id: ctx.accounts.base_program_id.key(),
        accounts: stake_accounts,
        data: stake_data,
    };

    invoke(
        &stake_ix,
        ctx.remaining_accounts,
    )?;

    Ok(())
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];

    sighash.copy_from_slice(&hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}

#[derive(Accounts)]
pub struct Action<'info> {
    pub gateway_authority: Signer<'info>,
    /// CHECK: Safe
    pub gateway_state_info: AccountInfo<'info>,
    /// CHECK: Safe
    pub base_program_id: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SwapResultWrapper {
    pub out_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AddLiquidityResultWrapper {
    pub lp_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RemoveLiquidityResultWrapper {
    pub swap_in_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct StakeResultWrapper {
    pub lp_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UnstakeResultWrapper {
    pub share_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum PoolDirection {
    Obverse,
    Reverse,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GatewayStateWrapper {
    pub discriminator: u64,
    pub user_key: Pubkey,
    pub random_seed: u64,
    pub version: u8,
    pub current_index: u8, // Start from 0
    pub queue_size: u8,

    // Queues
    pub protocol_queue: [u8; 8],
    pub action_queue: [u8; 8],
    pub version_queue: [u8; 8],
    pub payload_queue: [u64; 8],

    // Extra metadata
    pub swap_min_out_amount: u64,
    pub pool_direction: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unsupported PoolDirection")]
    UnsupportedPoolDirection,
    #[msg("Unsupported Action Version")]
    UnsupportedVersion
}
