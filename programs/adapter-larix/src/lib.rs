use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;
declare_id!("ADPTLQQ1Bwgybb2qge7QKSW7woDrhEjcLWG642qP2X4");

#[program]
pub mod adapter_larix {
    use std::borrow::BorrowMut;

    use super::*;
    pub fn supply<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let deposit_reserve_id: u8 = 4;

        // Use remaining accounts
        let mut supply_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 0);

        let mut reserve_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 1);

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = SupplyInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let supply_amount = input_struct.token_in_amount;
        let supply_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
        ];
        let mut supply_ix_data: Vec<u8> = vec![];
        supply_ix_data.append(&mut deposit_reserve_id.try_to_vec()?);
        supply_ix_data.append(&mut supply_amount.try_to_vec()?);

        let supply_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: supply_accounts,
            data: supply_ix_data,
        };

        invoke(&supply_ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = SupplyOutputWrapper {
            reserve_out_amount: reserve_token_account_and_balance.get_balance_change(),
            supply_in_amount: supply_token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn unsupply<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let withdraw_reserve_id: u8 = 5;

        // Use remaining accounts
        let mut supply_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 8);

        let mut reserve_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 0);

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = UnsupplyInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let unsupply_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
        ];

        // [255, 255, 255, 255, 255, 255, 255, 255] is u64_max le byte array
        // Larix program will use all the balance in the token account
        let unsupply_data = vec![withdraw_reserve_id, 255, 255, 255, 255, 255, 255, 255, 255];

        let unsupply_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unsupply_accounts,
            data: unsupply_data,
        };

        invoke(&unsupply_ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = UnsupplyOutputWrapper {
            reserve_in_amount: reserve_token_account_and_balance.get_balance_change(),
            supply_out_amount: supply_token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn borrow<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let borrow_ix_id: u8 = 10;

        // Use remaining accounts
        let mut borrow_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 1);

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = BorrowInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let borrow_amount = input_struct.borrow_amount;

        let borrow_ix_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
        ];

        let mut borrow_ix_data = vec![borrow_ix_id];
        borrow_ix_data.append(borrow_amount.to_le_bytes().to_vec().borrow_mut());

        let borrow_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: borrow_ix_accounts,
            data: borrow_ix_data,
        };

        invoke(&borrow_ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = BorrowOutputWrapper {
            token_out_amount: borrow_token_account_and_balance.get_balance_change(),

            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn repay<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let repay_ix_id: u8 = 11;
        // Use remaining accounts
        let mut repay_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 0);

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = RepayInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let mut repay_amount = input_struct.repay_amount;

        // This magic number is the max value of Number in javascript
        // in this case it's a tag for reaping max amount
        const MAGIC_NUMBER: u64 = 9007199254740991;

        if repay_amount.eq(&MAGIC_NUMBER) {
            repay_amount = u64::MAX;
        }

        let repay_ix_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
        ];

        let mut repay_ix_data = vec![repay_ix_id];
        repay_ix_data.append(repay_amount.to_le_bytes().to_vec().borrow_mut());

        let repay_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: repay_ix_accounts,
            data: repay_ix_data,
        };

        invoke(&repay_ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = RepayOutputWrapper {
            token_in_amount: repay_token_account_and_balance.get_balance_change(),
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
        let stake_id: u8 = 18;

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = StakeInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        // Use remaining accounts
        let mut reserve_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 0);

        let stake_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), true),
            AccountMeta::new(ctx.remaining_accounts[5].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
        ];

        // [255, 255, 255, 255, 255, 255, 255, 255] is u64_max le byte array
        // Larix program will use all the balance in the token account
        let stake_data: Vec<u8> = vec![stake_id, 255, 255, 255, 255, 255, 255, 255, 255];

        let stake_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: stake_accounts,
            data: stake_data,
        };

        invoke(&stake_ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = StakeOutputWrapper {
            reserve_in_amount: reserve_token_account_and_balance.get_balance_change(),
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
        let unstake_id: u8 = 19;

        // Use remaining accounts
        let mut reserve_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 1);

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = UnstakeInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);
        let unstake_amount = input_struct.reserve_out_amount;

        let unstake_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
        ];

        let mut unstake_data = unstake_id.to_le_bytes().to_vec();
        unstake_data.append(unstake_amount.to_le_bytes().to_vec().clone().borrow_mut());

        let unstake_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unstake_accounts,
            data: unstake_data,
        };

        invoke(&unstake_ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = UnstakeOutputWrapper {
            reserve_out_amount: reserve_token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);
        msg!("Output: {:?}", output_struct);
        Ok(())
    }

    pub fn collateralize<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let collateralize_id: u8 = 8;
        // Use remaining accounts
        let mut reserve_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 0);
        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = CollateralizeInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let mut deposit_to_obligation_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
        ];
        if ctx.remaining_accounts.len() > 8 {
            // The last account that's not optional is at index 7
            // Optional accounts starts at index 8
            for index in 8..ctx.remaining_accounts.len() {
                deposit_to_obligation_accounts.push(AccountMeta::new_readonly(
                    ctx.remaining_accounts[index].key(),
                    false,
                ))
            }
        }

        // [255, 255, 255, 255, 255, 255, 255, 255] is u64_max le byte array
        // Larix program will use all the balance in the token account
        let deposit_data: Vec<u8> = vec![collateralize_id, 255, 255, 255, 255, 255, 255, 255, 255];

        let deposit_to_obligation_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key().clone(),
            accounts: deposit_to_obligation_accounts,
            data: deposit_data,
        };

        invoke(&deposit_to_obligation_ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = CollateralizeOutputWrapper {
            reserve_in_amount: reserve_token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);
        msg!("Output: {:?}", output_struct);
        Ok(())
    }

    pub fn uncollateralize<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let withdraw_obligation: u8 = 9;

        // Use remaining accounts
        let mut reserve_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 1);

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = UncollateralizeInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let reserved_amount = input_struct.reserve_out_amount;

        let unsupply_amount = reserved_amount.clone().to_le_bytes().to_vec();
        let withdraw_from_obligation_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
        ];

        let mut withdraw_from_obligation_data: Vec<u8> = vec![withdraw_obligation];
        withdraw_from_obligation_data.append(unsupply_amount.clone().borrow_mut());

        let withdraw_from_obligation_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: withdraw_from_obligation_accounts,
            data: withdraw_from_obligation_data,
        };

        invoke(&withdraw_from_obligation_ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = UncollateralizeOutputWrapper {
            reserve_out_amount: reserve_token_account_and_balance.get_balance_change(),
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
        let harvest_id: u8 = 20;

        // Use remaining accounts
        let mut reward_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 2);

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = HarvestInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let mut harvest_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
        ];
        if ctx.remaining_accounts.len() > 7 {
            // The last account that's not optional is at index 6
            // Optional accounts starts at index 7
            for index in 7..ctx.remaining_accounts.len() {
                harvest_accounts.push(AccountMeta::new_readonly(
                    ctx.remaining_accounts[index].key(),
                    false,
                ))
            }
        }
        let harvest_data: Vec<u8> = vec![harvest_id];

        let harvest_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key().clone(),
            accounts: harvest_accounts,
            data: harvest_data,
        };

        invoke(&harvest_ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = HarvestOutputWrapper {
            reward_amount: reward_token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);
        msg!("Output: {:?}", output_struct);
        Ok(())
    }

    pub fn claim_collateral_reward<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let claim_id: u8 = 21;

        // Use remaining accounts
        let mut reward_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 2);

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = ClaimCollateralRewardInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let mut claim_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
        ];
        if ctx.remaining_accounts.len() > 7 {
            // The last account that's not optional is at index 6
            // Optional accounts starts at index 7
            for index in 7..ctx.remaining_accounts.len() {
                claim_accounts.push(AccountMeta::new_readonly(
                    ctx.remaining_accounts[index].key(),
                    false,
                ))
            }
        }
        let claim_data: Vec<u8> = vec![claim_id];

        let harvest_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key().clone(),
            accounts: claim_accounts,
            data: claim_data,
        };

        invoke(&harvest_ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = ClaimCollateralRewardOutputWrapper {
            reward_amount: reward_token_account_and_balance.get_balance_change(),
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
    #[msg("Unsupported Action")]
    UnsupportedAction,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct SupplyInputWrapper {
    pub token_in_amount: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct SupplyOutputWrapper {
    pub reserve_out_amount: u64,
    pub supply_in_amount: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnsupplyInputWrapper {}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnsupplyOutputWrapper {
    pub supply_out_amount: u64,
    pub reserve_in_amount: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct BorrowInputWrapper {
    pub borrow_amount: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct BorrowOutputWrapper {
    pub token_out_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct RepayInputWrapper {
    pub repay_amount: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct RepayOutputWrapper {
    pub token_in_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeInputWrapper {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeOutputWrapper {
    pub reserve_in_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnstakeInputWrapper {
    pub reserve_out_amount: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnstakeOutputWrapper {
    pub reserve_out_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct CollateralizeInputWrapper {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct CollateralizeOutputWrapper {
    pub reserve_in_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UncollateralizeInputWrapper {
    pub reserve_out_amount: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UncollateralizeOutputWrapper {
    pub reserve_out_amount: u64,
    pub dummy_2: u64,
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
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct ClaimCollateralRewardInputWrapper {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct ClaimCollateralRewardOutputWrapper {
    pub reward_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// Make a tuple for being accessed by index rather than field name
pub type SupplyOutputTuple = (u64, u64, u64, u64);
pub type UnsupplyOutputTuple = (u64, u64, u64, u64);
pub type StakeOutputTuple = (u64, u64, u64, u64);
pub type UnstakeOutputTuple = (u64, u64, u64, u64);
pub type BorrowOutputTuple = (u64, u64, u64, u64);
pub type RepayOutputTuple = (u64, u64, u64, u64);
pub type CollateralizeOutputTuple = (u64, u64, u64, u64);
pub type UncollateralizeOutputTuple = (u64, u64, u64, u64);
pub type HarvestOutputTuple = (u64, u64, u64, u64);
pub type ClaimCollateralRewardOutputTuple = (u64, u64, u64, u64);

impl From<SupplyOutputWrapper> for SupplyOutputTuple {
    fn from(result: SupplyOutputWrapper) -> SupplyOutputTuple {
        let SupplyOutputWrapper {
            reserve_out_amount,
            supply_in_amount,
            dummy_3,
            dummy_4,
        } = result;
        (reserve_out_amount, supply_in_amount, dummy_3, dummy_4)
    }
}

impl From<UnsupplyOutputWrapper> for UnsupplyOutputTuple {
    fn from(result: UnsupplyOutputWrapper) -> UnsupplyOutputTuple {
        let UnsupplyOutputWrapper {
            supply_out_amount,
            reserve_in_amount,
            dummy_3,
            dummy_4,
        } = result;
        (supply_out_amount, reserve_in_amount, dummy_3, dummy_4)
    }
}

impl From<StakeOutputWrapper> for StakeOutputTuple {
    fn from(result: StakeOutputWrapper) -> StakeOutputTuple {
        let StakeOutputWrapper {
            reserve_in_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (reserve_in_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<UnstakeOutputWrapper> for UnstakeOutputTuple {
    fn from(result: UnstakeOutputWrapper) -> UnstakeOutputTuple {
        let UnstakeOutputWrapper {
            reserve_out_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (reserve_out_amount, dummy_2, dummy_3, dummy_4)
    }
}
impl From<CollateralizeOutputWrapper> for CollateralizeOutputTuple {
    fn from(result: CollateralizeOutputWrapper) -> CollateralizeOutputTuple {
        let CollateralizeOutputWrapper {
            reserve_in_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (reserve_in_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<UncollateralizeOutputWrapper> for UncollateralizeOutputTuple {
    fn from(result: UncollateralizeOutputWrapper) -> UncollateralizeOutputTuple {
        let UncollateralizeOutputWrapper {
            reserve_out_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (reserve_out_amount, dummy_2, dummy_3, dummy_4)
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
impl From<ClaimCollateralRewardOutputWrapper> for ClaimCollateralRewardOutputTuple {
    fn from(result: ClaimCollateralRewardOutputWrapper) -> ClaimCollateralRewardOutputTuple {
        let ClaimCollateralRewardOutputWrapper {
            reward_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (reward_amount, dummy_2, dummy_3, dummy_4)
    }
}
impl From<BorrowOutputWrapper> for BorrowOutputTuple {
    fn from(result: BorrowOutputWrapper) -> BorrowOutputTuple {
        let BorrowOutputWrapper {
            token_out_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (token_out_amount, dummy_2, dummy_3, dummy_4)
    }
}
impl From<RepayOutputWrapper> for RepayOutputTuple {
    fn from(result: RepayOutputWrapper) -> RepayOutputTuple {
        let RepayOutputWrapper {
            token_in_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (token_in_amount, dummy_2, dummy_3, dummy_4)
    }
}

pub fn load_token_account_and_balance<'info>(
    remaining_accounts: &[AccountInfo<'info>],
    account_index: usize,
) -> TokenAccountAndBalance<'info> {
    let token_account_info = &remaining_accounts[account_index];
    let token_account = Account::<TokenAccount>::try_from(token_account_info).unwrap();
    let balance_before = token_account.amount.clone();
    return TokenAccountAndBalance {
        token_accout: token_account,
        balance_before: balance_before,
    };
}

pub struct TokenAccountAndBalance<'info> {
    token_accout: Account<'info, TokenAccount>,
    balance_before: u64,
}

impl<'info> TokenAccountAndBalance<'info> {
    pub fn get_balance_change(&mut self) -> u64 {
        self.token_accout.reload().unwrap();
        let balance_before = self.balance_before;
        let balance_after = self.token_accout.amount;
        if balance_after > balance_before {
            balance_after.checked_sub(balance_before).unwrap()
        } else if balance_after == balance_before {
            0_u64
        } else {
            balance_before.checked_sub(balance_after).unwrap()
        }
    }
}
