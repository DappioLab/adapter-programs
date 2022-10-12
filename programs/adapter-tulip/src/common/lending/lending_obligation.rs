use super::error::LendingError;
use super::last_update::LastUpdate;
use super::{reserve::Reserve, *};
use crate::common::math::{
    common::{TryAdd, TryDiv, TryMul, TrySub},
    decimal::Decimal,
    rate::Rate,
};
use anchor_lang::solana_program::{
    clock::Slot,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use std::{
    cmp::Ordering,
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

/// Max number of collateral and liquidity reserve accounts combined for an obligation
pub const MAX_OBLIGATION_RESERVES: usize = 10;

/// Lending market obligation state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LendingObligation {
    /// Version of the struct
    pub version: u8,
    /// Last update to collateral, liquidity, or their market values
    pub last_update: LastUpdate,
    /// Lending market address
    pub lending_market: Pubkey,
    /// Owner authority which can borrow liquidity
    pub owner: Pubkey,
    /// Deposited collateral for the obligation, unique by deposit reserve address
    pub deposits: Vec<LendingObligationCollateral>,
    /// Borrowed liquidity for the obligation, unique by borrow reserve address
    pub borrows: Vec<LendingObligationLiquidity>,
    /// Market value of deposits
    pub deposited_value: Decimal,
    /// Market value of borrows
    pub borrowed_value: Decimal,
    /// The maximum borrow value at the weighted average loan to value ratio
    pub allowed_borrow_value: Decimal,
    /// The dangerous borrow value at the weighted average liquidation threshold
    pub unhealthy_borrow_value: Decimal,
}

impl LendingObligation {
    /// Create a new obligation
    pub fn new(params: InitLendingObligationParams) -> Self {
        let mut obligation = Self::default();
        Self::init(&mut obligation, params);
        obligation
    }

    /// Initialize an obligation
    pub fn init(&mut self, params: InitLendingObligationParams) {
        self.version = PROGRAM_VERSION;
        self.last_update = LastUpdate::new(params.current_slot);
        self.lending_market = params.lending_market;
        self.owner = params.owner;
        self.deposits = params.deposits;
        self.borrows = params.borrows;
    }

    /// Calculate the current ratio of borrowed value to deposited value
    pub fn loan_to_value(&self) -> Result<Decimal, ProgramError> {
        self.borrowed_value.try_div(self.deposited_value)
    }

    /// Repay liquidity and remove it from borrows if zeroed out
    pub fn repay(&mut self, settle_amount: Decimal, liquidity_index: usize) -> ProgramResult {
        let liquidity = &mut self.borrows[liquidity_index];
        if settle_amount == liquidity.borrowed_amount_wads {
            self.borrows.remove(liquidity_index);
        } else {
            liquidity.repay(settle_amount)?;
        }
        Ok(())
    }

    /// Withdraw collateral and remove it from deposits if zeroed out
    pub fn withdraw(&mut self, withdraw_amount: u64, collateral_index: usize) -> ProgramResult {
        let collateral = &mut self.deposits[collateral_index];
        if withdraw_amount == collateral.deposited_amount {
            self.deposits.remove(collateral_index);
        } else {
            collateral.withdraw(withdraw_amount)?;
        }
        Ok(())
    }

    /// Calculate the maximum collateral value that can be withdrawn
    pub fn max_withdraw_value(&self) -> Result<Decimal, ProgramError> {
        let required_deposit_value = self
            .borrowed_value
            .try_mul(self.deposited_value)?
            .try_div(self.allowed_borrow_value)?;
        if required_deposit_value >= self.deposited_value {
            return Ok(Decimal::zero());
        }
        self.deposited_value.try_sub(required_deposit_value)
    }

    /// Calculate the maximum liquidity value that can be borrowed
    pub fn remaining_borrow_value(&self) -> Result<Decimal, ProgramError> {
        self.allowed_borrow_value.try_sub(self.borrowed_value)
    }

    /// Calculate the maximum liquidation amount for a given liquidity
    pub fn max_liquidation_amount(
        &self,
        liquidity: &LendingObligationLiquidity,
    ) -> Result<Decimal, ProgramError> {
        let max_liquidation_value = self
            .borrowed_value
            .try_mul(Rate::from_percent(LIQUIDATION_CLOSE_FACTOR))?
            .min(liquidity.market_value);
        let max_liquidation_pct = max_liquidation_value.try_div(liquidity.market_value)?;
        liquidity.borrowed_amount_wads.try_mul(max_liquidation_pct)
    }

    /// Find collateral by deposit reserve
    pub fn find_collateral_in_deposits(
        &self,
        deposit_reserve: Pubkey,
    ) -> Result<(&LendingObligationCollateral, usize), ProgramError> {
        if self.deposits.is_empty() {
            msg!("Obligation has no deposits");
            return Err(LendingError::ObligationDepositsEmpty.into());
        }
        let collateral_index = self
            ._find_collateral_index_in_deposits(deposit_reserve)
            .ok_or(LendingError::InvalidObligationCollateral)?;
        Ok((&self.deposits[collateral_index], collateral_index))
    }

    /// Find or add collateral by deposit reserve
    pub fn find_or_add_collateral_to_deposits(
        &mut self,
        deposit_reserve: Pubkey,
    ) -> Result<&mut LendingObligationCollateral, ProgramError> {
        if let Some(collateral_index) = self._find_collateral_index_in_deposits(deposit_reserve) {
            return Ok(&mut self.deposits[collateral_index]);
        }
        if self.deposits.len() + self.borrows.len() >= MAX_OBLIGATION_RESERVES {
            msg!(
                "Obligation cannot have more than {} deposits and borrows combined",
                MAX_OBLIGATION_RESERVES
            );
            return Err(LendingError::ObligationReserveLimit.into());
        }
        let collateral = LendingObligationCollateral::new(deposit_reserve);
        self.deposits.push(collateral);
        Ok(self.deposits.last_mut().unwrap())
    }

    fn _find_collateral_index_in_deposits(&self, deposit_reserve: Pubkey) -> Option<usize> {
        self.deposits
            .iter()
            .position(|collateral| collateral.deposit_reserve == deposit_reserve)
    }

    /// Find liquidity by borrow reserve
    pub fn find_liquidity_in_borrows(
        &self,
        borrow_reserve: Pubkey,
    ) -> Result<(&LendingObligationLiquidity, usize), ProgramError> {
        if self.borrows.is_empty() {
            msg!("Obligation has no borrows");
            return Err(LendingError::ObligationBorrowsEmpty.into());
        }
        let liquidity_index = self
            ._find_liquidity_index_in_borrows(borrow_reserve)
            .ok_or(LendingError::InvalidObligationLiquidity)?;
        Ok((&self.borrows[liquidity_index], liquidity_index))
    }

    /// Find liquidity by borrow reserve mut
    pub fn find_liquidity_in_borrows_mut(
        &mut self,
        borrow_reserve: Pubkey,
    ) -> Result<(&mut LendingObligationLiquidity, usize), ProgramError> {
        if self.borrows.is_empty() {
            msg!("Obligation has no borrows");
            return Err(LendingError::ObligationBorrowsEmpty.into());
        }
        let liquidity_index = self
            ._find_liquidity_index_in_borrows(borrow_reserve)
            .ok_or(LendingError::InvalidObligationLiquidity)?;
        Ok((&mut self.borrows[liquidity_index], liquidity_index))
    }

    /// Find or add liquidity by borrow reserve
    pub fn find_or_add_liquidity_to_borrows(
        &mut self,
        borrow_reserve: Pubkey,
        cumulative_borrow_rate_wads: Decimal,
    ) -> Result<&mut LendingObligationLiquidity, ProgramError> {
        if let Some(liquidity_index) = self._find_liquidity_index_in_borrows(borrow_reserve) {
            return Ok(&mut self.borrows[liquidity_index]);
        }
        if self.deposits.len() + self.borrows.len() >= MAX_OBLIGATION_RESERVES {
            msg!(
                "Obligation cannot have more than {} deposits and borrows combined",
                MAX_OBLIGATION_RESERVES
            );
            return Err(LendingError::ObligationReserveLimit.into());
        }
        let liquidity =
            LendingObligationLiquidity::new(borrow_reserve, cumulative_borrow_rate_wads);
        self.borrows.push(liquidity);
        Ok(self.borrows.last_mut().unwrap())
    }

    fn _find_liquidity_index_in_borrows(&self, borrow_reserve: Pubkey) -> Option<usize> {
        self.borrows
            .iter()
            .position(|liquidity| liquidity.borrow_reserve == borrow_reserve)
    }
}

/// Initialize an obligation
pub struct InitLendingObligationParams {
    /// Last update to collateral, liquidity, or their market values
    pub current_slot: Slot,
    /// Lending market address
    pub lending_market: Pubkey,
    /// Owner authority which can borrow liquidity
    pub owner: Pubkey,
    /// Deposited collateral for the obligation, unique by deposit reserve address
    pub deposits: Vec<LendingObligationCollateral>,
    /// Borrowed liquidity for the obligation, unique by borrow reserve address
    pub borrows: Vec<LendingObligationLiquidity>,
}

impl Sealed for LendingObligation {}
impl IsInitialized for LendingObligation {
    fn is_initialized(&self) -> bool {
        self.version != UNINITIALIZED_VERSION
    }
}

/// Obligation collateral state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LendingObligationCollateral {
    /// Reserve collateral is deposited to
    pub deposit_reserve: Pubkey,
    /// Amount of collateral deposited
    pub deposited_amount: u64,
    /// Collateral market value in quote currency
    pub market_value: Decimal,
}

