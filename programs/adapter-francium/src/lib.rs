use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

use anchor_spl::token::TokenAccount;
declare_id!("ADPTax5HwQ2ZWVLmceCek8UrqMhwCy5q3SHwi8W71Kv2");

#[program]
pub mod adapter_francium {

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
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
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
            load_token_account_and_balance(ctx.remaining_accounts, 1);

        let mut reserve_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 0);

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = UnsupplyInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let reserved_amount = input_struct.reserve_amount;

        let unsupply_amount_vec = reserved_amount.clone().to_le_bytes().to_vec();

        let unsupply_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
        ];

        let mut unsupply_ix_data: Vec<u8> = vec![];
        unsupply_ix_data.append(&mut withdraw_reserve_id.to_le_bytes().to_vec());
        unsupply_ix_data.append(&mut unsupply_amount_vec.clone());

        let unsupply_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unsupply_accounts,
            data: unsupply_ix_data,
        };

        invoke(&unsupply_ix, ctx.remaining_accounts)?;
        // Wrap Output
        let output_struct = UnsupplyOutputWrapper {
            reserve_in_amount: reserve_token_account_and_balance.get_balance_change(),
            token_out_amount: supply_token_account_and_balance.get_balance_change(),
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
        let deposit_reward_id: u8 = 3;

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = StakeInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        // Use remaining accounts
        let mut reserve_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 2);

        let deposit_reward_account = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
        ];

        let deposit_reward_data: Vec<u8> = vec![deposit_reward_id, 0, 0, 0, 0, 0, 0, 0, 0];

        let deposit_reward = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: deposit_reward_account,
            data: deposit_reward_data,
        };

        invoke(&deposit_reward, ctx.remaining_accounts)?;

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
        let withdraw_reward_id: u8 = 4;
        // Use remaining accounts
        let mut reserve_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 2);

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = UnstakeInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let unsupply_amount = input_struct.reserve_out_amount;

        let withdraw_reward_account = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
        ];

        let mut withdraw_reward_data: Vec<u8> = vec![withdraw_reward_id];

        withdraw_reward_data.append(&mut unsupply_amount.clone().to_le_bytes().to_vec());

        msg!(&*format!("{:?}", withdraw_reward_data));

        let withdraw_reward = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: withdraw_reward_account,
            data: withdraw_reward_data,
        };

        invoke(&withdraw_reward, ctx.remaining_accounts)?;
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
pub struct UnsupplyInputWrapper {
    pub reserve_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnsupplyOutputWrapper {
    pub token_out_amount: u64,
    pub reserve_in_amount: u64,
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
// Make a tuple for being accessed by index rather than field name
pub type SupplyOutputTuple = (u64, u64, u64, u64);
pub type UnsupplyOutputTuple = (u64, u64, u64, u64);
pub type StakeOutputTuple = (u64, u64, u64, u64);
pub type UnstakeOutputTuple = (u64, u64, u64, u64);
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
            token_out_amount,
            reserve_in_amount,
            dummy_3,
            dummy_4,
        } = result;
        (token_out_amount, reserve_in_amount, dummy_3, dummy_4)
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
