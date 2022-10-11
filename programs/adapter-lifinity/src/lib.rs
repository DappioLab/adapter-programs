use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
    hash::hash,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

pub mod price;
use crate::price::{PoolDirectionP, get_lifinity_lp_price, get_lifinity_withdraw_amount};

declare_id!("ADPTF4WmNPebELw6UvnSVBdL7BAqs5ceg9tyrHsQfrJK");

#[program]
pub mod adapter_lifinity {
    use super::*;

    // Currently didn't use it since we use only Jupiter for swap
    pub fn swap(
        ctx: Context<Action>,
    ) -> Result<()> {
        let discriminator: [u8;8] = sighash("global", "swap");
        
        // Use remaining accounts
        let dest_token_account_info = ctx.remaining_accounts[4].clone();
        let mut dest_token_account = Account::<TokenAccount>::try_from(&dest_token_account_info)?;
        let dest_token_amount_before = dest_token_account.amount;

        let swap_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), true),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new(ctx.remaining_accounts[12].key(), false),
        ];

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let swap_in_amount = gateway_state.payload_queue[current_index as usize];

        let mut swap_data = vec![];
        swap_data.append(&mut discriminator.try_to_vec()?);
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
        let discriminator: [u8;8] = sighash("global", "deposit_all_token_types");

        // Use remaining accounts
        let lp_token_account_info = ctx.remaining_accounts[8].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;
        
        // Get the data from payload queue
        let token_in_amount = gateway_state.payload_queue[current_index as usize];

        let price_pool_direction = match gateway_state.pool_direction {
            // Obverse
            0 => PoolDirectionP::Obverse,
            // Reverse
            1 => PoolDirectionP::Reverse,
            _ => return Err(ErrorCode::UnsupportedPoolDirection.into())
        };

        // Get Price
        let lifinity_lp_price_wrapper = get_lifinity_lp_price(
            ctx.remaining_accounts[5].clone(),
            ctx.remaining_accounts[6].clone(),
            ctx.remaining_accounts[7].clone(),
            token_in_amount,
            price_pool_direction
        );

        let (
            add_lp_coin_amount,
            add_lp_pc_amount,
            minimal_lp_receive,
        ) = match gateway_state.pool_direction {
            // Obverse
            0 => {
                // Obverse => pc
                (
                    (token_in_amount as f64 * lifinity_lp_price_wrapper.coin_to_pc_price) as u64,
                    token_in_amount,
                    lifinity_lp_price_wrapper.lp_amount as u64,
                )
            },
            // Reverse
            1 => {
                (
                    token_in_amount,
                    (token_in_amount as f64 * lifinity_lp_price_wrapper.pc_to_coin_price) as u64,
                    lifinity_lp_price_wrapper.lp_amount as u64,
                )
            },
            _ => return Err(ErrorCode::UnsupportedPoolDirection.into())
        };
     
        msg!("coin_amount: {}", add_lp_coin_amount.to_string());
        msg!("pc_amount: {}", add_lp_pc_amount.to_string());

        let add_lp_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), true),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
        ];

        let mut add_lp_data = vec![];
        add_lp_data.append(&mut discriminator.try_to_vec()?);
        add_lp_data.append(&mut minimal_lp_receive.try_to_vec()?);
        add_lp_data.append(&mut add_lp_coin_amount.try_to_vec()?);
        add_lp_data.append(&mut add_lp_pc_amount.try_to_vec()?);
        
        msg!(&*format!("add_lp_data: {:?}", add_lp_data.clone()));

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
        let discriminator: [u8;8] = sighash("global", "withdraw_all_token_types");


        // Use remaining accounts

        let coin_token_account_info = ctx.remaining_accounts[7].clone();
        let mut coin_token_account = Account::<TokenAccount>::try_from(&coin_token_account_info)?;
        let coin_token_amount_before = coin_token_account.amount;

        let pc_token_account_info = ctx.remaining_accounts[8].clone();
        let mut pc_token_account = Account::<TokenAccount>::try_from(&pc_token_account_info)?;
        let pc_token_amount_before = pc_token_account.amount;

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let lp_amount_in = gateway_state.payload_queue[current_index as usize];

        let (coin_amount_out, pc_amount_out) = get_lifinity_withdraw_amount(
            ctx.remaining_accounts[4].clone(),
            ctx.remaining_accounts[5].clone(),
            ctx.remaining_accounts[6].clone(),
            lp_amount_in,
        );

        let remove_lp_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), true),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
        ];

        let mut remove_lp_data = vec![];
        remove_lp_data.append(&mut discriminator.try_to_vec()?);
        remove_lp_data.append(&mut lp_amount_in.try_to_vec()?);
        remove_lp_data.append(&mut coin_amount_out.try_to_vec()?);
        remove_lp_data.append(&mut pc_amount_out.try_to_vec()?);

        msg!(&*format!("remove_lp_data: {:?}", remove_lp_data.clone()));

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
}

#[derive(Accounts)]
pub struct Action<'info> {
    pub gateway_authority: Signer<'info>,
    /// CHECK: Safe
    pub gateway_state_info: AccountInfo<'info>,
    /// CHECK: Safe
    pub base_program_id: AccountInfo<'info>,
}

fn get_gateway_state(gateway_state_info: &AccountInfo) -> GatewayStateWrapper {
    let mut gateway_state_data = &**gateway_state_info.try_borrow_data().unwrap();
    GatewayStateWrapper::deserialize(&mut gateway_state_data).unwrap()
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
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];

    sighash.copy_from_slice(&hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}