impl LendingObligationCollateral {
    /// Create new obligation collateral
    pub fn new(deposit_reserve: Pubkey) -> Self {
        Self {
            deposit_reserve,
            deposited_amount: 0,
            market_value: Decimal::zero(),
        }
    }

    /// Increase deposited collateral
    pub fn deposit(&mut self, collateral_amount: u64) -> ProgramResult {
        self.deposited_amount = self
            .deposited_amount
            .checked_add(collateral_amount)
            .ok_or(LendingError::MathOverflow)?;
        Ok(())
    }

    /// Decrease deposited collateral
    pub fn withdraw(&mut self, collateral_amount: u64) -> ProgramResult {
        self.deposited_amount = self
            .deposited_amount
            .checked_sub(collateral_amount)
            .ok_or(LendingError::MathOverflow)?;
        Ok(())
    }
}

/// Obligation liquidity state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LendingObligationLiquidity {
    /// Reserve liquidity is borrowed from
    pub borrow_reserve: Pubkey,
    /// Borrow rate used for calculating interest
    pub cumulative_borrow_rate_wads: Decimal,
    /// Amount of liquidity borrowed plus interest
    pub borrowed_amount_wads: Decimal,
    /// Liquidity market value in quote currency
    pub market_value: Decimal,
}

impl LendingObligationLiquidity {
    /// Create new obligation liquidity
    pub fn new(borrow_reserve: Pubkey, cumulative_borrow_rate_wads: Decimal) -> Self {
        Self {
            borrow_reserve,
            cumulative_borrow_rate_wads,
            borrowed_amount_wads: Decimal::zero(),
            market_value: Decimal::zero(),
        }
    }

