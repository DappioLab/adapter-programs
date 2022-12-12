use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

declare_id!("ADPT9nhC1asRcEB13FKymLTatqWGCuZHDznGgnakWKxW");

#[program]
pub mod adapter_tulip {
    use super::*;

    pub fn deposit<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = DepositInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let (ix, mut token_account_and_balance) = match input_struct.farm_type_0 {
            0 => {
                // reference: https://github.com/sol-farm/tulipv2-sdk/blob/main/vaults/src/instructions/mod.rs#L20
                let token_account_and_balance =
                    load_token_account_and_balance(ctx.remaining_accounts, 7);
                let mut ix_data = sighash("global", "issue_shares").try_to_vec()?;
                ix_data.append(&mut input_struct.farm_type_0.try_to_vec()?);
                ix_data.append(&mut input_struct.farm_type_1.try_to_vec()?);
                ix_data.append(&mut input_struct.lp_or_token_a_amount.try_to_vec()?);

                let accounts = load_remaining_accounts(
                    ctx.remaining_accounts,
                    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                );
                (
                    Instruction {
                        program_id: ctx.accounts.base_program_id.key(),
                        accounts,
                        data: ix_data,
                    },
                    token_account_and_balance,
                )
            }
            2 => {
                // reference: https://github.com/sol-farm/tulipv2-sdk/blob/main/vaults/src/instructions/orca.rs#L255
                let token_account_and_balance =
                    load_token_account_and_balance(ctx.remaining_accounts, 7);
                let mut ix_data = sighash("global", "orca_add_liq_issue_shares").try_to_vec()?;
                ix_data.append(&mut input_struct.lp_or_token_a_amount.try_to_vec()?);
                ix_data.append(&mut input_struct.token_b_amount.try_to_vec()?);
                ix_data.append(&mut input_struct.farm_type_0.try_to_vec()?);
                ix_data.append(&mut input_struct.farm_type_1.try_to_vec()?);

                let accounts = load_remaining_accounts(
                    ctx.remaining_accounts,
                    vec![
                        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
                    ],
                );
                (
                    Instruction {
                        program_id: ctx.accounts.base_program_id.key(),
                        accounts,
                        data: ix_data,
                    },
                    token_account_and_balance,
                )
            }
            _ => return Err(ErrorCode::UnsupportedVaultProtocol.into()),
        };

