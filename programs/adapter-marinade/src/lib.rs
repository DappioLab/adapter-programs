use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
};

declare_id!("ADPTAGmW9f42rJ5a25vECpjtzhQCHVDBgqhRCxi5cQbi");

pub mod adapter_lido {
    use super::*;
    use std::borrow::BorrowMut;

    pub fn initiate_deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        let mut deposit_data = sighash("global", "deposit").to_vec();
        let mut input_bytes = &input[..];
        let input_struct = InitiateDepositInputWrapper::deserialize(&mut input_bytes)?;

        let deposit_amount = input_struct.deposit_amount;

        deposit_data.append(deposit_amount.to_le_bytes().to_vec().borrow_mut());

        let deposit_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
        ];

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: deposit_accounts,
            data: deposit_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        msg!("Input: {:?}", input_struct);

        // Deriving keys
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct InitiateDepositInputWrapper {
    pub deposit_amount: u64,
}

pub struct Action<'info> {
    // TODO: Add constraints
    pub gateway_authority: Signer<'info>,
    /// CHECK: Safe
    pub base_program_id: AccountInfo<'info>,
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];

    sighash.copy_from_slice(&hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}
