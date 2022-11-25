use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

declare_id!("ADPTyBr92sBCE1hdYBRvXbMpF4hKs17xyDjFPxopcsrh");

#[program]
pub mod adapter_nft_finance {
    use super::*;

    pub fn lock_nft<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = LockNftInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let prove_token_account_info = ctx.remaining_accounts[7].clone();
        let mut prove_token_account = Account::<TokenAccount>::try_from(&prove_token_account_info)?;
        let prove_token_amount_before = prove_token_account.amount;

        let lock_nft_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[12].key(), false),
        ];

        let mut lock_nft_data = vec![];
        let sighash_arr = sighash("global", "stake");
        lock_nft_data.append(&mut sighash_arr.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: lock_nft_accounts,
            data: lock_nft_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        prove_token_account.reload()?;

        let prove_token_amount_after = prove_token_account.amount;
        let prove_token_amount = prove_token_amount_after - prove_token_amount_before;

        // Wrap Output
        let output_struct = LockNftOutputWrapper {
            prove_token_amount,
            ..Default::default()
        };

        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn unlock_nft<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = UnlockNftInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let unlock_nft_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
        ];

        let mut unlock_nft_data = vec![];
        let sighash_arr = sighash("global", "unstake");
        unlock_nft_data.append(&mut sighash_arr.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unlock_nft_accounts,
            data: unlock_nft_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = UnlockNftOutputWrapper {
            ..Default::default()
        };

        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn stake_proof<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = StakeProofInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let farm_token_account_info = ctx.remaining_accounts[4].clone();
        let mut farm_token_account = Account::<TokenAccount>::try_from(&farm_token_account_info)?;
        let farm_token_amount_before = farm_token_account.amount;

        let stake_proof_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
        ];

        let mut stake_proof_data = vec![];
        let sighash_arr = sighash("global", "deposit");
        stake_proof_data.append(&mut sighash_arr.try_to_vec()?);
        stake_proof_data.append(&mut input_struct.prove_token_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: stake_proof_accounts,
            data: stake_proof_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        farm_token_account.reload()?;

        let farm_token_amount_after = farm_token_account.amount;
        let farm_token_amount = farm_token_amount_after - farm_token_amount_before;

        // Wrap Output
        let output_struct = StakeProofOutputWrapper {
            farm_token_amount,
            ..Default::default()
        };

        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn unstake_proof<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = UnstakeProofInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let prove_token_account_info = ctx.remaining_accounts[3].clone();
        let mut prove_token_account = Account::<TokenAccount>::try_from(&prove_token_account_info)?;
        let prove_token_amount_before = prove_token_account.amount;

        let unstake_proof_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
        ];

        let mut unstake_proof_data = vec![];
        let sighash_arr = sighash("global", "withdraw");
        unstake_proof_data.append(&mut sighash_arr.try_to_vec()?);
        unstake_proof_data.append(&mut input_struct.farm_token_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unstake_proof_accounts,
            data: unstake_proof_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        prove_token_account.reload()?;

        let prove_token_amount_after = prove_token_account.amount;
        let prove_token_amount = prove_token_amount_after - prove_token_amount_before;

        // Wrap Output
        let output_struct = UnstakeProofOutputWrapper {
            prove_token_amount,
            ..Default::default()
        };

        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn claim<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = ClaimInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let reward_token_account_info = ctx.remaining_accounts[2].clone();
        let mut reward_token_account =
            Account::<TokenAccount>::try_from(&reward_token_account_info)?;
        let reward_token_amount_before = reward_token_account.amount;

        let claim_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
        ];

        let mut claim_data = vec![];
        let sighash_arr = sighash("global", "claim");
        claim_data.append(&mut sighash_arr.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: claim_accounts,
            data: claim_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        reward_token_account.reload()?;

        let reward_token_amount_after = reward_token_account.amount;
        let reward_token_amount = reward_token_amount_after - reward_token_amount_before;

        // Wrap Output
        let output_struct = ClaimOutputWrapper {
            reward_token_amount,
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

// InputWrapper
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct LockNftInputWrapper {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnlockNftInputWrapper {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeProofInputWrapper {
    pub prove_token_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnstakeProofInputWrapper {
    pub farm_token_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct ClaimInputWrapper {}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct LockNftOutputWrapper {
    pub prove_token_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnlockNftOutputWrapper {
    pub dummy_1: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeProofOutputWrapper {
    pub farm_token_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnstakeProofOutputWrapper {
    pub prove_token_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct ClaimOutputWrapper {
    pub reward_token_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// Make a tuple for being accessed by index rather than field name
pub type LockNftOutputTuple = (u64, u64, u64, u64);
pub type UnlockNftOutputTuple = (u64, u64, u64, u64);
pub type StakeProofOutputTuple = (u64, u64, u64, u64);
pub type UnstakeProofOutputTuple = (u64, u64, u64, u64);
pub type ClaimOutputTuple = (u64, u64, u64, u64);

impl From<LockNftOutputWrapper> for LockNftOutputTuple {
    fn from(result: LockNftOutputWrapper) -> LockNftOutputTuple {
        let LockNftOutputWrapper {
            prove_token_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (prove_token_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<UnlockNftOutputWrapper> for UnlockNftOutputTuple {
    fn from(result: UnlockNftOutputWrapper) -> UnlockNftOutputTuple {
        let UnlockNftOutputWrapper {
            dummy_1,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (dummy_1, dummy_2, dummy_3, dummy_4)
    }
}

impl From<StakeProofOutputWrapper> for StakeProofOutputTuple {
    fn from(result: StakeProofOutputWrapper) -> StakeProofOutputTuple {
        let StakeProofOutputWrapper {
            farm_token_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (farm_token_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<UnstakeProofOutputWrapper> for UnstakeProofOutputTuple {
    fn from(result: UnstakeProofOutputWrapper) -> UnstakeProofOutputTuple {
        let UnstakeProofOutputWrapper {
            prove_token_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (prove_token_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<ClaimOutputWrapper> for ClaimOutputTuple {
    fn from(result: ClaimOutputWrapper) -> ClaimOutputTuple {
        let ClaimOutputWrapper {
            reward_token_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (reward_token_amount, dummy_2, dummy_3, dummy_4)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unsupported PoolDirection")]
    UnsupportedPoolDirection,
    #[msg("Unsupported Action")]
    UnsupportedAction,
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];

    sighash.copy_from_slice(&hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}