    /// Decrease borrowed liquidity
    pub fn repay(&mut self, settle_amount: Decimal) -> ProgramResult {
        self.borrowed_amount_wads = self.borrowed_amount_wads.try_sub(settle_amount)?;
        Ok(())
    }

    /// Increase borrowed liquidity
    pub fn borrow(&mut self, borrow_amount: Decimal) -> ProgramResult {
        self.borrowed_amount_wads = self.borrowed_amount_wads.try_add(borrow_amount)?;
        Ok(())
    }

    /// Accrue interest
    pub fn accrue_interest(&mut self, cumulative_borrow_rate_wads: Decimal) -> ProgramResult {
        match cumulative_borrow_rate_wads.cmp(&self.cumulative_borrow_rate_wads) {
            Ordering::Less => {
                msg!("Interest rate cannot be negative");
                return Err(LendingError::NegativeInterestRate.into());
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                let compounded_interest_rate: Rate = cumulative_borrow_rate_wads
                    .try_div(self.cumulative_borrow_rate_wads)?
                    .try_into()?;

                self.borrowed_amount_wads = self
                    .borrowed_amount_wads
                    .try_mul(compounded_interest_rate)?;
                self.cumulative_borrow_rate_wads = cumulative_borrow_rate_wads;
            }
        }

        Ok(())
    }
}

const OBLIGATION_COLLATERAL_LEN: usize = 88; // 32 + 8 + 16 + 32
const OBLIGATION_LIQUIDITY_LEN: usize = 112; // 32 + 16 + 16 + 16 + 32
const OBLIGATION_LEN: usize = 1300; // 1 + 8 + 1 + 32 + 32 + 16 + 16 + 16 + 16 + 64 + 1 + 1 + (88 * 1) + (112 * 9)
                                    // @TODO: break this up by obligation / collateral / liquidity https://git.io/JOCca
impl Pack for LendingObligation {
    const LEN: usize = OBLIGATION_LEN;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let output = array_mut_ref![dst, 0, OBLIGATION_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            version,
            last_update_slot,
            last_update_stale,
            lending_market,
            owner,
            deposited_value,
            borrowed_value,
            allowed_borrow_value,
            unhealthy_borrow_value,
            _padding,
            deposits_len,
            borrows_len,
            data_flat,
        ) = mut_array_refs![
            output,
            1,
            8,
            1,
            PUBKEY_BYTES,
            PUBKEY_BYTES,
            16,
            16,
            16,
            16,
            64,
            1,
            1,
            OBLIGATION_COLLATERAL_LEN + (OBLIGATION_LIQUIDITY_LEN * (MAX_OBLIGATION_RESERVES - 1))
        ];

