use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

declare_id!("ADPTCXAFfJFVqcw73B4PWRZQjMNo7Q3Yj4g7p4zTiZnQ");

#[program]
pub mod adapter_solend {
    use super::*;

    pub fn supply<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = SupplyInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        // Use remaining accounts
        let reserved_token_account_info = ctx.remaining_accounts[7].clone();
        let mut reserved_token_account =
            Account::<TokenAccount>::try_from(&reserved_token_account_info)?;
        let reserved_token_amount_before = reserved_token_account.amount;

        let add_supply_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[12].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[13].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[14].key(), false),
        ];

        let mut add_supply_data = vec![];
        const SUPPLY_IX: u8 = 14; // DepositReserveLiquidity and DepositObligationCollateral
        add_supply_data.append(&mut SUPPLY_IX.try_to_vec()?);
        add_supply_data.append(&mut input_struct.supply_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: add_supply_accounts,
            data: add_supply_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        reserved_token_account.reload()?;

        let reserved_token_amount_after = reserved_token_account.amount;
        let reserved_amount = reserved_token_amount_after - reserved_token_amount_before;

        msg!("out_amount: {}", reserved_amount.to_string());

        // Return Result
        let output_struct = SupplyOutputWrapper {
            reserved_amount,
            ..Default::default()
        };

        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn unsupply<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = UnsupplyInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        // Use remaining accounts
        let unsupply_token_account_info = ctx.remaining_accounts[6].clone();
        let mut unsupply_token_account =
            Account::<TokenAccount>::try_from(&unsupply_token_account_info)?;
        let unsupply_token_amount_before = unsupply_token_account.amount;

        let remove_supply_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new(ctx.remaining_accounts[7].key(), false),
            AccountMeta::new(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[10].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[11].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[12].key(), false),
        ];

        let mut remove_supply_data = vec![];
        const UNSUPPLY_IX: u8 = 15; // WithdrawObligationCollateral and RedeemReserveCollateral
        remove_supply_data.append(&mut UNSUPPLY_IX.try_to_vec()?);
        remove_supply_data.append(&mut input_struct.reserved_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: remove_supply_accounts,
            data: remove_supply_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        unsupply_token_account.reload()?;

        let unsupply_token_amount_after = unsupply_token_account.amount;
        let unsupply_amount = unsupply_token_amount_after - unsupply_token_amount_before;

        // Return Result
        let output_struct = UnsupplyOutputWrapper {
            unsupply_amount,
            ..Default::default()
        };

        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn borrow<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = BorrowInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        // Use remaining accounts
        let borrow_token_account_info = ctx.remaining_accounts[1].clone();
        let mut borrow_token_account =
            Account::<TokenAccount>::try_from(&borrow_token_account_info)?;
        let borrow_token_amount_before = borrow_token_account.amount;

        let borrow_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[8].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[9].key(), false),
        ];

        let mut borrow_data = vec![];
        const BORROW_IX: u8 = 10; // BorrowObligationLiquidity
        borrow_data.append(&mut BORROW_IX.try_to_vec()?);
        borrow_data.append(&mut input_struct.borrow_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: borrow_accounts,
            data: borrow_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        borrow_token_account.reload()?;

        let borrow_token_amount_after = borrow_token_account.amount;
        let borrow_amount = borrow_token_amount_after - borrow_token_amount_before;

        // Return Result
        let output_struct = BorrowOutputWrapper {
            borrow_amount,
            ..Default::default()
        };

        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn repay<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = RepayInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        // Use remaining accounts
        let repay_token_account_info = ctx.remaining_accounts[0].clone();
        let mut repay_token_account = Account::<TokenAccount>::try_from(&repay_token_account_info)?;
        let repay_token_amount_before = repay_token_account.amount;

        let repay_accounts = vec![
            AccountMeta::new(ctx.remaining_accounts[0].key(), false),
            AccountMeta::new(ctx.remaining_accounts[1].key(), false),
            AccountMeta::new(ctx.remaining_accounts[2].key(), false),
            AccountMeta::new(ctx.remaining_accounts[3].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[4].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[5].key(), true),
            AccountMeta::new_readonly(ctx.remaining_accounts[6].key(), false),
            AccountMeta::new_readonly(ctx.remaining_accounts[7].key(), false),
        ];

        let mut repay_data = vec![];
        const REPAY_IX: u8 = 11; // RepayObligationLiquidity
        repay_data.append(&mut REPAY_IX.try_to_vec()?);
        repay_data.append(&mut input_struct.repay_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: repay_accounts,
            data: repay_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        repay_token_account.reload()?;

        let repay_token_amount_after = repay_token_account.amount;
        let repay_amount = repay_token_amount_before - repay_token_amount_after;

        // Return Result
        let output_struct = RepayOutputWrapper {
            repay_amount,
            ..Default::default()
        };

        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct SupplyInputWrapper {
    pub supply_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnsupplyInputWrapper {
    pub reserved_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct BorrowInputWrapper {
    pub borrow_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct RepayInputWrapper {
    pub repay_amount: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct SupplyOutputWrapper {
    pub reserved_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnsupplyOutputWrapper {
    pub unsupply_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct BorrowOutputWrapper {
    pub borrow_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct RepayOutputWrapper {
    pub repay_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

pub type SupplyOutputTuple = (u64, u64, u64, u64);
pub type UnsupplyOutputTuple = (u64, u64, u64, u64);
pub type BorrowOutputTuple = (u64, u64, u64, u64);
pub type RepayOutputTuple = (u64, u64, u64, u64);

impl From<SupplyOutputWrapper> for SupplyOutputTuple {
    fn from(result: SupplyOutputWrapper) -> SupplyOutputTuple {
        let SupplyOutputWrapper {
            reserved_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (reserved_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<UnsupplyOutputWrapper> for UnsupplyOutputTuple {
    fn from(result: UnsupplyOutputWrapper) -> UnsupplyOutputTuple {
        let UnsupplyOutputWrapper {
            unsupply_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (unsupply_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<BorrowOutputWrapper> for BorrowOutputTuple {
    fn from(result: BorrowOutputWrapper) -> BorrowOutputTuple {
        let BorrowOutputWrapper {
            borrow_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (borrow_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<RepayOutputWrapper> for RepayOutputTuple {
    fn from(result: RepayOutputWrapper) -> RepayOutputTuple {
        let RepayOutputWrapper {
            repay_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (repay_amount, dummy_2, dummy_3, dummy_4)
    }
}
