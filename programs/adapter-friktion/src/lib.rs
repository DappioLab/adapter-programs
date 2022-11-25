use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

declare_id!("ADPTzbsaBdXA3FqXoPHjaTjPfh9kadxxFKxonZihP1Ji");

#[program]
pub mod adapter_friktion {
    use super::*;

    pub fn initiate_deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = InitiateDepositInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let mut deposit_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 9);

        let mut deposit_data = sighash("global", "deposit").to_vec();
        deposit_data.append(&mut input_struct.deposit_amount.try_to_vec()?);
        let deposit_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new(ctx.remaining_accounts[12].key(), false),
            AccountMeta::new(ctx.remaining_accounts[13].key(), false),
            AccountMeta::new(ctx.remaining_accounts[14].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[15].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[16].key(), false),
        ];
        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: deposit_accounts,
            data: deposit_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = InitiateDepositOutputWrapper {
            deposit_amount: deposit_token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn initiate_withdrawal<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = InitiateWithdrawInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let mut share_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 8);

        let mut data = sighash("global", "withdraw").to_vec();
        data.append(&mut input_struct.share_amount.try_to_vec()?);

        let accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new(ctx.remaining_accounts[12].key(), false),
            AccountMeta::new(ctx.remaining_accounts[13].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[14].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[15].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[16].key(), false),
        ];

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts,
            data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = InitiateWithdrawOutputWrapper {
            share_amount: share_token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn cancel_withdrawal<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = CancelWithdrawInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let mut share_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 5);

        let data = sighash("global", "cancel_pending_withdrawal").to_vec();
        let accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
        ];

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts,
            data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = CancelWithdrawOutputWrapper {
            share_amount: share_token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn cancel_deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = CancelDepositInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let mut deposit_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 4);

        let data = sighash("global", "cancel_pending_deposit").to_vec();
        let accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
        ];

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts,
            data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = CancelDepositOutputWrapper {
            withdraw_amount: deposit_token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn finalize_withdrawal<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = FinalizeWithdrawInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let mut deposit_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 5);

        let data = sighash("global", "claim_pending_withdrawal").to_vec();
        let accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
        ];

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts,
            data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = FinalizeWithdrawOutputWrapper {
            withdraw_amount: deposit_token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn finalize_deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = FinalizeDepositInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let mut share_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 4);

        let data = sighash("global", "claim_pending_deposit").to_vec();
        let accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
        ];

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts,
            data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = FinalizeDepositOutputWrapper {
            share_amount: share_token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct InitiateDepositInputWrapper {
    pub deposit_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct InitiateWithdrawInputWrapper {
    pub share_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct CancelDepositInputWrapper {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct CancelWithdrawInputWrapper {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct FinalizeDepositInputWrapper {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct FinalizeWithdrawInputWrapper {}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct InitiateDepositOutputWrapper {
    pub deposit_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct InitiateWithdrawOutputWrapper {
    pub share_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct CancelDepositOutputWrapper {
    pub withdraw_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct CancelWithdrawOutputWrapper {
    pub share_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct FinalizeDepositOutputWrapper {
    pub share_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct FinalizeWithdrawOutputWrapper {
    pub withdraw_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

pub type InitiateDepositOutputTuple = (u64, u64, u64, u64);
pub type InitiateWithdrawOutputTuple = (u64, u64, u64, u64);
pub type CancelDepositOutputTuple = (u64, u64, u64, u64);
pub type CancelWithdrawOutputTuple = (u64, u64, u64, u64);
pub type FinalizeDepositOutputTuple = (u64, u64, u64, u64);
pub type FinalizeWithdrawOutputTuple = (u64, u64, u64, u64);

impl From<InitiateDepositOutputWrapper> for InitiateDepositOutputTuple {
    fn from(result: InitiateDepositOutputWrapper) -> InitiateDepositOutputTuple {
        let InitiateDepositOutputWrapper {
            deposit_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (deposit_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<InitiateWithdrawOutputWrapper> for InitiateWithdrawOutputTuple {
    fn from(result: InitiateWithdrawOutputWrapper) -> InitiateWithdrawOutputTuple {
        let InitiateWithdrawOutputWrapper {
            share_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (share_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<CancelDepositOutputWrapper> for CancelDepositOutputTuple {
    fn from(result: CancelDepositOutputWrapper) -> CancelDepositOutputTuple {
        let CancelDepositOutputWrapper {
            withdraw_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (withdraw_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<CancelWithdrawOutputWrapper> for CancelWithdrawOutputTuple {
    fn from(result: CancelWithdrawOutputWrapper) -> CancelWithdrawOutputTuple {
        let CancelWithdrawOutputWrapper {
            share_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (share_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<FinalizeDepositOutputWrapper> for FinalizeDepositOutputTuple {
    fn from(result: FinalizeDepositOutputWrapper) -> FinalizeDepositOutputTuple {
        let FinalizeDepositOutputWrapper {
            share_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (share_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<FinalizeWithdrawOutputWrapper> for FinalizeWithdrawOutputTuple {
    fn from(result: FinalizeWithdrawOutputWrapper) -> FinalizeWithdrawOutputTuple {
        let FinalizeWithdrawOutputWrapper {
            withdraw_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (withdraw_amount, dummy_2, dummy_3, dummy_4)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unsupported Action")]
    UnsupportedAction,
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];

    sighash.copy_from_slice(&hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
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
    }; // (token_account.clone(), token_account.amount.clone());
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
