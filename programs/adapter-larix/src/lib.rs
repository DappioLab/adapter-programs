use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

declare_id!("ADPTLQQ1Bwgybb2qge7QKSW7woDrhEjcLWG642qP2X4");

#[program]
pub mod adapter_larix {
    use std::borrow::{BorrowMut};

    use super::*;
    pub fn supply<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Action<'info>>) -> Result<()> {
        let deposit_reserve_id: u8 = 4;

        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let supply_amount = gateway_state.payload_queue[current_index as usize];
        let supply_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
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

        Ok(())
    }

    pub fn unsupply<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let withdraw_reserve_id: u8 = 5;
        let unsupply_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
        ];

        // [255, 255, 255, 255, 255, 255, 255, 255] is u64_max le byte array
        // Larix program will use all the balance in the token account
        let unsupply_data = vec![withdraw_reserve_id, 255, 255, 255, 255, 255, 255, 255, 255];

        let unsupply_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unsupply_accounts,
            data: unsupply_data,
        };

        invoke(&unsupply_ix, ctx.remaining_accounts)?;

        Ok(())
    }

    pub fn borrow<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Action<'info>>) -> Result<()> {
        let borrow_ix_id: u8 = 10;
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;
        // Get the data from payload queue
        let borrow_amount = gateway_state.payload_queue[current_index as usize];

        let borrow_ix_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
        ];

        let mut borrow_ix_data = vec![borrow_ix_id];
        borrow_ix_data.append(borrow_amount.to_le_bytes().to_vec().borrow_mut());

        let borrow_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: borrow_ix_accounts,
            data: borrow_ix_data,
        };

        invoke(&borrow_ix, ctx.remaining_accounts)?;

        Ok(())
    }

    pub fn repay<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Action<'info>>) -> Result<()> {
        let repay_ix_id: u8 = 11;
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let mut repay_amount = gateway_state.payload_queue[current_index as usize];

        // This magic number is the max value of Number in javascript
        // in this case it's a tag for reaping max amount
        const MAGIC_NUMBER: u64 = 9007199254740991;

        if repay_amount.eq(&MAGIC_NUMBER) {
            repay_amount = u64::MAX;
        }

        let repay_ix_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
        ];

        let mut repay_ix_data = vec![repay_ix_id];
        repay_ix_data.append(repay_amount.to_le_bytes().to_vec().borrow_mut());

        let repay_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: repay_ix_accounts,
            data: repay_ix_data,
        };

        invoke(&repay_ix, ctx.remaining_accounts)?;

        Ok(())
    }

    pub fn stake<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Action<'info>>) -> Result<()> {
        let stake_id: u8 = 18;
        let stake_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), true),
            AccountMeta::new(ctx.remaining_accounts[5].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
        ];

        // [255, 255, 255, 255, 255, 255, 255, 255] is u64_max le byte array
        // Larix program will use all the balance in the token account
        let stake_data: Vec<u8> = vec![stake_id, 255, 255, 255, 255, 255, 255, 255, 255];

        let stake_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: stake_accounts,
            data: stake_data,
        };

        invoke(&stake_ix, ctx.remaining_accounts)?;

        Ok(())
    }

    pub fn unstake<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let unstake_id: u8 = 19;
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let unstake_amount = gateway_state.payload_queue[current_index as usize];

        let unstake_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
        ];

        let mut unstake_data = unstake_id.to_le_bytes().to_vec();
        unstake_data.append(unstake_amount.to_le_bytes().to_vec().clone().borrow_mut());

        let unstake_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unstake_accounts,
            data: unstake_data,
        };

        invoke(&unstake_ix, ctx.remaining_accounts)?;

        // Return Result
        let result = UnstakeResultWrapper {
            lp_amount: unstake_amount,
        };
        let mut buffer: Vec<u8> = Vec::new();
        result.serialize(&mut buffer).unwrap();

        anchor_lang::solana_program::program::set_return_data(&buffer);

        Ok(())
    }

    pub fn collateralize<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let collateralize_id: u8 = 8;
        let mut deposit_to_obligation_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
        ];
        if ctx.remaining_accounts.len() > 8 {
            // The last account that's not optional is at index 7
            // Optional accounts starts at index 8
            for index in 8..ctx.remaining_accounts.len() {
                deposit_to_obligation_accounts.push(AccountMeta::new_readonly(
                    ctx.remaining_accounts[index].key(),
                    false,
                ))
            }
        }

        // [255, 255, 255, 255, 255, 255, 255, 255] is u64_max le byte array
        // Larix program will use all the balance in the token account
        let deposit_data: Vec<u8> = vec![collateralize_id, 255, 255, 255, 255, 255, 255, 255, 255];

        let deposit_to_obligation_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key().clone(),
            accounts: deposit_to_obligation_accounts,
            data: deposit_data,
        };

        invoke(&deposit_to_obligation_ix, ctx.remaining_accounts)?;

        Ok(())
    }

    pub fn uncollateralize<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let withdraw_obligation: u8 = 9;
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;
        // Get the data from payload queue
        let reserved_amount = gateway_state.payload_queue[current_index as usize];

        let unsupply_amount = reserved_amount.clone().to_le_bytes().to_vec();
        let withdraw_from_obligation_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
        ];

        let mut withdraw_from_obligation_data: Vec<u8> = vec![withdraw_obligation];
        withdraw_from_obligation_data.append(unsupply_amount.clone().borrow_mut());

        let withdraw_from_obligation_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: withdraw_from_obligation_accounts,
            data: withdraw_from_obligation_data,
        };

        invoke(&withdraw_from_obligation_ix, ctx.remaining_accounts)?;

        Ok(())
    }

    pub fn harvest<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let harvest_id: u8 = 20;
        let mut harvest_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
        ];
        if ctx.remaining_accounts.len() > 7 {
            // The last account that's not optional is at index 6
            // Optional accounts starts at index 7
            for index in 7..ctx.remaining_accounts.len() {
                harvest_accounts.push(AccountMeta::new_readonly(
                    ctx.remaining_accounts[index].key(),
                    false,
                ))
            }
        }
        let harvest_data: Vec<u8> = vec![harvest_id];

        let harvest_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key().clone(),
            accounts: harvest_accounts,
            data: harvest_data,
        };

        invoke(&harvest_ix, ctx.remaining_accounts)?;

        Ok(())
    }

    pub fn claim_collateral_reward<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let claim_id: u8 = 21;
        let mut claim_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[3].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
        ];
        if ctx.remaining_accounts.len() > 7 {
            // The last account that's not optional is at index 6
            // Optional accounts starts at index 7
            for index in 7..ctx.remaining_accounts.len() {
                claim_accounts.push(AccountMeta::new_readonly(
                    ctx.remaining_accounts[index].key(),
                    false,
                ))
            }
        }
        let claim_data: Vec<u8> = vec![claim_id];

        let harvest_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key().clone(),
            accounts: claim_accounts,
            data: claim_data,
        };

        invoke(&harvest_ix, ctx.remaining_accounts)?;

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

fn get_gateway_state(gateway_state_info: &AccountInfo) -> GatewayStateWrapper {
    let mut gateway_state_data = &**gateway_state_info.try_borrow_data().unwrap();
    GatewayStateWrapper::deserialize(&mut gateway_state_data).unwrap()
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UnstakeResultWrapper {
    pub lp_amount: u64,
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
    #[msg("Unsupported Action")]
    UnsupportedAction,
}
