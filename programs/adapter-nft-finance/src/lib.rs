use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};

declare_id!("ADPTyBr92sBCE1hdYBRvXbMpF4hKs17xyDjFPxopcsrh");

#[program]
pub mod adapter_nft_finance {
    use super::*;

    pub fn lock_nft<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let sighash_arr = sighash("global", "stake");

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
        lock_nft_data.append(&mut sighash_arr.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: lock_nft_accounts,
            data: lock_nft_data,
        };

        invoke(
            &ix,
            ctx.remaining_accounts,
        )?;

        Ok(())
    }

    pub fn unlock_nft<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let sighash_arr = sighash("global", "unstake");

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
        unlock_nft_data.append(&mut sighash_arr.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unlock_nft_accounts,
            data: unlock_nft_data,
        };

        invoke(
            &ix,
            ctx.remaining_accounts,
        )?;

        Ok(())
    }

    pub fn stake_proof<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let sighash_arr = sighash("global", "deposit");

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let stake_proof_amount = gateway_state.payload_queue[current_index as usize];

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
        stake_proof_data.append(&mut sighash_arr.try_to_vec()?);
        stake_proof_data.append(&mut stake_proof_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: stake_proof_accounts,
            data: stake_proof_data,
        };

        invoke(
            &ix,
            ctx.remaining_accounts,
        )?;

        Ok(())
    }

    pub fn unstake_proof<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let sighash_arr = sighash("global", "withdraw");

        // Deserialize gateway_state
        let gateway_state = get_gateway_state(&ctx.accounts.gateway_state_info);
        let current_index = gateway_state.current_index;

        // Get the data from payload queue
        let unstake_proof_amount = gateway_state.payload_queue[current_index as usize];

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
        unstake_proof_data.append(&mut sighash_arr.try_to_vec()?);
        unstake_proof_data.append(&mut unstake_proof_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: unstake_proof_accounts,
            data: unstake_proof_data,
        };

        invoke(
            &ix,
            ctx.remaining_accounts,
        )?;

        Ok(())
    }

    pub fn claim<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
    ) -> Result<()> {
        let sighash_arr = sighash("global", "claim");

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
        claim_data.append(&mut sighash_arr.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: claim_accounts,
            data: claim_data,
        };

        invoke(
            &ix,
            ctx.remaining_accounts,
        )?;

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
pub struct AddLiquidityResultWrapper {
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