        // obligation
        *version = self.version.to_le_bytes();
        *last_update_slot = self.last_update.slot.to_le_bytes();
        pack_bool(self.last_update.stale, last_update_stale);
        lending_market.copy_from_slice(self.lending_market.as_ref());
        owner.copy_from_slice(self.owner.as_ref());
        pack_decimal(self.deposited_value, deposited_value);
        pack_decimal(self.borrowed_value, borrowed_value);
        pack_decimal(self.allowed_borrow_value, allowed_borrow_value);
        pack_decimal(self.unhealthy_borrow_value, unhealthy_borrow_value);
        *deposits_len = u8::try_from(self.deposits.len()).unwrap().to_le_bytes();
        *borrows_len = u8::try_from(self.borrows.len()).unwrap().to_le_bytes();

        let mut offset = 0;

        // deposits
        for collateral in &self.deposits {
            let deposits_flat = array_mut_ref![data_flat, offset, OBLIGATION_COLLATERAL_LEN];
            #[allow(clippy::ptr_offset_with_cast)]
            let (deposit_reserve, deposited_amount, market_value, _padding_deposit) =
                mut_array_refs![deposits_flat, PUBKEY_BYTES, 8, 16, 32];
            deposit_reserve.copy_from_slice(collateral.deposit_reserve.as_ref());
            *deposited_amount = collateral.deposited_amount.to_le_bytes();
            pack_decimal(collateral.market_value, market_value);
            offset += OBLIGATION_COLLATERAL_LEN;
        }

        // borrows
        for liquidity in &self.borrows {
            let borrows_flat = array_mut_ref![data_flat, offset, OBLIGATION_LIQUIDITY_LEN];
            #[allow(clippy::ptr_offset_with_cast)]
            let (
                borrow_reserve,
                cumulative_borrow_rate_wads,
                borrowed_amount_wads,
                market_value,
                _padding_borrow,
            ) = mut_array_refs![borrows_flat, PUBKEY_BYTES, 16, 16, 16, 32];
            borrow_reserve.copy_from_slice(liquidity.borrow_reserve.as_ref());
            pack_decimal(
                liquidity.cumulative_borrow_rate_wads,
                cumulative_borrow_rate_wads,
            );
            pack_decimal(liquidity.borrowed_amount_wads, borrowed_amount_wads);
            pack_decimal(liquidity.market_value, market_value);
            offset += OBLIGATION_LIQUIDITY_LEN;
        }
    }

    /// Unpacks a byte buffer into an [ObligationInfo](struct.ObligationInfo.html).
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![src, 0, OBLIGATION_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            version,
            last_update_slot,
            last_update_stale,
            lending_market,
            owner,
            deposited_value,
            borrowed_value,
            allowed_borrow_value,
            unhealthy_borrow_value,
            _padding,
            deposits_len,
            borrows_len,
            data_flat,
        ) = array_refs![
            input,
            1,
            8,
            1,
            PUBKEY_BYTES,
            PUBKEY_BYTES,
            16,
            16,
            16,
            16,
            64,
            1,
            1,
            OBLIGATION_COLLATERAL_LEN + (OBLIGATION_LIQUIDITY_LEN * (MAX_OBLIGATION_RESERVES - 1))
        ];

        let version = u8::from_le_bytes(*version);
        if version > PROGRAM_VERSION {
            msg!("Obligation version does not match lending program version");
            return Err(ProgramError::InvalidAccountData);
        }

        let deposits_len = u8::from_le_bytes(*deposits_len);
        let borrows_len = u8::from_le_bytes(*borrows_len);
        let mut deposits = Vec::with_capacity(deposits_len as usize + 1);
        let mut borrows = Vec::with_capacity(borrows_len as usize + 1);

        let mut offset = 0;
        for _ in 0..deposits_len {
            let deposits_flat = array_ref![data_flat, offset, OBLIGATION_COLLATERAL_LEN];
            #[allow(clippy::ptr_offset_with_cast)]
            let (deposit_reserve, deposited_amount, market_value, _padding_deposit) =
                array_refs![deposits_flat, PUBKEY_BYTES, 8, 16, 32];
            deposits.push(LendingObligationCollateral {
                deposit_reserve: Pubkey::new(deposit_reserve),
                deposited_amount: u64::from_le_bytes(*deposited_amount),
                market_value: unpack_decimal(market_value),
            });
            offset += OBLIGATION_COLLATERAL_LEN;
        }
        for _ in 0..borrows_len {
            let borrows_flat = array_ref![data_flat, offset, OBLIGATION_LIQUIDITY_LEN];
            #[allow(clippy::ptr_offset_with_cast)]
            let (
                borrow_reserve,
                cumulative_borrow_rate_wads,
                borrowed_amount_wads,
                market_value,
                _padding_borrow,
            ) = array_refs![borrows_flat, PUBKEY_BYTES, 16, 16, 16, 32];
            borrows.push(LendingObligationLiquidity {
                borrow_reserve: Pubkey::new(borrow_reserve),
                cumulative_borrow_rate_wads: unpack_decimal(cumulative_borrow_rate_wads),
                borrowed_amount_wads: unpack_decimal(borrowed_amount_wads),
                market_value: unpack_decimal(market_value),
            });
            offset += OBLIGATION_LIQUIDITY_LEN;
        }

        Ok(Self {
            version,
            last_update: LastUpdate {
                slot: u64::from_le_bytes(*last_update_slot),
                stale: unpack_bool(last_update_stale)?,
            },
            lending_market: Pubkey::new_from_array(*lending_market),
            owner: Pubkey::new_from_array(*owner),
            deposits,
            borrows,
            deposited_value: unpack_decimal(deposited_value),
            borrowed_value: unpack_decimal(borrowed_value),
            allowed_borrow_value: unpack_decimal(allowed_borrow_value),
            unhealthy_borrow_value: unpack_decimal(unhealthy_borrow_value),
        })
    }
}

