use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
    stake,
};
use anchor_spl::token::TokenAccount;

declare_id!("4XnyVwkN5dR7Rinnys59qMqLjhxytpv4JbFLJuXe5mKk");

#[program]
pub mod adapter_lido {
    use anchor_lang::solana_program::program::invoke_signed;

    use super::*;

    pub fn deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = DepositInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let mut recipient_st_sol_account =
            Account::<TokenAccount>::try_from(&ctx.remaining_accounts[2])?;

        let token_amount = recipient_st_sol_account.amount;

        let deposit_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), true),
            AccountMeta::new(recipient_st_sol_account.key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
        ];

        // Prepend instruction byte to Solido Input
        let mut data = vec![1u8];
        data.extend(input);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: deposit_accounts,
            data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        recipient_st_sol_account.reload()?;

        let share_amount = recipient_st_sol_account
            .amount
            .checked_sub(token_amount)
            .unwrap();

        // Wrap Output
        let output_struct = DepositOutputWrapper {
            share_amount,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn withdraw<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = WithdrawInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let user_account = ctx.remaining_accounts[1].clone();
        let account_balance = user_account.lamports();

        let withdraw_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new_readonly(user_account.key(), true),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[12].key(), false),
        ];

        let mut data = vec![23u8];
        data.extend(input);

        let withdraw_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: withdraw_accounts,
            data,
        };

        invoke(&withdraw_ix, ctx.remaining_accounts)?;

        let deactivate_ix = stake::instruction::deactivate_stake(
            &user_account.key(),
            &ctx.remaining_accounts[6].key(),
        );
        invoke_signed(&deactivate_ix, &vec![user_account.clone()], &[&[&user_account.key().to_bytes(), &input_struct.gateway_key.to_bytes()]])?;

        let lp_amount = user_account
            .lamports()
            .checked_sub(account_balance)
            .unwrap();

        // Wrap Output
        let output_struct = WithdrawOutputWrapper {
            lp_amount,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Action<'info> {
    pub gateway_authority: Signer<'info>,
    /// CHECK: Safe
    pub base_program_id: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct DepositInputWrapper {
    /// Amount to deposit.
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct WithdrawInputWrapper {
    /// Amount to withdraw.
    pub amount: u64,
    /// Index of the Heaviest Validator.
    pub validator_index: u32,
    pub gateway_key: Pubkey,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct DepositOutputWrapper {
    pub share_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct WithdrawOutputWrapper {
    pub lp_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

pub type DepositOutputTuple = (u64, u64, u64, u64);
pub type WithdrawOutputTuple = (u64, u64, u64, u64);

impl From<DepositOutputWrapper> for DepositOutputTuple {
    fn from(result: DepositOutputWrapper) -> DepositOutputTuple {
        let DepositOutputWrapper {
            share_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (share_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<WithdrawOutputWrapper> for WithdrawOutputTuple {
    fn from(result: WithdrawOutputWrapper) -> WithdrawOutputTuple {
        let WithdrawOutputWrapper {
            lp_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (lp_amount, dummy_2, dummy_3, dummy_4)
    }
}
