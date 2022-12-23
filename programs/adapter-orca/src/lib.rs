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
    use std::vec;

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
        let mut lp_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 7);
        // Get the data from input_struct
        let pool_token_in_amount = input_struct.token_in_amount;
        // Load Ix accounts
        let add_lp_accounts =
            load_remaining_accounts(ctx.remaining_accounts, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);

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

        // Wrap Output
        let output_struct = AddLiquidityOutputWrapper {
            lp_amount: lp_token_account_and_balance.get_balance_change(),
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
        let mut lp_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 4);
        msg!("lp_amount: {}", lp_amount.to_string());
        let (ix, mut token_a_account_and_balance, token_b_account_and_balance) =
            match input_struct.action {
                // RemoveLiquidity
                2 => {
                    const REMOVE_LP_IX: u8 = 3;
                    let remove_lp_accounts = load_remaining_accounts(
                        ctx.remaining_accounts,
                        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                    );

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
                        load_token_account_and_balance(ctx.remaining_accounts, 7),
                        Some(load_token_account_and_balance(ctx.remaining_accounts, 8)),
                    )
                }
                // RemoveLiquiditySingle
                3 => {
                    const REMOVE_LP_IX: u8 = 5;
                    let remove_lp_accounts = load_remaining_accounts(
                        ctx.remaining_accounts,
                        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                    );

                    let minimal_receive: u64 = 1;

                    let mut remove_lp_data = vec![];
                    remove_lp_data.append(&mut REMOVE_LP_IX.to_le_bytes().to_vec());
                    remove_lp_data.append(&mut minimal_receive.to_le_bytes().to_vec());
                    remove_lp_data.append(&mut lp_amount.to_le_bytes().to_vec());
                    (
                        Instruction {
                            program_id: ctx.accounts.base_program_id.key(),
                            accounts: remove_lp_accounts,
                            data: remove_lp_data,
                        },
                        load_token_account_and_balance(ctx.remaining_accounts, 7),
                        None,
                    )
                }
                _ => {
                    return Err(ErrorCode::UnsupportedAction.into());
                }
            };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = RemoveLiquidityOutputWrapper {
            token_a_out_amount: token_a_account_and_balance.get_balance_change(),
            token_b_out_amount: match token_b_account_and_balance {
                Some(mut i) => i.get_balance_change(),
                None => 0_u64,
            },
            lp_amount: lp_token_account_and_balance.get_balance_change(),
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
        let mut lp_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 1);

        let mut share_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 5);

        let stake_accounts = load_remaining_accounts(
            ctx.remaining_accounts,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        );

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

        // Wrap Output
        let output_struct = StakeOutputWrapper {
            share_amount: share_token_account_and_balance.get_balance_change(),
            lp_amount: lp_token_account_and_balance.get_balance_change(),
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
        let mut lp_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 1);

        let mut share_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 4);
        let unstake_accounts = load_remaining_accounts(
            ctx.remaining_accounts,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        );

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

        // Wrap Output
        let output_struct = UnstakeOutputWrapper {
            share_amount: share_token_account_and_balance.get_balance_change(),
            lp_amount: lp_token_account_and_balance.get_balance_change(),
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
        let mut reward_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 1);

        let mut input_bytes = &input[..];
        let input_struct = HarvestInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        // Harvest
        let harvest_ix: u8 = 4;
        let harvest_accounts =
            load_remaining_accounts(ctx.remaining_accounts, vec![0, 1, 2, 3, 4, 5, 6, 7]);

        let mut harvest_data = vec![];
        harvest_data.push(harvest_ix);
        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: harvest_accounts,
            data: harvest_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = HarvestOutputWrapper {
            reward_amount: reward_account_and_balance.get_balance_change(),
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
