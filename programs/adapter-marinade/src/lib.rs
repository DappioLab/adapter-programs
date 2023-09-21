use anchor_lang::prelude::*;
use anchor_lang::solana_program::{hash::hash, instruction::Instruction, program::invoke};

declare_id!("3fUTvDPGzfRHpRkbZdTGzHzZHzBm1Km5cyAssfKZwTh3");

pub mod adapter_marinade {
    use adapter_common::{load_remaining_accounts, load_token_account_and_balance, sighash};

    use super::*;
    use std::borrow::BorrowMut;

    pub fn deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let mut deposit_data = sighash("global", "deposit").to_vec();
        let mut input_bytes = &input[..];
        let input_struct = DepositInputWrapper::deserialize(&mut input_bytes)?;

        let mut share_balance = load_token_account_and_balance(ctx.remaining_accounts, 7);
        let mut token_balance = load_token_account_and_balance(ctx.remaining_accounts, 6);
        let deposit_amount = input_struct.deposit_amount;

        deposit_data.append(deposit_amount.to_le_bytes().to_vec().borrow_mut());
        let deposit_accounts = load_remaining_accounts(
            ctx.remaining_accounts,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        );

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: deposit_accounts,
            data: deposit_data,
        };
        invoke(&ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = DepositOutputWrapper {
            share_amount: share_balance.get_balance_change(),
            deposit_amount: token_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        // Return Result
        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }
    pub fn withdraw<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let mut withdraw_data = sighash("global", "liquid_unstake").to_vec();
        let mut input_bytes = &input[..];
        let input_struct = WithdrawInputWrapper::deserialize(&mut input_bytes)?;

        let mut share_balance = load_token_account_and_balance(ctx.remaining_accounts, 5);
        let mut token_balance = load_token_account_and_balance(ctx.remaining_accounts, 7);
        let withdraw_amount = input_struct.withdraw_amount;

        withdraw_data.append(withdraw_amount.to_le_bytes().to_vec().borrow_mut());
        let withdraw_accounts = load_remaining_accounts(
            ctx.remaining_accounts,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        );

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: withdraw_accounts,
            data: withdraw_data,
        };
        invoke(&ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = WithdrawOutputWrapper {
            share_amount: share_balance.get_balance_change(),
            withdraw_amount: token_balance.get_balance_change(),
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct DepositInputWrapper {
    pub deposit_amount: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct DepositOutputWrapper {
    pub share_amount: u64,
    pub deposit_amount: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct WithdrawInputWrapper {
    pub withdraw_amount: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct WithdrawOutputWrapper {
    pub withdraw_amount: u64,
    pub share_amount: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

pub struct Action<'info> {
    // TODO: Add constraints
    pub gateway_authority: Signer<'info>,
    /// CHECK: Safe
    pub base_program_id: AccountInfo<'info>,
}
