use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::{Mint, TokenAccount};

declare_id!("ADPTPxbHbEBo9A8E53P2PZnmw3ZYJuwc8ArQQkbJtqhx");

#[program]
pub mod adapter_lido {
    use anchor_lang::solana_program::{account_info::next_account_infos, stake};

    use super::*;

    pub fn deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = DepositInputWrapper::deserialize(&mut input_bytes)?;
        
        msg!("Input: {:?}", input_struct);
     
        let deposit_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), true),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),  
        ];

        let ix = Instruction { program_id: ctx.accounts.base_program_id.key(), accounts: deposit_accounts, data: input };
        
        invoke(&ix, ctx.remaining_accounts)?;

        Ok(())
    }

    pub fn withdraw<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = WithdrawInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);
        
        let withdraw_accounts = if input_struct.instruction == 23 {
            vec![
                AccountMeta::new(ctx.remaining_accounts[0].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), true),
                AccountMeta::new(ctx.remaining_accounts[2].key(), false),
                AccountMeta::new(ctx.remaining_accounts[3].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
                AccountMeta::new(ctx.remaining_accounts[5].key(), false),
                AccountMeta::new(ctx.remaining_accounts[6].key(), true),
                AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),  
                AccountMeta::new(ctx.remaining_accounts[8].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[12].key(), false),
            ]
        } else {
            vec![
                AccountMeta::new(ctx.remaining_accounts[0].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), true),
                AccountMeta::new(ctx.remaining_accounts[2].key(), false),
                AccountMeta::new(ctx.remaining_accounts[3].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
                AccountMeta::new(ctx.remaining_accounts[5].key(), false),
                AccountMeta::new(ctx.remaining_accounts[6].key(), true),
                AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
                AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
            ]
        };

        let withdraw_ix = Instruction { program_id: ctx.accounts.base_program_id.key(), accounts: withdraw_accounts, data: input };
        
        invoke(&withdraw_ix, ctx.remaining_accounts)?;    
        
        let deactivate_ix = stake::instruction::deactivate_stake(&ctx.remaining_accounts[1].key(), &ctx.remaining_accounts[6].key());
        invoke(&deactivate_ix, &vec![ctx.remaining_accounts[1].clone()])?;    

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
pub struct DepositInputWrapper {
    /// The instruction to deposit. Should always be 1.
    pub instruction: u8,
    /// Amount to deposit.
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct WithdrawInputWrapper {
    /// The instuction to withdraw. This is either a 2 (for v1) or 23 (v2).
    pub instruction: u8,
    /// Amount to withdraw.
    pub amount: u64,
    /// Index of the Heaviest Validator. Unused in Lido v1.
    pub validator_index: u32,
}