use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

pub mod price;
use crate::price::{get_raydium_lp_price, PoolDirectionP};

declare_id!("ADPT1q4xG8F9m64cQyjqGe11cCXQq6vL4beY5hJavhQ5");

#[program]
pub mod adapter_raydium {
    use super::*;

    pub fn swap<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = SwapInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        // Use remaining accounts
        let dest_token_account_info = ctx.remaining_accounts[16].clone();
        let mut dest_token_account = Account::<TokenAccount>::try_from(&dest_token_account_info)?;
        let dest_token_amount_before = dest_token_account.amount;

        let swap_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new(ctx.remaining_accounts[12].key(), false),
            AccountMeta::new(ctx.remaining_accounts[13].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[14].key(), false),
            AccountMeta::new(ctx.remaining_accounts[15].key(), false),
            AccountMeta::new(ctx.remaining_accounts[16].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[17].key(), true),
        ];

        const SWAP_IX: u8 = 9;
        let mut swap_data = vec![];
        swap_data.append(&mut SWAP_IX.try_to_vec()?);
        swap_data.append(&mut input_struct.swap_in_amount.try_to_vec()?);
        swap_data.append(&mut input_struct.swap_min_out_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: swap_accounts,
            data: swap_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        dest_token_account.reload()?;

        let dest_token_amount_after = dest_token_account.amount;
        let swap_out_amount = dest_token_amount_after - dest_token_amount_before;

        msg!("out_amount: {}", swap_out_amount.to_string());

        // Return Result
        let output_struct = SwapOutputWrapper {
            swap_out_amount,
            ..Default::default()
        };

        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn add_liquidity<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = AddLiquidityInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        // Use remaining accounts
        let lp_token_account_info = ctx.remaining_accounts[11].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        let price_pool_direction = match input_struct.pool_direction {
            // Obverse
            0 => PoolDirectionP::Obverse,
            // Reverse
            1 => PoolDirectionP::Reverse,
            _ => return Err(ErrorCode::UnsupportedPoolDirection.into()),
        };

        // Get the data from payload queue
        let token_in_amount = input_struct.token_in_amount;

        // Get Price
        let raydium_lp_price_wrapper = get_raydium_lp_price(
            ctx.remaining_accounts[3].clone(),
            ctx.remaining_accounts[1].clone(),
            ctx.remaining_accounts[6].clone(),
            ctx.remaining_accounts[7].clone(),
            ctx.remaining_accounts[5].clone(),
            token_in_amount,
            price_pool_direction,
        );

        let (add_lp_coin_amount, add_lp_pc_amount, add_lp_fixed_coin) =
            match input_struct.pool_direction {
                // Obverse
                0 => (
                    (token_in_amount as f64 * raydium_lp_price_wrapper.coin_to_pc_price) as u64,
                    token_in_amount,
                    0 as u64,
                ),
                // Reverse
                1 => (
                    token_in_amount,
                    (token_in_amount as f64 * raydium_lp_price_wrapper.pc_to_coin_price) as u64,
                    1 as u64,
                ),
                _ => return Err(ErrorCode::UnsupportedPoolDirection.into()),
            };

        let add_lp_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false), // user_coin_token_account
            AccountMeta::new(ctx.remaining_accounts[10].key(), false), // user_pc_token_account
            AccountMeta::new(ctx.remaining_accounts[11].key(), false), // user_lp_token_account
            AccountMeta::new_readonly(ctx.remaining_accounts[12].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[13].key(), false),
        ];

        const ADD_LP_IX: u8 = 3;
        let mut add_lp_data = vec![];
        add_lp_data.append(&mut ADD_LP_IX.try_to_vec()?);
        add_lp_data.append(&mut add_lp_coin_amount.try_to_vec()?);
        add_lp_data.append(&mut add_lp_pc_amount.try_to_vec()?);
        add_lp_data.append(&mut add_lp_fixed_coin.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: add_lp_accounts,
            data: add_lp_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        lp_token_account.reload()?;
        let lp_token_amount_after = lp_token_account.amount;
        let lp_amount = lp_token_amount_after - lp_token_amount_before;

        msg!("lp_amount: {}", lp_amount.to_string());

        // Wrap Output
        let output_struct = AddLiquidityOutputWrapper {
            lp_amount,
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

        // Use remaining accounts
        let lp_token_account_info = ctx.remaining_accounts[15].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        let coin_token_account_info = ctx.remaining_accounts[16].clone();
        let mut coin_token_account = Account::<TokenAccount>::try_from(&coin_token_account_info)?;
        let coin_token_amount_before = coin_token_account.amount;

        let pc_token_account_info = ctx.remaining_accounts[17].clone();
        let mut pc_token_account = Account::<TokenAccount>::try_from(&pc_token_account_info)?;
        let pc_token_amount_before = pc_token_account.amount;

        let remove_lp_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new(ctx.remaining_accounts[12].key(), false),
            AccountMeta::new(ctx.remaining_accounts[13].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[14].key(), false),
            AccountMeta::new(ctx.remaining_accounts[15].key(), false),
            AccountMeta::new(ctx.remaining_accounts[16].key(), false),
            AccountMeta::new(ctx.remaining_accounts[17].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[18].key(), true),
            AccountMeta::new(ctx.remaining_accounts[19].key(), false),
            AccountMeta::new(ctx.remaining_accounts[20].key(), false),
            AccountMeta::new(ctx.remaining_accounts[21].key(), false),
        ];

        const REMOVE_LP_IX: u8 = 4;
        let mut remove_lp_data = vec![];
        remove_lp_data.append(&mut REMOVE_LP_IX.try_to_vec()?);
        remove_lp_data.append(&mut input_struct.lp_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: remove_lp_accounts,
            data: remove_lp_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        lp_token_account.reload()?;
        let lp_token_amount_after = lp_token_account.amount;
        let lp_amount = lp_token_amount_before - lp_token_amount_after;

        coin_token_account.reload()?;
        let coin_token_amount_after = coin_token_account.amount;
        let token_a_amount = coin_token_amount_after - coin_token_amount_before;

        pc_token_account.reload()?;
        let pc_token_amount_after = pc_token_account.amount;
        let token_b_amount = pc_token_amount_after - pc_token_amount_before;

        // Wrap Output
        let output_struct = RemoveLiquidityOutputWrapper {
            token_a_amount,
            token_b_amount,
            lp_amount,
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

        let lp_token_account_info = ctx.remaining_accounts[4].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        let stake_ix = match input_struct.version {
            3 => 10,
            5 => 11,
            _ => {
                return Err(ErrorCode::UnsupportedVersion.into());
            }
        };

        invoke_stake(&ctx, stake_ix, input_struct.lp_amount)?;

        lp_token_account.reload()?;
        let lp_token_amount_after = lp_token_account.amount;
        let share_amount = lp_token_amount_before - lp_token_amount_after;

        // Wrap Output
        let output_struct = StakeOutputWrapper {
            share_amount,
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

        let lp_token_account_info = ctx.remaining_accounts[4].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;

        let unstake_ix = match input_struct.version {
            3 => 11,
            5 => 12,
            _ => {
                return Err(ErrorCode::UnsupportedVersion.into());
            }
        };

        invoke_stake(&ctx, unstake_ix, input_struct.share_amount)?;

        lp_token_account.reload()?;
        let lp_token_amount_after = lp_token_account.amount;
        let lp_amount = lp_token_amount_after - lp_token_amount_before;

        // Wrap Output
        let output_struct = UnstakeOutputWrapper {
            lp_amount,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn harvest(ctx: Context<Action>, input: Vec<u8>) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = HarvestInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let reward_token_a_account_info = ctx.remaining_accounts[6].clone();
        let mut reward_token_a_account =
            Account::<TokenAccount>::try_from(&reward_token_a_account_info)?;
        let reward_token_a_amount_before = reward_token_a_account.amount;

        let (unstake_ix, reward_token_b_before) = match input_struct.version {
            3 => (11, 0),
            5 => {
                let reward_token_b_account_info = ctx.remaining_accounts[10].clone();
                let reward_token_b_account =
                    Account::<TokenAccount>::try_from(&reward_token_b_account_info)?;
                let reward_token_b_amount_before = reward_token_b_account.amount;
                (12, reward_token_b_amount_before)
            }
            _ => {
                return Err(ErrorCode::UnsupportedVersion.into());
            }
        };

        invoke_stake(&ctx, unstake_ix, 0)?;

        reward_token_a_account.reload()?;
        let reward_token_a_amount_after = reward_token_a_account.amount;
        let reward_a_amount = reward_token_a_amount_after - reward_token_a_amount_before;

        let reward_token_b_after = match input_struct.version {
            3 => 0,
            5 => {
                let reward_token_b_account_info = ctx.remaining_accounts[10].clone();
                let reward_token_b_account =
                    Account::<TokenAccount>::try_from(&reward_token_b_account_info)?;
                let reward_token_b_amount_after = reward_token_b_account.amount;
                reward_token_b_amount_after
            }
            _ => {
                return Err(ErrorCode::UnsupportedVersion.into());
            }
        };
        let reward_b_amount = reward_token_b_after - reward_token_b_before;

        // Wrap Output
        let output_struct = HarvestOutputWrapper {
            reward_a_amount,
            reward_b_amount,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }
}

pub fn invoke_stake(ctx: &Context<Action>, ix: u8, amount: u64) -> Result<()> {
    let mut stake_accounts = vec![
        AccountMeta::new(ctx.remaining_accounts[0].key(), false),
        AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
        AccountMeta::new(ctx.remaining_accounts[2].key(), false),
        AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), true),
        AccountMeta::new(ctx.remaining_accounts[4].key(), false),
        AccountMeta::new(ctx.remaining_accounts[5].key(), false),
        AccountMeta::new(ctx.remaining_accounts[6].key(), false),
        AccountMeta::new(ctx.remaining_accounts[7].key(), false),
        AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
        AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
    ];

    // Check if the remaining_accounts are left (nornally 10 accounts)
    if ctx.remaining_accounts.len() > 10 {
        stake_accounts.push(AccountMeta::new(ctx.remaining_accounts[10].key(), false));
        stake_accounts.push(AccountMeta::new(ctx.remaining_accounts[11].key(), false));
    }

    let mut stake_data = vec![];
    stake_data.append(&mut ix.try_to_vec()?);
    stake_data.append(&mut amount.try_to_vec()?);

    let stake_ix = Instruction {
        program_id: ctx.accounts.base_program_id.key(),
        accounts: stake_accounts,
        data: stake_data,
    };

    invoke(&stake_ix, ctx.remaining_accounts)?;

    Ok(())
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
    pub gateway_state_info: AccountInfo<'info>,
    /// CHECK: Safe
    pub base_program_id: AccountInfo<'info>,
}

// InputWrapper
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct SwapInputWrapper {
    pub swap_in_amount: u64,
    pub swap_min_out_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct AddLiquidityInputWrapper {
    pub token_in_amount: u64,
    pub pool_direction: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct RemoveLiquidityInputWrapper {
    pub lp_amount: u64,
    pub pool_direction: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeInputWrapper {
    pub lp_amount: u64,
    pub version: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnstakeInputWrapper {
    pub share_amount: u64,
    pub version: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct HarvestInputWrapper {
    pub version: u8,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct SwapOutputWrapper {
    pub swap_out_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

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
    pub lp_amount: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct StakeOutputWrapper {
    pub share_amount: u64,
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
    pub reward_a_amount: u64,
    pub reward_b_amount: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// Make a tuple for being accessed by index rather than field name
pub type SwapOutputTuple = (u64, u64, u64, u64);
pub type AddLiquidityOutputTuple = (u64, u64, u64, u64);
pub type RemoveLiquidityOutputTuple = (u64, u64, u64, u64);
pub type StakeOutputTuple = (u64, u64, u64, u64);
pub type UnstakeOutputTuple = (u64, u64, u64, u64);
pub type HarvestOutputTuple = (u64, u64, u64, u64);

impl From<SwapOutputWrapper> for SwapOutputTuple {
    fn from(result: SwapOutputWrapper) -> SwapOutputTuple {
        let SwapOutputWrapper {
            swap_out_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (swap_out_amount, dummy_2, dummy_3, dummy_4)
    }
}

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
            lp_amount,
            dummy_4,
        } = result;
        (token_a_amount, token_b_amount, lp_amount, dummy_4)
    }
}

impl From<StakeOutputWrapper> for StakeOutputTuple {
    fn from(result: StakeOutputWrapper) -> StakeOutputTuple {
        let StakeOutputWrapper {
            share_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (share_amount, dummy_2, dummy_3, dummy_4)
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
            reward_a_amount,
            reward_b_amount,
            dummy_3,
            dummy_4,
        } = result;
        (reward_a_amount, reward_b_amount, dummy_3, dummy_4)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unsupported PoolDirection")]
    UnsupportedPoolDirection,
    #[msg("Unsupported Action Version")]
    UnsupportedVersion,
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];

    sighash.copy_from_slice(&hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}
