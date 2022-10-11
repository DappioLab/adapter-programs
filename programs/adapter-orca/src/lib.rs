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
    use super::*;

    pub fn add_liquidity<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let add_lp_ix: u8 = 4;
        // Use remaining accounts
        let lp_token_account_info = ctx.remaining_accounts[7].clone();
        let mut lp_token_account = Account::<TokenAccount>::try_from(&lp_token_account_info)?;
        let lp_token_amount_before = lp_token_account.amount;
        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let pool_tokenin_amount = gateway_state.payload_queue[current_index as usize];

        let add_lp_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), true),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
        ];

        let minimal_receive: u64 = 0;
        let mut add_lp_data = vec![];
        add_lp_data.append(&mut add_lp_ix.to_le_bytes().to_vec());
        add_lp_data.append(&mut pool_tokenin_amount.to_le_bytes().to_vec());
        add_lp_data.append(&mut minimal_receive.to_le_bytes().to_vec());

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

        // Return Result
        let result = AddLiquidityResultWrapper { lp_amount };

        let mut buffer: Vec<u8> = Vec::new();
        result.serialize(&mut buffer).unwrap();
        anchor_lang::solana_program::program::set_return_data(&buffer);

        Ok(())
    }

    pub fn remove_liquidity<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;
        let action = gateway_state.action_queue[current_index as usize];

        // Get the data from payload queue
        let lp_amount = gateway_state.payload_queue[current_index as usize];

        msg!("lp_amount: {}", lp_amount.to_string());

        let ix = match action {
            // RemoveLiquidity
            2 => {
                let remove_lp_ix: u8 = 3;
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
                    AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
                ];

                let minimal_receive: u64 = 0;

                let mut remove_lp_data = vec![];
                remove_lp_data.append(&mut remove_lp_ix.to_le_bytes().to_vec());
                remove_lp_data.append(&mut lp_amount.to_le_bytes().to_vec());
                remove_lp_data.append(&mut minimal_receive.to_le_bytes().to_vec());
                remove_lp_data.append(&mut minimal_receive.to_le_bytes().to_vec());

                Instruction {
                    program_id: ctx.accounts.base_program_id.key(),
                    accounts: remove_lp_accounts,
                    data: remove_lp_data,
                }
            }
            // RemoveLiquiditySingle
            3 => {
                let remove_lp_ix: u8 = 5;
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
                    AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
                ];
                let minimal_receive: u64 = 1;

                let mut remove_lp_data = vec![];
                remove_lp_data.append(&mut remove_lp_ix.to_le_bytes().to_vec());
                remove_lp_data.append(&mut minimal_receive.to_le_bytes().to_vec());
                remove_lp_data.append(&mut lp_amount.to_le_bytes().to_vec());

                Instruction {
                    program_id: ctx.accounts.base_program_id.key(),
                    accounts: remove_lp_accounts,
                    data: remove_lp_data,
                }
            }
            _ => {
                return Err(ErrorCode::UnsupportedAction.into());
            }
        };

        invoke(&ix, ctx.remaining_accounts)?;

        Ok(())
    }

    pub fn stake<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Action<'info>>) -> Result<()> {
        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let lp_amount = gateway_state.payload_queue[current_index as usize];

        let stake_ix: u8 = 2;
        let stake_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), true),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
        ];

        let mut stake_data = vec![];
        stake_data.append(&mut stake_ix.to_le_bytes().to_vec());
        stake_data.append(&mut lp_amount.to_le_bytes().to_vec());

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: stake_accounts,
            data: stake_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        Ok(())
    }

    pub fn unstake<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let unstake_ix: u8 = 3;
        let lp_amount = gateway_state.payload_queue[current_index as usize];

        msg!("lp_amount: {}", lp_amount.to_string());

        let unstake_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), true),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
        ];

        let mut unstake_data = vec![];
        unstake_data.append(&mut unstake_ix.to_le_bytes().to_vec());
        unstake_data.append(&mut lp_amount.to_le_bytes().to_vec());

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unstake_accounts,
            data: unstake_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        let result = UnstakeResultWrapper {
            lp_amount,
        };
        let mut buffer: Vec<u8> = Vec::new();
        result.serialize(&mut buffer).unwrap();

        anchor_lang::solana_program::program::set_return_data(&buffer);

        Ok(())
    }

    pub fn harvest<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        // Harvest
        let harvest_ix: u8 = 4;
        let harvest_accounts = vec![
            AccountMeta::new_readonly(ctx.remaining_accounts[0].key(), true),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
        ];

        let mut harvest_data = vec![];
        harvest_data.push(harvest_ix);
        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: harvest_accounts,
            data: harvest_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AddLiquidityResultWrapper {
    pub lp_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UnstakeResultWrapper {
    pub lp_amount: u64,
}

fn get_gateway_state(gateway_state_info: &AccountInfo) -> GatewayStateWrapper {
    let mut gateway_state_data = &**gateway_state_info.try_borrow_data().unwrap();
    GatewayStateWrapper::deserialize(&mut gateway_state_data).unwrap()
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct GatewayStateWrapper {
    pub discriminator: u64,
    pub user_key: Pubkey,
    pub random_seed: u64,
    pub version: u8,
    pub current_index: u8, // Start from 0
    pub queue_size: u8,

    // Queues
    pub protocol_queue: [u8; 8],
    pub action_queue: [u8; 8],
    pub version_queue: [u8; 8],
    pub payload_queue: [u64; 8],

    // Extra metadata
    pub swap_min_out_amount: u64,
    pub pool_direction: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unsupported PoolDirection")]
    UnsupportedPoolDirection,
    #[msg("Unsupported Action")]
    UnsupportedAction,
}
