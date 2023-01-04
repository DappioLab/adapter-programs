use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    pubkey::Pubkey,
};
use anchor_spl::token::TokenAccount;

declare_id!("ADPT8iF4A7BSUWQ8AsVwmcod2suFzA4bpYpJj7kUWK3E");

const SWAP_CONFIG_SIZE: usize = 30;

#[program]
pub mod adapter_jupiter {
    use super::*;

    const PLATFORM_FEE: u8 = 0;

    pub fn swap(ctx: Context<Action>, input: Vec<u8>) -> Result<()> {
        let discriminator: [u8; 8] = sighash("global", "route");

        // Use remaining accounts
        let mut dest_token_account_and_balance =
            load_token_account_and_balance(ctx.remaining_accounts, 2);

        // Get Input
        let mut input_bytes: &[u8] = &input[..];
        let input_struct = SwapInputWrapper::deserialize(&mut input_bytes)?;
        msg!("Input: {:?}", input_struct);

        let mut last_index: usize = SWAP_CONFIG_SIZE - 1;
        let mut start_index: usize = 1;
        if input_struct.swap_config[0] > 0 && input_struct.swap_config[0] < SWAP_CONFIG_SIZE as u8 {
            last_index = input_struct.swap_config[0] as usize;
        } else if input_struct.swap_config[0] == 0 {
            start_index = 0;
        };

        let swap_accounts = load_remaining_accounts(ctx.remaining_accounts, None);

        let mut swap_data = vec![];
        swap_data.append(&mut discriminator.try_to_vec()?);
        swap_data.extend(
            &mut input_struct.swap_config[start_index..=last_index]
                .iter()
                .cloned(),
        );
        swap_data.append(&mut input_struct.in_amount.try_to_vec()?);
        swap_data.append(&mut input_struct.out_amount.try_to_vec()?);
        swap_data.append(&mut input_struct.slippage_bps.try_to_vec()?);
        swap_data.append(&mut PLATFORM_FEE.try_to_vec()?);
        msg!("swap_data: {:?}", swap_data);

        let ix = Instruction {
            program_id: ctx.accounts.base_program_id.key(),
            accounts: swap_accounts,
            data: swap_data,
        };

        invoke(&ix, ctx.remaining_accounts)?;

        // Wrap Output
        let output_struct = SwapOutputWrapper {
            swap_out_amount: dest_token_account_and_balance.get_balance_change(),
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
pub struct SwapInputWrapper {
    pub in_amount: u64,
    pub out_amount: u64,
    pub slippage_bps: u16,
    pub swap_config: [u8; SWAP_CONFIG_SIZE],
}

// OutputWrapper needs to take up all the space of 32 bytes
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct SwapOutputWrapper {
    pub swap_out_amount: u64,
    pub dummy_2: u64,
    pub dummy_3: u64,
    pub dummy_4: u64,
}

pub type SwapOutputTuple = (u64, u64, u64, u64);

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
        token_accout: token_account,
        balance_before: balance_before,
    };
}

pub struct TokenAccountAndBalance<'info> {
    token_accout: Account<'info, TokenAccount>,
    balance_before: u64,
}

impl<'info> TokenAccountAndBalance<'info> {
    pub fn get_balance_change(&mut self) -> u64 {
        self.token_accout.reload().unwrap();
        let balance_before = self.balance_before;
        let balance_after = self.token_accout.amount;
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
    index_array: Option<Vec<usize>>,
) -> Vec<AccountMeta> {
    let mut accounts: Vec<AccountMeta> = vec![];
    match index_array {
        Some(index_array) => {
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
        }
        None => {
            for account in remaining_accounts.iter() {
                if account.is_writable {
                    accounts.push(AccountMeta::new(account.key(), account.is_signer))
                } else {
                    accounts.push(AccountMeta::new_readonly(account.key(), account.is_signer))
                }
            }
        }
    }

    return accounts;
}

// pub enum SwapLeg {
//     Chain{
//         swap_legs: Vec<SwapLegDeeper>
//     },
//     Split{
//         split_legs: Vec<SplitLeg>
//     },
//     Swap{
//         swap: Swap
//     }
// }

// pub enum SwapLegDeeper {
//     Chain{
//         swap_legs: Vec<SwapLegSwap>
//     },
//     Split{
//         split_legs: Vec<SplitLegDeeper>
//     },
//     Swap{
//         swap: Swap
//     }
// }

// pub enum SwapLegSwap{
//     PlaceholderOne,
//     PlaceholderTwo,
//     Swap{
//         swap: Swap
//     }
// }

// pub struct SplitLegDeeper {
//     pub percent: u8,
//     pub swap_leg: SwapLegSwap
// }

// pub struct  SplitLeg {
//    pub percent: u8,
//    pub swap_leg: SwapLegDeeper
// }

// pub enum Side {
//     Bid,
//     Ask
// }

// pub enum Swap {
//     Saber,
//     SaberAddDecimalsDeposit,
//     SaberAddDecimalsWithdraw,
//     TokenSwap,
//     Sencha,
//     Step,
//     Cropper,
//     Raydium,
//     Crema,
//     Lifinity,
//     Mercurial,
//     Cykura,
//     Serum{
//         side: Side
//     },
//     MarinadeDeposit,
//     MarinadeUnstake,
//     Aldrin{
//         side: Side
//     },
//     AldrinV2{
//         side: Side
//     },
//     Whirlpool{
//         a_to_b: bool
//     },
//     Invariant{
//         x_to_y: bool
//     },
//     Meteora,
//     GooseFX,
//     DeltaFi{
//         stable: bool
//     }
// }
