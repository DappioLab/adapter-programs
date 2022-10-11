use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

declare_id!("ADPTax5HwQ2ZWVLmceCek8UrqMhwCy5q3SHwi8W71Kv2");

#[program]
pub mod adapter_francium {

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

        Ok(())
    }

    pub fn unsupply<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let withdraw_reserve_id: u8 = 5;
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let reserved_amount = gateway_state.payload_queue[current_index as usize];

        let unsupply_amount = reserved_amount.clone().to_le_bytes().to_vec();

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
        unsupply_ix_data.append(&mut unsupply_amount.clone());

        let unsupply_ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unsupply_accounts,
            data: unsupply_ix_data,
        };

        invoke(&unsupply_ix, ctx.remaining_accounts)?;

        Ok(())
    }

    pub fn stake<'a, 'b, 'c, 'info>(ctx: Context<'a, 'b, 'c, 'info, Action<'info>>) -> Result<()> {
        let deposit_reward_id: u8 = 3;
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

        Ok(())
    }

    pub fn unstake<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let withdraw_reward_id: u8 = 4;
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let unsupply_amount = gateway_state.payload_queue[current_index as usize];

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

        let result = UnstakeResultWrapper {
            lp_amount: unsupply_amount,
        };
        let mut buffer: Vec<u8> = Vec::new();
        result.serialize(&mut buffer).unwrap();

        anchor_lang::solana_program::program::set_return_data(&buffer);

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
