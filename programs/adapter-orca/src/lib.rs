use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

declare_id!("ADPTTyNqameXftbqsxwXhbs7v7XP8E82YMaUStPgjmU5");

#[program]
pub mod adapter_orca {
    use super::*;

    pub fn add_liquidity<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = AddLiquidityInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        // Use remaining accounts
        let lp_token_account_info = ctx.remaining_accounts[7].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        // Get the data from input_struct
        let pool_token_in_amount = input_struct.token_in_amount;

        let add_lp_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), true),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
        ];
        // Build Ix Data
        const ADD_LP_IX: u8 = 4;
        let minimal_receive: u64 = 0;
        let mut add_lp_data = vec![];
        add_lp_data.append(&mut ADD_LP_IX.to_le_bytes().to_vec());
        add_lp_data.append(&mut pool_token_in_amount.to_le_bytes().to_vec());
        add_lp_data.append(&mut minimal_receive.to_le_bytes().to_vec());

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: add_lp_accounts,
            data: add_lp_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        lp_token_account.reload()?;
        let lp_token_amount_after = lp_token_account.amount;
        let lp_amount = lp_token_amount_after
            .checked_sub(lp_token_amount_before)
            .unwrap();

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
        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = RemoveLiquidityInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        // Get the data from input_struct
        let lp_amount = input_struct.lp_amount;

        // Use remaining accounts
        let lp_token_account_info = ctx.remaining_accounts[7].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        msg!("lp_amount: {}", lp_amount.to_string());
        let (ix, token_a_amount_before, token_b_amount_before) = match input_struct.action {
            // RemoveLiquidity
            2 => {
                const REMOVE_LP_IX: u8 = 3;
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
                let token_a_account_amount =
                    Account::<TokenAccount>::try_from(&ctx.remaining_accounts[7].clone())
                        .unwrap()
                        .amount;
                let token_b_account_amount =
                    Account::<TokenAccount>::try_from(&ctx.remaining_accounts[8].clone())
                        .unwrap()
                        .amount;

                let minimal_receive: u64 = 0;

                let mut remove_lp_data = vec![];
                remove_lp_data.append(&mut REMOVE_LP_IX.to_le_bytes().to_vec());
                remove_lp_data.append(&mut lp_amount.to_le_bytes().to_vec());
                remove_lp_data.append(&mut minimal_receive.to_le_bytes().to_vec());
                remove_lp_data.append(&mut minimal_receive.to_le_bytes().to_vec());

                (
                    Instruction {
                        program_id: ctx.accounts.base_program_id.key(),
                        accounts: remove_lp_accounts,
                        data: remove_lp_data,
                    },
                    token_a_account_amount,
                    token_b_account_amount,
                )
            }
            // RemoveLiquiditySingle
            3 => {
                const REMOVE_LP_IX: u8 = 5;
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
                    AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
                ];
                let minimal_receive: u64 = 1;

                let mut remove_lp_data = vec![];
                remove_lp_data.append(&mut REMOVE_LP_IX.to_le_bytes().to_vec());
                remove_lp_data.append(&mut minimal_receive.to_le_bytes().to_vec());
                remove_lp_data.append(&mut lp_amount.to_le_bytes().to_vec());
                let (token_a_amount_before, token_b_amount_before) = (
                    Account::<TokenAccount>::try_from(&ctx.remaining_accounts[7].clone())
                        .unwrap()
                        .amount,
                    0_u64,
                );

                (
                    Instruction {
                        program_id: ctx.accounts.base_program_id.key(),
                        accounts: remove_lp_accounts,
                        data: remove_lp_data,
                    },
                    token_a_amount_before,
                    token_b_amount_before,
                )
            }
            _ => {
                return Err(ErrorCode::UnsupportedAction.into());
            }
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Load lp amount change
        lp_token_account.reload();
        let lp_amount = lp_token_amount_before
            .checked_sub(lp_token_account.amount)
            .unwrap();

        // Load Token A amount change
        let a_out_amount = Account::<TokenAccount>::try_from(&ctx.remaining_accounts[7].clone())
            .unwrap()
            .amount
            .checked_sub(token_a_amount_before)
            .unwrap();

        // Load Token B amount change only for both side
        let b_out_amount = match input_struct.action {
            2 => Account::<TokenAccount>::try_from(&ctx.remaining_accounts[8].clone())
                .unwrap()
                .amount
                .checked_sub(token_b_amount_before)
                .unwrap(),
            _ => 0_u64,
        };
        // Wrap Output
        let output_struct = RemoveLiquidityOutputWrapper {
            token_a_out_amount: a_out_amount,
            token_b_out_amount: b_out_amount,
            lp_amount: lp_amount,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);
        Ok(())
    }

    pub fn stake<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = StakeInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        // Get the data from input_struct
        let lp_amount = input_struct.lp_amount;

        // Use remaining accounts
        let lp_token_account_info = ctx.remaining_accounts[1].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        let share_token_account_info = ctx.remaining_accounts[5].clone();
        let mut share_token_account = Account::<TokenAccount>::try_from(&share_token_account_info)?;
        let share_token_amount_before = share_token_account.amount;

        let stake_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), true),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
        ];

        const STAKE_IX: u8 = 2;
        let mut stake_data = vec![];
        stake_data.append(&mut STAKE_IX.to_le_bytes().to_vec());
        stake_data.append(&mut lp_amount.to_le_bytes().to_vec());

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: stake_accounts,
            data: stake_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Load lp amount change
        lp_token_account.reload();
        let lp_amount = lp_token_amount_before
            .checked_sub(lp_token_account.amount)
            .unwrap();

        // Load share amount change
        share_token_account.reload();
        let share_amount = share_token_account
            .amount
            .checked_sub(share_token_amount_before)
            .unwrap();

        // Wrap Output
        let output_struct = StakeOutputWrapper {
            share_amount: share_amount,
            lp_amount: lp_amount,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn unstake<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = UnstakeInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);
        // Get the data from input_struct

        let lp_amount = input_struct.share_amount;
        msg!("lp_amount: {}", lp_amount.to_string());

        // Use remaining accounts
        let lp_token_account_info = ctx.remaining_accounts[1].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        let share_token_account_info = ctx.remaining_accounts[4].clone();
        let mut share_token_account = Account::<TokenAccount>::try_from(&share_token_account_info)?;
        let share_token_amount_before = share_token_account.amount;

        let unstake_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), true),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
        ];

        const UNSTAKE_IX: u8 = 3;
        let mut unstake_data = vec![];
        unstake_data.append(&mut UNSTAKE_IX.to_le_bytes().to_vec());
        unstake_data.append(&mut lp_amount.to_le_bytes().to_vec());

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unstake_accounts,
            data: unstake_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        lp_token_account.reload();
        // Load lp amount change
        lp_token_account.reload();
        let lp_amount = lp_token_account
            .amount
            .checked_sub(lp_token_amount_before)
            .unwrap();

        // Load share amount change
        share_token_account.reload();
        let share_amount = share_token_amount_before
            .checked_sub(share_token_account.amount)
            .unwrap();

        // Wrap Output
        let output_struct = UnstakeOutputWrapper {
            share_amount: share_amount,
            lp_amount: lp_amount,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn harvest<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Use remaining accounts
        let reward_token_account_info = ctx.remaining_accounts[5].clone();
        let mut reward_token_account =
            Account::<TokenAccount>::try_from(&reward_token_account_info)?;
        let reward_token_amount_before = reward_token_account.amount;

        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = HarvestInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        // Harvest
        let harvest_ix: u8 = 4;
        let harvest_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
        ];

        let mut harvest_data = vec![];
        harvest_data.push(harvest_ix);
        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: harvest_accounts,
            data: harvest_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Load reward amount change
        reward_token_account.reload();
        let reward_amount = reward_token_account
            .amount
            .checked_sub(reward_token_amount_before)
            .unwrap();

        // Wrap Output
        let output_struct = HarvestOutputWrapper {
            reward_amount: reward_amount,
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
    // TODO: Add constraints
    pub gateway_authority: Signer<'info>,
    /// CHECK: Safe
    pub base_program_id: AccountInfo<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unsupported PoolDirection")]
    UnsupportedPoolDirection,
    #[msg("Unsupported Action")]
    UnsupportedAction,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AddLiquidityInputWrapper {
    pub token_in_amount: u64,
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
    pub action: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct RemoveLiquidityOutputWrapper {
    pub token_a_out_amount: u64,
    pub token_b_out_amount: u64,
    pub lp_amount: u64,
    pub dummy_4: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeInputWrapper {
    pub lp_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeOutputWrapper {
    pub share_amount: u64,
    pub lp_amount: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnstakeInputWrapper {
    pub share_amount: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnstakeOutputWrapper {
    pub lp_amount: u64,
    pub share_amount: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct HarvestInputWrapper {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct HarvestOutputWrapper {
    pub reward_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// Make a tuple for being accessed by index rather than field name
pub type AddLiquidityOutputTuple = (u64, u64, u64, u64);
pub type RemoveLiquidityOutputTuple = (u64, u64, u64, u64);
pub type StakeOutputTuple = (u64, u64, u64, u64);
pub type UnstakeOutputTuple = (u64, u64, u64, u64);
pub type HarvestOutputTuple = (u64, u64, u64, u64);

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
            token_a_out_amount,
            token_b_out_amount,
            lp_amount,
            dummy_4,
        } = result;
        (token_a_out_amount, token_b_out_amount, lp_amount, dummy_4)
    }
}

impl From<StakeOutputWrapper> for StakeOutputTuple {
    fn from(result: StakeOutputWrapper) -> StakeOutputTuple {
        let StakeOutputWrapper {
            share_amount,
            lp_amount,
            dummy_3,
            dummy_4,
        } = result;
        (share_amount, lp_amount, dummy_3, dummy_4)
    }
}

impl From<UnstakeOutputWrapper> for UnstakeOutputTuple {
    fn from(result: UnstakeOutputWrapper) -> UnstakeOutputTuple {
        let UnstakeOutputWrapper {
            lp_amount,
            share_amount,
            dummy_3,
            dummy_4,
        } = result;
        (lp_amount, share_amount, dummy_3, dummy_4)
    }
}

impl From<HarvestOutputWrapper> for HarvestOutputTuple {
    fn from(result: HarvestOutputWrapper) -> HarvestOutputTuple {
        let HarvestOutputWrapper {
            reward_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (reward_amount, dummy_2, dummy_3, dummy_4)
    }
}
