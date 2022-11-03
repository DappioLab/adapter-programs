use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;
declare_id!("ADPTR3wPKDCZ8HNBBpY3GGXB8hu6DZDqyPJMimyHjKNk");

#[program]
pub mod adapter_genopets_staking {
    use super::*;

    pub fn stake<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = StakeInputWrapper::deserialize(&mut input_bytes)?;

        let mut stake_data = sighash("global", "stake").to_vec();
        stake_data.append(&mut input_struct.amount.to_le_bytes().to_vec());
        stake_data.append(&mut input_struct.lock_for_months.to_le_bytes().to_vec());
        stake_data.append(&mut input_struct.as_sgene.try_to_vec()?);

        let stake_accounts = load_remaining_accounts(
            ctx.remaining_accounts,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        );

        let mut stake_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 5);

        let stake_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: stake_accounts,
            data: stake_data,
        };
        invoke(&stake_ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = StakeOutputWrapper {
            token_in_amount: stake_token_account_and_balance.get_balance_change(),
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
        let mut input_bytes = &input[..];
        let input_struct = UnstakeInputWrapper::deserialize(&mut input_bytes)?;

        let mut unstake_data = vec![]; // Instruction data
        let mut unstake_accout_index_array: Vec<usize> = vec![]; // Remaining accounts 
        let mut unstake_token_account_index: usize = 0;
        if input_struct.as_sgene {
            unstake_data = sighash("global", "withdraw_as_sgene").to_vec();
            unstake_accout_index_array =
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
            unstake_token_account_index = 4;
        } else {
            unstake_data = sighash("global", "withdraw").to_vec();
            unstake_data.push(0); // default False cuz it's deprecated
            unstake_accout_index_array = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            unstake_token_account_index = 5;
        }

        let unstake_ix_accounts =
            load_remaining_accounts(ctx.remaining_accounts, unstake_accout_index_array);

        let mut unstake_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, unstake_token_account_index);

        let unstake_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unstake_ix_accounts,
            data: unstake_data,
        };
        invoke(&unstake_ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = UnstakeOutputWrapper {
            token_out_amount: unstake_token_account_and_balance.get_balance_change(),
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
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = HarvestInputWrapper::deserialize(&mut input_bytes)?;

        let harvest_data = sighash("global", "claim_rewards").to_vec();

        let harvest_accounts = load_remaining_accounts(
            ctx.remaining_accounts,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        );

        let harvest_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: harvest_accounts,
            data: harvest_data,
        };
        invoke(&harvest_ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = HarvestOutputWrapper {
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeInputWrapper {
    pub amount: u64,
    pub lock_for_months: u8,
    pub as_sgene: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeOutputWrapper {
    pub token_in_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnstakeInputWrapper {
    pub as_sgene: bool,
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnstakeOutputWrapper {
    pub token_out_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct HarvestInputWrapper {}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct HarvestOutputWrapper {
    pub dummy_1: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

pub type StakeOutputTuple = (u64, u64, u64, u64);
pub type UnstakeOutputTuple = (u64, u64, u64, u64);
pub type HarvestOutputTuple = (u64, u64, u64, u64);

impl From<StakeOutputWrapper> for StakeOutputTuple {
    fn from(result: StakeOutputWrapper) -> StakeOutputTuple {
        let StakeOutputWrapper {
            token_in_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (token_in_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<UnstakeOutputWrapper> for UnstakeOutputTuple {
    fn from(result: UnstakeOutputWrapper) -> UnstakeOutputTuple {
        let UnstakeOutputWrapper {
            token_out_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (token_out_amount, dummy_2, dummy_3, dummy_4)
    }
}
impl From<HarvestOutputWrapper> for HarvestOutputTuple {
    fn from(result: HarvestOutputWrapper) -> HarvestOutputTuple {
        let HarvestOutputWrapper {
            dummy_1,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (dummy_1, dummy_2, dummy_3, dummy_4)
    }
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

pub fn load_remaining_accounts<'info>(
    remaining_accounts: &[AccountInfo<'info>],
    index_array: Vec<usize>,
) -> Vec<AccountMeta> {
    let mut accounts: Vec<AccountMeta> = vec![];
    for index in index_array.iter() {
        if remaining_accounts[*index].is_writable {
            accounts.push(AccountMeta::new(
                remaining_accounts[*index].key(),
                remaining_accounts[*index].is_signer,
            ))
        } else {
            accounts.push(AccountMeta::new_readonly(
                remaining_accounts[*index].key(),
                remaining_accounts[*index].is_signer,
            ))
        }
    }
    return accounts;
}