/// performs an off-chain refresh of the lending obligation
pub fn pseudo_refresh_lending_obligation(
    obligation_info: &mut LendingObligation,
    reserves: HashMap<Pubkey, Reserve>,
) -> Result<(), ProgramError> {
    let mut deposited_value = Decimal::zero();
    let mut borrowed_value = Decimal::zero();
    let mut allowed_borrow_value = Decimal::zero();
    let mut unhealthy_borrow_value = Decimal::zero();

    for collateral in obligation_info.deposits.iter_mut() {
        let deposit_reserve = match reserves.get(&collateral.deposit_reserve) {
            Some(reserve) => reserve,
            None => {
                msg!(
                    "failed to find deposit reserve {}",
                    collateral.deposit_reserve
                );
                return Err(ProgramError::InvalidAccountData);
            }
        };
        // @TODO: add lookup table https://git.io/JOCYq
        let decimals = 10u64
            .checked_pow(deposit_reserve.liquidity.mint_decimals as u32)
            .ok_or(LendingError::MathOverflow)?;

        let market_value = deposit_reserve
            .collateral_exchange_rate()?
            .decimal_collateral_to_liquidity(collateral.deposited_amount.into())?
            .try_mul(deposit_reserve.liquidity.market_price)?
            .try_div(decimals)?;
        collateral.market_value = market_value;

        let loan_to_value_rate = Rate::from_percent(deposit_reserve.config.loan_to_value_ratio);
        let liquidation_threshold_rate =
            Rate::from_percent(deposit_reserve.config.liquidation_threshold);

        deposited_value = deposited_value.try_add(market_value)?;
        allowed_borrow_value =
            allowed_borrow_value.try_add(market_value.try_mul(loan_to_value_rate)?)?;
        unhealthy_borrow_value =
            unhealthy_borrow_value.try_add(market_value.try_mul(liquidation_threshold_rate)?)?;
    }

    for liquidity in obligation_info.borrows.iter_mut() {
        let borrow_reserve = match reserves.get(&liquidity.borrow_reserve) {
            Some(reserve) => reserve,
            None => {
                msg!("failed to find borrow reserve {}", liquidity.borrow_reserve);
                return Err(ProgramError::InvalidAccountData);
            }
        };

        liquidity.accrue_interest(borrow_reserve.liquidity.cumulative_borrow_rate_wads)?;

        // @TODO: add lookup table https://git.io/JOCYq
        let decimals = 10u64
            .checked_pow(borrow_reserve.liquidity.mint_decimals as u32)
            .ok_or(LendingError::MathOverflow)?;

        let market_value = liquidity
            .borrowed_amount_wads
            .try_mul(borrow_reserve.liquidity.market_price)?
            .try_div(decimals)?;
        liquidity.market_value = market_value;

        borrowed_value = borrowed_value.try_add(market_value)?;
    }

    obligation_info.deposited_value = deposited_value;
    obligation_info.borrowed_value = borrowed_value;
    obligation_info.allowed_borrow_value = allowed_borrow_value;
    obligation_info.unhealthy_borrow_value = unhealthy_borrow_value;

    Ok(())
}