        invoke(&ix, &ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = DepositOutputWrapper {
            share_amount: token_account_and_balance.get_balance_change(),
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    /// withdraws `amount` of shares from the deposit tracking account into the `receiving_shares_account`.
    /// these withdrawn shares still accrue rewards, the rewards accrued are no longer tracked by the deposit
    /// tracking account
    pub fn withdraw<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = WithdrawInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        let (lp_or_token_a_amount, token_b_amount) = match input_struct.farm_type_0 {
            0 => {
                let mut token_account_and_balance =
                    load_token_account_and_balance(ctx.remaining_accounts, 13);

                // reference: https://github.com/sol-farm/tulipv2-sdk/blob/main/vaults/src/instructions/deposit_tracking.rs#L38
                let mut withdraw_deposit_tracking_data =
                    sighash("global", "withdraw_deposit_tracking").try_to_vec()?;
                withdraw_deposit_tracking_data.append(&mut input_struct.share_amount.try_to_vec()?);
                withdraw_deposit_tracking_data.append(&mut input_struct.farm_type_0.try_to_vec()?);
                withdraw_deposit_tracking_data.append(&mut input_struct.farm_type_1.try_to_vec()?);

                let withdraw_deposit_tracking_index_array = vec![0, 15, 16, 20, 21, 22, 12, 14, 1];
                let withdraw_deposit_tracking_accounts = load_remaining_accounts(
                    ctx.remaining_accounts,
                    withdraw_deposit_tracking_index_array.clone(),
                );

                let withdraw_deposit_tracking_ix = Instruction {
                    program_id: ctx.accounts.base_program_id.key(),
                    accounts: withdraw_deposit_tracking_accounts,
                    data: withdraw_deposit_tracking_data,
                };
                let withdraw_deposit_tracking_account_infos = account_info_array(
                    ctx.remaining_accounts,
                    withdraw_deposit_tracking_index_array.clone(),
                );
                invoke(
                    &withdraw_deposit_tracking_ix,
                    &withdraw_deposit_tracking_account_infos
                        [0..withdraw_deposit_tracking_index_array.len()],
                )?;

                // reference: https://github.com/sol-farm/tulipv2-sdk/blob/main/vaults/src/instructions/raydium.rs#L5
                let mut withdraw_raydium_data =
                    sighash("global", "withdraw_raydium_vault").try_to_vec()?;
                withdraw_raydium_data.append(&mut input_struct.share_amount.try_to_vec()?);
                withdraw_raydium_data.append(&mut input_struct.farm_type_0.try_to_vec()?);
                withdraw_raydium_data.append(&mut input_struct.farm_type_1.try_to_vec()?);

                let withdraw_raydium_accounts = load_remaining_accounts(
                    ctx.remaining_accounts,
                    vec![
                        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                    ],
                );

                let withdraw_raydium_ix = Instruction {
                    program_id: ctx.accounts.base_program_id.key(),
                    accounts: withdraw_raydium_accounts,
                    data: withdraw_raydium_data,
                };
                invoke(&withdraw_raydium_ix, &ctx.remaining_accounts[0..20])?;
                (token_account_and_balance.get_balance_change(), 0u64)
            }
            2 => {
                let mut token_a_account_and_balance =
                    load_token_account_and_balance(ctx.remaining_accounts, 4);
                let mut token_b_account_and_balance =
                    load_token_account_and_balance(ctx.remaining_accounts, 5);

                // reference: https://github.com/sol-farm/tulipv2-sdk/blob/main/vaults/src/instructions/deposit_tracking.rs#L38
                let mut withdraw_deposit_tracking_data =
                    sighash("global", "withdraw_deposit_tracking").try_to_vec()?;
                withdraw_deposit_tracking_data.append(&mut input_struct.share_amount.try_to_vec()?);
                withdraw_deposit_tracking_data.append(&mut input_struct.farm_type_0.try_to_vec()?);
                withdraw_deposit_tracking_data.append(&mut input_struct.farm_type_1.try_to_vec()?);

                let withdraw_deposit_tracking_index_array = vec![6, 3, 27, 0, 1, 2, 9, 25, 7];
                let withdraw_deposit_tracking_accounts = load_remaining_accounts(
                    ctx.remaining_accounts,
                    withdraw_deposit_tracking_index_array.clone(),
                );

                let withdraw_deposit_tracking_ix = Instruction {
                    program_id: ctx.accounts.base_program_id.key(),
                    accounts: withdraw_deposit_tracking_accounts,
                    data: withdraw_deposit_tracking_data,
                };
                let withdraw_deposit_tracking_account_infos = account_info_array(
                    ctx.remaining_accounts,
                    withdraw_deposit_tracking_index_array.clone(),
                );
                invoke(
                    &withdraw_deposit_tracking_ix,
                    &withdraw_deposit_tracking_account_infos
                        [0..withdraw_deposit_tracking_index_array.len()],
                )?;

                let mut is_double_dip = false;
                if ctx.remaining_accounts.len() == 33 {
                    // withdraw vault
                    // reference: https://github.com/sol-farm/tulipv2-sdk/blob/main/vaults/src/instructions/orca.rs#L5
                    let mut withdraw_orca_vault_data =
                        sighash("global", "withdraw_orca_vault").try_to_vec()?;
                    withdraw_orca_vault_data.append(&mut false.try_to_vec()?);
                    withdraw_orca_vault_data.append(&mut input_struct.share_amount.try_to_vec()?);
                    withdraw_orca_vault_data.append(&mut 0u8.try_to_vec()?);

                    let withdraw_orca_vault_accounts = load_remaining_accounts(
                        ctx.remaining_accounts,
                        vec![
                            6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                            25, 26, 27, 28, 29, 30, 31, 32,
                        ],
                    );

                    let withdraw_orca_vault_ix = Instruction {
                        program_id: ctx.accounts.base_program_id.key(),
                        accounts: withdraw_orca_vault_accounts,
                        data: withdraw_orca_vault_data,
                    };
                    invoke(&withdraw_orca_vault_ix, &ctx.remaining_accounts[6..33])?;
                } else {
                    is_double_dip = true;
                    // withdraw dd vault (two stage)
                    // reference: https://github.com/sol-farm/tulipv2-sdk/blob/main/vaults/src/instructions/orca.rs#L72
                    let mut withdraw_orca_dd_vault_stage_one_data =
                        sighash("global", "withdraw_orca_vault_dd_stage_one").try_to_vec()?;
                    withdraw_orca_dd_vault_stage_one_data.append(&mut true.try_to_vec()?);
                    withdraw_orca_dd_vault_stage_one_data
                        .append(&mut input_struct.share_amount.try_to_vec()?);
                    withdraw_orca_dd_vault_stage_one_data.append(&mut 0u8.try_to_vec()?);

                    let withdraw_orca_dd_vault_stage_one_accounts = load_remaining_accounts(
                        ctx.remaining_accounts,
                        vec![
                            6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
                        ],
                    );

                    let withdraw_orca_dd_vault_stage_one_ix = Instruction {
                        program_id: ctx.accounts.base_program_id.key(),
                        accounts: withdraw_orca_dd_vault_stage_one_accounts,
                        data: withdraw_orca_dd_vault_stage_one_data,
                    };
                    invoke(
                        &withdraw_orca_dd_vault_stage_one_ix,
                        &ctx.remaining_accounts[6..35],
                    )?;

                    // reference: https://github.com/sol-farm/tulipv2-sdk/blob/main/vaults/src/instructions/orca.rs#L144
                    let withdraw_orca_dd_vault_stage_two_data =
                        sighash("global", "withdraw_orca_vault_dd_stage_two").try_to_vec()?;

                    let withdraw_orca_dd_vault_stage_two_accounts = load_remaining_accounts(
                        ctx.remaining_accounts,
                        vec![
                            6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
                            24, 25, 26, 27, 28, 29, 30, 31, 32, 33,
                        ],
                    );

                    let withdraw_orca_dd_vault_stage_two_ix = Instruction {
                        program_id: ctx.accounts.base_program_id.key(),
                        accounts: withdraw_orca_dd_vault_stage_two_accounts,
                        data: withdraw_orca_dd_vault_stage_two_data,
                    };
                    invoke(
                        &withdraw_orca_dd_vault_stage_two_ix,
                        &ctx.remaining_accounts[6..34],
                    )?;
                }

                // reference: https://github.com/sol-farm/tulipv2-sdk/blob/main/vaults/src/instructions/orca.rs#L209
                let mut remove_liq_data =
                    sighash("global", "withdraw_orca_vault_remove_liq").try_to_vec()?;
                remove_liq_data.append(&mut is_double_dip.try_to_vec()?);

                let remove_liq_index_array =
                    vec![6, 7, 8, 10, 4, 5, 16, 17, 28, 21, 22, 23, 26, 27, 30, 25];
                let remove_liq_accounts =
                    load_remaining_accounts(ctx.remaining_accounts, remove_liq_index_array.clone());

                let remove_liq_ix = Instruction {
                    program_id: ctx.accounts.base_program_id.key(),
                    accounts: remove_liq_accounts,
                    data: remove_liq_data,
                };
                let remove_liq_account_infos =
                    account_info_array(ctx.remaining_accounts, remove_liq_index_array.clone());
                invoke(
                    &remove_liq_ix,
                    &remove_liq_account_infos[0..remove_liq_index_array.len()],
                )?;

                (
                    token_a_account_and_balance.get_balance_change(),
                    token_b_account_and_balance.get_balance_change(),
                )
            }
            _ => return Err(ErrorCode::UnsupportedVaultProtocol.into()),
        };

        // Wrap Output
        let output_struct = WithdrawOutputWrapper {
            lp_or_token_a_amount,
            token_b_amount,
            ..Default::default()
        };
        let mut output: Vec<u8> = Vec::new();
        output_struct.serialize(&mut output).unwrap();

        anchor_lang::solana_program::program::set_return_data(&output);

        msg!("Output: {:?}", output_struct);

        Ok(())
    }

    pub fn supply<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Action<'info>>,
        input: Vec<u8>,
    ) -> Result<()> {
        // Get Input
        let mut input_bytes = &input[..];
        let input_struct = SupplyInputWrapper::deserialize(&mut input_bytes)?;

        msg!("Input: {:?}", input_struct);

        // Use remaining accounts
        let reserve_token_account_info = ctx.remaining_accounts[1].clone();
        let mut reserve_token_account =
            Account::<TokenAccount>::try_from(&reserve_token_account_info)?;
        let reserve_token_amount_before = reserve_token_account.amount;

        let add_supply_accounts = vec![
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

        let mut add_supply_data = vec![];
        const SUPPLY_IX: u8 = 4; // DepositReserveLiquidity
        add_supply_data.append(&mut SUPPLY_IX.try_to_vec()?);
        add_supply_data.append(&mut input_struct.supply_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: add_supply_accounts,
            data: add_supply_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        reserve_token_account.reload()?;
        let reserve_token_amount_after = reserve_token_account.amount;
        let reserved_amount = reserve_token_amount_after - reserve_token_amount_before;

        // Wrap Output
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
        let supply_token_account_info = ctx.remaining_accounts[1].clone();
        let mut supply_token_account =
            Account::<TokenAccount>::try_from(&supply_token_account_info)?;
        let supply_token_amount_before = supply_token_account.amount;

        let reserved_token_account_info = ctx.remaining_accounts[0].clone();
        let reserved_token_account =
            Account::<TokenAccount>::try_from(&reserved_token_account_info)?;
        let reserved_token_amount = reserved_token_account.amount;

        let unsupply_amount = match reserved_token_amount >= input_struct.reserved_amount {
            true => input_struct.reserved_amount,
            false => reserved_token_amount,
        };

        let remove_supply_accounts = vec![
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

        let mut remove_supply_data = vec![];
        const UNSUPPLY_IX: u8 = 5; // RedeemReserveCollateral
        remove_supply_data.append(&mut UNSUPPLY_IX.try_to_vec()?);
        remove_supply_data.append(&mut unsupply_amount.try_to_vec()?);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: remove_supply_accounts,
            data: remove_supply_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        supply_token_account.reload()?;

        let supply_token_amount_after = supply_token_account.amount;
        let unsupply_amount = supply_token_amount_after - supply_token_amount_before;

        // Wrap Output
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
}

#[derive(Accounts)]
pub struct Action<'info> {
    // TODO: Add constraints
    pub gateway_authority: Signer<'info>,
    /// CHECK: Safe
    pub base_program_id: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct DepositInputWrapper {
    pub lp_or_token_a_amount: u64,
    pub token_b_amount: u64,
    pub farm_type_0: u64,
    pub farm_type_1: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct WithdrawInputWrapper {
    pub share_amount: u64,
    pub farm_type_0: u64,
    pub farm_type_1: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct SupplyInputWrapper {
    pub supply_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct UnsupplyInputWrapper {
    pub reserved_amount: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct DepositOutputWrapper {
    pub share_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct WithdrawOutputWrapper {
    pub lp_or_token_a_amount: u64,
    pub token_b_amount: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
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

pub type DepositOutputTuple = (u64, u64, u64, u64);
pub type WithdrawOutputTuple = (u64, u64, u64, u64);
pub type SupplyOutputTuple = (u64, u64, u64, u64);
pub type UnsupplyOutputTuple = (u64, u64, u64, u64);

impl From<DepositOutputWrapper> for DepositOutputTuple {
    fn from(result: DepositOutputWrapper) -> DepositOutputTuple {
        let DepositOutputWrapper {
            share_amount,
            dummy_2,
            dummy_3,
            dummy_4,
        } = result;
        (share_amount, dummy_2, dummy_3, dummy_4)
    }
}

impl From<WithdrawOutputWrapper> for WithdrawOutputTuple {
    fn from(result: WithdrawOutputWrapper) -> WithdrawOutputTuple {
        let WithdrawOutputWrapper {
            lp_or_token_a_amount,
            token_b_amount,
            dummy_3,
            dummy_4,
        } = result;
        (lp_or_token_a_amount, token_b_amount, dummy_3, dummy_4)
    }
}

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

#[error_code]
pub enum ErrorCode {
    #[msg("Unsupported Vault Protocol")]
    UnsupportedVaultProtocol,
    #[msg("Index might out of bound, currently only support 30 addresses")]
    IndexOutOfBound,
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];

    sighash.copy_from_slice(&hash(preimage.as_bytes()).to_bytes()[..8]);
    sighash
}
pub fn load_token_account_and_balance<'info>(
    remaining_accounts: &[AccountInfo<'info>],
    account_index: usize,
) -> TokenAccountAndBalance<'info> {
    let token_account_info = &remaining_accounts[account_index];
    let token_account = Account::<TokenAccount>::try_from(token_account_info).unwrap();
    let balance_before = token_account.amount.clone();
    return TokenAccountAndBalance {
        token_account,
        balance_before,
    };
}

pub struct TokenAccountAndBalance<'info> {
    token_account: Account<'info, TokenAccount>,
    balance_before: u64,
}

impl<'info> TokenAccountAndBalance<'info> {
    pub fn get_balance_change(&mut self) -> u64 {
        self.token_account.reload().unwrap();
        let balance_before = self.balance_before;
        let balance_after = self.token_account.amount;
        if balance_after > balance_before {
            balance_after.checked_sub(balance_before).unwrap()
        } else if balance_after == balance_before {
            0_u64
        } else {
            balance_before.checked_sub(balance_after).unwrap()
        }
    }
}

pub fn load_remaining_accounts<'info>(
    remaining_accounts: &[AccountInfo<'info>],
    index_array: Vec<usize>,
) -> Vec<AccountMeta> {
    let mut accounts: Vec<AccountMeta> = vec![];
    for index in index_array.iter() {
        if remaining_accounts[*index].is_writable {
            accounts.push(AccountMeta::new(
                remaining_accounts[*index].key(),
                remaining_accounts[*index].is_signer,
            ))
        } else {
            accounts.push(AccountMeta::new_readonly(
                remaining_accounts[*index].key(),
                remaining_accounts[*index].is_signer,
            ))
        }
    }
    return accounts;
}

pub fn account_info_array<'info>(
    remaining_accounts: &[AccountInfo<'info>],
    index_array: Vec<usize>,
) -> [AccountInfo<'info>; 30] {
    let mut accounts: [AccountInfo<'info>; 30] = [
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
        remaining_accounts[0].clone(),
    ];
    for i in 0..index_array.len() {
        let index = index_array[i];
        accounts[i] = remaining_accounts[index].clone();
    }
    return accounts;
}
