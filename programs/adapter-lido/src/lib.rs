use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::{Mint, TokenAccount};

declare_id!("ADPTPxbHbEBo9A8E53P2PZnmw3ZYJuwc8ArQQkbJtqhx");

#[program]
pub mod adapter_lido {
    use super::*;

    pub fn deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = DepositInputWrapper::deserialize(&mut input_bytes)?;
        
        msg!("Input: {:?}", input_struct);

        // Deriving keys
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct DepositInputWrapper {
    pub amount: u64,
}
