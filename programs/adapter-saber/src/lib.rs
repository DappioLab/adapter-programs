use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

declare_id!("8s41oFgb8cFqeeTJZ4W5HN1LCmKkBCGTiZFUDmMyAsHT");

#[program]
pub mod adapter_saber {
    use super::*;

    pub fn add_liquidity<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = AddLiquidityInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        // Use remaining accounts
        let lp_token_account_info = ctx.remaining_accounts[8].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        // Get the data from input_struct
        let (pool_token_a_in_amount, pool_token_b_in_amount) = match input_struct.pool_direction {
            // Obverse
            0 => (0, input_struct.token_in_amount),
            // Reverse
            1 => (input_struct.token_in_amount, 0),
            _ => return Err(ErrorCode::UnsupportedPoolDirection.into()),
        };

        let add_lp_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), true),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
        ];

        const ADD_LP_IX: u8 = 2;
        let minimal_receive: u64 = 0;
        let mut add_lp_data = vec![];
        add_lp_data.append(&mut ADD_LP_IX.try_to_vec()?);
        add_lp_data.append(&mut pool_token_a_in_amount.try_to_vec()?);
        add_lp_data.append(&mut pool_token_b_in_amount.try_to_vec()?);
        add_lp_data.append(&mut minimal_receive.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: add_lp_accounts,
            data: add_lp_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;
        lp_token_account.reload()?;

        let lp_token_amount_after = lp_token_account.amount;
        let lp_amount = lp_token_amount_after - lp_token_amount_before;

        // Wrap Output
        let output_struct = AddLiquidityOutputWrapper {
            lp_amount,
            dummy_2: 1000,
            ..Default::default()
        };

        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn remove_liquidity<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = RemoveLiquidityInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);


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
                    AccountMeta::new(ctx.remaining_accounts[10].key(), false),
                    AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
                ];

                let minimal_receive: u64 = 0;

                let mut remove_lp_data = vec![];
                remove_lp_data.append(&mut REMOVE_LP_IX.try_to_vec()?);
                remove_lp_data.append(&mut input_struct.lp_amount.try_to_vec()?);
                remove_lp_data.append(&mut minimal_receive.try_to_vec()?);
                remove_lp_data.append(&mut minimal_receive.try_to_vec()?);

                let token_a_account_info = ctx.remaining_accounts[7].clone();
                let token_a_account = Account::<TokenAccount>::try_from(&token_a_account_info)?;
                let token_a_amount_before = token_a_account.amount;

                let token_b_account_info = ctx.remaining_accounts[8].clone();
                let token_b_account = Account::<TokenAccount>::try_from(&token_b_account_info)?;
                let token_b_amount_before = token_b_account.amount;

                (Instruction {
                    program_id: ctx.accounts.base_program_id.key(),
                    accounts: remove_lp_accounts,
                    data: remove_lp_data,
                }, token_a_amount_before, token_b_amount_before)
            }
            // RemoveLiquiditySingle
            3 => {
                const REMOVE_LP_IX: u8 = 4;

                let remove_lp_accounts = vec![
                    AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
                    AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
                    AccountMeta::new(ctx.remaining_accounts[2].key(), true),
                    AccountMeta::new(ctx.remaining_accounts[3].key(), false),
                    AccountMeta::new(ctx.remaining_accounts[4].key(), false),
                    AccountMeta::new(ctx.remaining_accounts[5].key(), false),
                    AccountMeta::new(ctx.remaining_accounts[6].key(), false),
                    AccountMeta::new(ctx.remaining_accounts[7].key(), false),
                    AccountMeta::new(ctx.remaining_accounts[8].key(), false),
                    AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
                ];

                let minimal_receive: u64 = 0;

                let mut remove_lp_data = vec![];
                remove_lp_data.append(&mut REMOVE_LP_IX.try_to_vec()?);
                remove_lp_data.append(&mut input_struct.lp_amount.try_to_vec()?);
                remove_lp_data.append(&mut minimal_receive.try_to_vec()?);


                let token_a_account_info = ctx.remaining_accounts[7].clone();
                let token_a_account = Account::<TokenAccount>::try_from(&token_a_account_info)?;
                let token_a_amount_before = token_a_account.amount;

                (Instruction {
                    program_id: ctx.accounts.base_program_id.key(),
                    accounts: remove_lp_accounts,
                    data: remove_lp_data,
                }, token_a_amount_before, 0)
            }
            _ => {
                return Err(ErrorCode::UnsupportedAction.into());
            }
        };

        invoke(&ix, ctx.remaining_accounts)?;

        let (token_a_amount_after, token_b_amount_after) = match input_struct.action {
            2 => {
                let token_a_account_info = ctx.remaining_accounts[7].clone();
                let mut token_a_account = Account::<TokenAccount>::try_from(&token_a_account_info)?;
                let token_a_amount_after = token_a_account.amount;

                let token_b_account_info = ctx.remaining_accounts[8].clone();
                let mut token_b_account = Account::<TokenAccount>::try_from(&token_b_account_info)?;
                let token_b_amount_after = token_b_account.amount;

                (token_a_amount_after, token_b_amount_after)
            }
            3 => {
                let token_a_account_info = ctx.remaining_accounts[7].clone();
                let mut token_a_account = Account::<TokenAccount>::try_from(&token_a_account_info)?;
                let token_a_amount_after = token_a_account.amount;

                (token_a_amount_after, 0)
            }
            _ => {
                return Err(ErrorCode::UnsupportedAction.into());
            }
        };

        let token_a_amount = token_a_amount_after - token_a_amount_before;
        let token_b_amount = token_b_amount_after - token_b_amount_before;

        // Wrap Output
        // NOTICE: for RemoveLiquiditySingle no mater is token A or token B, we'll update
        // the amount in token_a_amount since there's only one tokenAccount state might change
        // and also avoid determine token A or B by uncertain pool direction.
        let output_struct = RemoveLiquidityOutputWrapper {
            token_a_amount,
            token_b_amount,
            ..Default::default()
        };

        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn stake<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = StakeInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let sighash_arr = sighash("global", "stake_tokens");

        let stake_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
        ];

        let mut stake_data = vec![];
        stake_data.append(&mut sighash_arr.try_to_vec()?);
        stake_data.append(&mut input_struct.lp_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: stake_accounts,
            data: stake_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = StakeOutputWrapper {
            dummy_1: 1000,
            dummy_2: 2000,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

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

        msg!("Input: {:?}", input_struct);

        let sighash_arr = sighash("global", "withdraw_tokens");

        let unstake_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
        ];

        let mut unstake_data = vec![];
        unstake_data.append(&mut sighash_arr.try_to_vec()?);
        unstake_data.append(&mut input_struct.share_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unstake_accounts,
            data: unstake_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = UnstakeOutputWrapper {
            lp_amount: input_struct.share_amount,
            dummy_2: 2000,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

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

        msg!("Input: {:?}", input_struct);

        let sighash_arr = sighash("global", "claim_rewards");

        let harvest_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[12].key(), false),
        ];

        let mut harvest_data = vec![];
        harvest_data.append(&mut sighash_arr.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: harvest_accounts,
            data: harvest_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = HarvestOutputWrapper {
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum PoolDirection {
    Obverse,
    Reverse,
}

#[derive(Accounts)]
pub struct Action<'info> {
    pub gateway_authority: Signer<'info>,
    /// CHECK: Safe
    pub base_program_id: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct AddLiquidityInputWrapper {
    pub token_in_amount: u64,
    pub pool_direction: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct RemoveLiquidityInputWrapper {
    pub lp_amount: u64,
    pub action: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeInputWrapper {
    pub lp_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnstakeInputWrapper {
    pub share_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct HarvestInputWrapper {}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct AddLiquidityOutputWrapper {
    pub lp_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct RemoveLiquidityOutputWrapper {
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeOutputWrapper {
    pub dummy_1: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnstakeOutputWrapper {
    pub lp_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct HarvestOutputWrapper {
    pub dummy_1: u64,
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
            token_a_amount,
            token_b_amount,
            dummy_3,
            dummy_4,
        } = result;
        (token_a_amount, token_b_amount, dummy_3, dummy_4)
    }
}

impl From<StakeOutputWrapper> for StakeOutputTuple {
    fn from(result: StakeOutputWrapper) -> StakeOutputTuple {
        let StakeOutputWrapper {
            dummy_1,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (dummy_1, dummy_2, dummy_3, dummy_4)
    }
}

impl From<UnstakeOutputWrapper> for UnstakeOutputTuple {
    fn from(result: UnstakeOutputWrapper) -> UnstakeOutputTuple {
        let UnstakeOutputWrapper {
            lp_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (lp_amount, dummy_2, dummy_3, dummy_4)
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
