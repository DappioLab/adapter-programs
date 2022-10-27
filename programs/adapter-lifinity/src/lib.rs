use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

pub mod price;
use crate::price::{get_lifinity_lp_price, get_lifinity_withdraw_amount, PoolDirectionP};

declare_id!("ADPTF4WmNPebELw6UvnSVBdL7BAqs5ceg9tyrHsQfrJK");

#[program]
pub mod adapter_lifinity {
    use super::*;

    // Currently didn't use it since we use only Jupiter for swap
    pub fn swap(ctx: Context<Action>, input: Vec<u8>) -> Result<()> {
        let discriminator: [u8; 8] = sighash("global", "swap");

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
        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = SwapInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let mut swap_data = vec![];
        swap_data.append(&mut discriminator.try_to_vec()?);
        swap_data.append(&mut input_struct.swap_in_amount.try_to_vec()?);
        swap_data.append(&mut input_struct.swap_min_out_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: swap_accounts,
            data: swap_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        dest_token_account.reload()?;

        let dest_token_amount_after = dest_token_account.amount;
        let out_amount = dest_token_amount_after - dest_token_amount_before;

        msg!("out_amount: {}", out_amount.to_string());

        // Wrap Output
        let output_struct = SwapOutputWrapper {
            swap_out_amount: out_amount,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn add_liquidity<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let discriminator: [u8; 8] = sighash("global", "deposit_all_token_types");

        // Use remaining accounts
        let lp_token_account_info = ctx.remaining_accounts[8].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = AddLiquidityInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let price_pool_direction = match input_struct.pool_direction {
            // Obverse
            0 => PoolDirectionP::Obverse,
            // Reverse
            1 => PoolDirectionP::Reverse,
            _ => return Err(ErrorCode::UnsupportedPoolDirection.into()),
        };

        // Get Price
        let lifinity_lp_price_wrapper = get_lifinity_lp_price(
            ctx.remaining_accounts[5].clone(),
            ctx.remaining_accounts[6].clone(),
            ctx.remaining_accounts[7].clone(),
            input_struct.token_in_amount,
            price_pool_direction,
        );

        let (add_lp_coin_amount, add_lp_pc_amount, minimal_lp_receive) = match input_struct
            .pool_direction
        {
            // Obverse
            0 => {
                // Obverse => pc
                (
                    (input_struct.token_in_amount as f64
                        * lifinity_lp_price_wrapper.coin_to_pc_price) as u64,
                    input_struct.token_in_amount,
                    lifinity_lp_price_wrapper.lp_amount as u64,
                )
            }
            // Reverse
            1 => (
                input_struct.token_in_amount,
                (input_struct.token_in_amount as f64 * lifinity_lp_price_wrapper.pc_to_coin_price)
                    as u64,
                lifinity_lp_price_wrapper.lp_amount as u64,
            ),
            _ => return Err(ErrorCode::UnsupportedPoolDirection.into()),
        };

        msg!("coin_amount: {}", add_lp_coin_amount.to_string());
        msg!("pc_amount: {}", add_lp_pc_amount.to_string());

        let add_lp_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
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

        invoke(&ix, ctx.remaining_accounts)?;

        lp_token_account.reload()?;
        let lp_token_amount_after = lp_token_account.amount;
        let lp_amount = lp_token_amount_after - lp_token_amount_before;

        msg!("lp_amount: {}", lp_amount.to_string());
        // Wrap Output
        let output_struct = AddLiquidityOutputWrapper {
            lp_amount,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn remove_liquidity<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let discriminator: [u8; 8] = sighash("global", "withdraw_all_token_types");

        // Use remaining accounts

        let coin_token_account_info = ctx.remaining_accounts[7].clone();
        let mut coin_token_account = Account::<TokenAccount>::try_from(&coin_token_account_info)?;
        let coin_token_amount_before = coin_token_account.amount;

        let pc_token_account_info = ctx.remaining_accounts[8].clone();
        let mut pc_token_account = Account::<TokenAccount>::try_from(&pc_token_account_info)?;
        let pc_token_amount_before = pc_token_account.amount;

        let lp_token_account_info = ctx.remaining_accounts[3].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = RemoveLiquidityInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let (coin_amount_out, pc_amount_out) = get_lifinity_withdraw_amount(
            ctx.remaining_accounts[4].clone(),
            ctx.remaining_accounts[5].clone(),
            ctx.remaining_accounts[6].clone(),
            input_struct.lp_amount,
        );

        let remove_lp_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
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
        remove_lp_data.append(&mut input_struct.lp_amount.try_to_vec()?);
        remove_lp_data.append(&mut coin_amount_out.try_to_vec()?);
        remove_lp_data.append(&mut pc_amount_out.try_to_vec()?);

        msg!(&*format!("remove_lp_data: {:?}", remove_lp_data.clone()));

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: remove_lp_accounts,
            data: remove_lp_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        coin_token_account.reload()?;
        let coin_token_amount_after = coin_token_account.amount;
        coin_token_amount_after - coin_token_amount_before;

        pc_token_account.reload()?;
        let pc_token_amount_after = pc_token_account.amount;
        pc_token_amount_after - pc_token_amount_before;

        lp_token_account.reload()?;
        let lp_out_amount = lp_token_account
            .amount
            .checked_sub(lp_token_amount_before)
            .unwrap();
        // Wrap Output
        let output_struct = RemoveLiquidityOutputWrapper {
            token_a_amount: coin_token_amount_after,
            token_b_amount: pc_token_amount_after,
            lp_amount: lp_out_amount,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

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

#[error_code]
pub enum ErrorCode {
    #[msg("Unsupported PoolDirection")]
    UnsupportedPoolDirection,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct SwapInputWrapper {
    pub swap_in_amount: u64,
    pub swap_min_out_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct SwapOutputWrapper {
    pub swap_out_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AddLiquidityInputWrapper {
    pub token_in_amount: u64,
    pub pool_direction: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct AddLiquidityOutputWrapper {
    pub lp_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct RemoveLiquidityInputWrapper {
    pub lp_amount: u64,
    pub pool_direction: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct RemoveLiquidityOutputWrapper {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub lp_amount: u64,
    pub dummy_4: u64,
}

pub type SwapOutputTuple = (u64, u64, u64, u64);
pub type AddLiquidityOutputTuple = (u64, u64, u64, u64);
pub type RemoveLiquidityOutputTuple = (u64, u64, u64, u64);

impl From<SwapOutputWrapper> for SwapOutputTuple {
    fn from(result: SwapOutputWrapper) -> SwapOutputTuple {
        let SwapOutputWrapper {
            swap_out_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (swap_out_amount, dummy_2, dummy_3, dummy_4)
    }
}
impl From<AddLiquidityOutputWrapper> for AddLiquidityOutputTuple {
    fn from(result: AddLiquidityOutputWrapper) -> AddLiquidityOutputTuple {
        let AddLiquidityOutputWrapper {
            lp_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (lp_amount, dummy_2, dummy_3, dummy_4)
    }
}
impl From<RemoveLiquidityOutputWrapper> for RemoveLiquidityOutputTuple {
    fn from(result: RemoveLiquidityOutputWrapper) -> RemoveLiquidityOutputTuple {
        let RemoveLiquidityOutputWrapper {
            token_a_amount,
            token_b_amount,
            lp_amount,
            dummy_4,
        } = result;
        (token_a_amount, token_b_amount, lp_amount, dummy_4)
    }
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];

    sighash.copy_from_slice(&hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}
