use crate::traits::{
    asset_pool::*,
    rate_strategy::RateStrategyRef,
    registry::RegistryRef,
    risk_strategy::RiskStrategyRef,
    shares::SharesRef,
};
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::{
        psp22::PSP22Error,
        traits::psp22::PSP22Ref,
    },
    traits::{
        AccountId,
        Balance,
        Storage,
        Timestamp,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Clone, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    // config
    pub registry: AccountId,
    pub asset: AccountId,
    pub collateral_token: AccountId,
    pub debt_token: AccountId,
    pub deposit_paused: bool,
    pub borrow_paused: bool,
    // state
    pub liquidity_index: u128,
    pub liquidity_rate: u128,
    pub debt_index: u128,
    pub debt_rate: u128,
    pub last_update_timestamp: Timestamp,
}

pub trait Internal {
    fn _deposit(&mut self, account: AccountId, from: AccountId, amount: Balance) -> Result<()>;
    fn _withdraw(&mut self, account: AccountId, to: AccountId, amount: Balance) -> Result<()>;
    fn _borrow(&mut self, account: AccountId, to: AccountId, amount: Balance) -> Result<()>;
    fn _repay(&mut self, account: AccountId, from: AccountId, amount: Balance) -> Result<()>;
    fn _transfer_collateral_on_liquidation(
        &mut self,
        liquidatee: AccountId,
        receiver: AccountId,
        amount: Balance,
    ) -> Result<()>;
    fn _update_state(&mut self);
    fn _update_rate(&mut self, asset: AccountId, liquidity_added: Balance, liqudity_taken: Balance);
    fn _validate_withdraw(
        &self,
        account: AccountId,
        asset: AccountId,
        amount: Balance,
    ) -> Result<()>;
    fn _validate_borrow(&self, account: AccountId, asset: AccountId, amount: Balance)
        -> Result<()>;
    fn _to_liquidity_share(&self, amount: Balance) -> Balance;
    fn _to_debt_share(&self, amount: Balance) -> Balance;
    fn _calculate_index_with_interest(current_index: u128, rate: u128, elapsed_sec: u128) -> u128;
    fn _set_deposit_paused(&mut self, paused: bool) -> Result<()>;
    fn _set_borrow_paused(&mut self, paused: bool) -> Result<()>;
    fn _assert_deposit_status(&self) -> Result<()>;
    fn _assert_borrow_status(&self) -> Result<()>;
    fn _assert_manager(&self) -> Result<()>;
    fn _data(&self) -> Data;
}

impl<T: Storage<Data>> AssetPool for T {
    default fn deposit(
        &mut self,
        account: AccountId,
        from: AccountId,
        amount: Balance,
    ) -> Result<()> {
        self._deposit(account, from, amount)
    }

    default fn withdraw(
        &mut self,
        account: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<()> {
        self._withdraw(account, to, amount)
    }

    default fn borrow(&mut self, account: AccountId, to: AccountId, amount: Balance) -> Result<()> {
        self._borrow(account, to, amount)
    }

    default fn repay(
        &mut self,
        account: AccountId,
        from: AccountId,
        amount: Balance,
    ) -> Result<()> {
        self._repay(account, from, amount)
    }

    default fn transfer_collateral_on_liquidation(
        &mut self,
        liquidatee: AccountId,
        receiver: AccountId,
        amount: Balance,
    ) -> Result<()> {
        self._transfer_collateral_on_liquidation(liquidatee, receiver, amount)
    }

    default fn set_deposit_paused(&mut self, paused: bool) -> Result<()> {
        self._assert_manager().unwrap();
        self._set_deposit_paused(paused)
    }

    default fn set_borrow_paused(&mut self, paused: bool) -> Result<()> {
        self._assert_manager().unwrap();
        self._set_borrow_paused(paused)
    }

    default fn registry(&self) -> AccountId { self.data().registry }
    default fn asset(&self) -> AccountId { self.data().asset }
    default fn collateral_token(&self) -> AccountId { self.data().collateral_token }
    default fn debt_token(&self) -> AccountId { self.data().debt_token }
    default fn deposit_paused(&self) -> bool { self.data().deposit_paused }
    default fn borrow_paused(&self) -> bool { self.data().borrow_paused }
    default fn liquidity_index(&self) -> u128 { self.data().liquidity_index }
    default fn liquidity_rate(&self) -> u128 { self.data().liquidity_rate }
    default fn debt_index(&self) -> u128 { self.data().debt_index }
    default fn debt_rate(&self) -> u128 { self.data().debt_rate }
    default fn last_update_timestamp(&self) -> Timestamp { self.data().last_update_timestamp }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            registry: AccountId::from([0u8; 32]),
            asset: AccountId::from([0u8; 32]),
            collateral_token: AccountId::from([0u8; 32]),
            debt_token: AccountId::from([0u8; 32]),
            deposit_paused: false,
            borrow_paused: false,
            liquidity_index: 0,
            liquidity_rate: 0,
            debt_index: 0,
            debt_rate: 0,
            last_update_timestamp: 0,
        }
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _deposit(
        &mut self,
        account: AccountId,
        from: AccountId,
        amount: Balance,
    ) -> Result<()> {
        self._assert_deposit_status()?;

        self._update_state();
        let asset = self.data().asset;
        self._update_rate(asset, amount, 0);

        let collateral_token = self.data().collateral_token;
        PSP22Ref::transfer_from(&asset, from, collateral_token, amount, Vec::<u8>::new())
            .map_err(to_psp22_error)?;
        let share = self._to_liquidity_share(amount);
        SharesRef::mint(&collateral_token, account, share).map_err(to_psp22_error)?;

        Ok(())
    }

    default fn _withdraw(
        &mut self,
        account: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<()> {
        self._update_state();
        let asset = self.data().asset;
        self._update_rate(asset, 0, amount);

        self._validate_withdraw(account, asset, amount)?;

        let collateral_token = self.data().collateral_token;
        let share = self._to_liquidity_share(amount);
        SharesRef::burn(&collateral_token, account, share).map_err(to_psp22_error)?;
        PSP22Ref::transfer_from(&asset, collateral_token, to, amount, Vec::<u8>::new())
            .map_err(to_psp22_error)?;

        Ok(())
    }

    default fn _borrow(
        &mut self,
        account: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<()> {
        self._assert_borrow_status()?;

        self._update_state();
        let asset = self.data().asset;
        self._update_rate(asset, 0, amount);

        self._validate_borrow(account, asset, amount)?;

        let share = self._to_debt_share(amount);
        SharesRef::mint(&self.data().debt_token, account, share).map_err(to_psp22_error)?;
        PSP22Ref::transfer_from(
            &asset,
            self.data().collateral_token,
            to,
            amount,
            Vec::<u8>::new(),
        )
        .map_err(to_psp22_error)?;

        Ok(())
    }

    default fn _repay(
        &mut self,
        account: AccountId,
        from: AccountId,
        amount: Balance,
    ) -> Result<()> {
        self._update_state();
        let asset = self.data().asset;
        self._update_rate(asset, amount, 0);

        PSP22Ref::transfer_from(
            &asset,
            from,
            self.data().collateral_token,
            amount,
            Vec::<u8>::new(),
        )
        .map_err(to_psp22_error)?;
        let share = self._to_debt_share(amount);
        SharesRef::burn(&self.data().debt_token, account, share).map_err(to_psp22_error)?;

        Ok(())
    }

    default fn _transfer_collateral_on_liquidation(
        &mut self,
        liquidatee: AccountId,
        receiver: AccountId,
        amount: Balance,
    ) -> Result<()> {
        self._update_state();
        let asset = self.data().asset;
        self._update_rate(asset, 0, amount);

        let collateral_token = self.data().collateral_token;
        SharesRef::burn(&collateral_token, liquidatee, amount).map_err(to_psp22_error)?;
        // TODO collect fees
        PSP22Ref::transfer_from(&asset, collateral_token, receiver, amount, Vec::<u8>::new())
            .map_err(to_psp22_error)?;

        Ok(())
    }

    default fn _update_state(&mut self) {
        let timestamp = Self::env().block_timestamp();
        let elapsed = (timestamp - self.data().last_update_timestamp) as u128;

        self.data().liquidity_index = Self::_calculate_index_with_interest(
            self.data().liquidity_index,
            self.data().liquidity_rate,
            elapsed,
        );
        self.data().debt_index = Self::_calculate_index_with_interest(
            self.data().debt_index,
            self.data().debt_rate,
            elapsed,
        );
        self.data().last_update_timestamp = timestamp;
    }

    default fn _update_rate(
        &mut self,
        asset: AccountId,
        liquidity_added: Balance,
        liquidity_taken: Balance,
    ) {
        let strategy = RegistryRef::rate_strategy(&self.data().registry, asset);
        let (liquidity_rate, debt_rate) =
            RateStrategyRef::calculate_rate(&strategy, asset, liquidity_added, liquidity_taken);

        self.data().liquidity_rate = liquidity_rate;
        self.data().debt_rate = debt_rate;
    }

    default fn _validate_withdraw(
        &self,
        account: AccountId,
        asset: AccountId,
        amount: Balance,
    ) -> Result<()> {
        // TODO reject if withdrawer balance insufficient
        // TODO reject if withdrawer collateral insufficient
        // TODO reject if asset balance of collateral token insufficient
        let strategy = RegistryRef::risk_strategy(&self.data().registry, asset);
        RiskStrategyRef::validate_borrow(&strategy, account, asset, amount).map_err(to_risk_error)
    }

    default fn _validate_borrow(
        &self,
        account: AccountId,
        asset: AccountId,
        amount: Balance,
    ) -> Result<()> {
        // TODO reject if borrower collateral insufficient
        // TODO reject if asset balance of collateral token insufficient
        let strategy = RegistryRef::risk_strategy(&self.data().registry, asset);
        RiskStrategyRef::validate_withdraw(&strategy, account, asset, amount).map_err(to_risk_error)
    }

    default fn _to_liquidity_share(&self, amount: Balance) -> Balance {
        let liquidity_index = self.data().liquidity_index;
        if liquidity_index == 0 {
            amount
        } else {
            amount / liquidity_index
        }
    }

    default fn _to_debt_share(&self, amount: Balance) -> Balance {
        let debt_index = self.data().debt_index;
        if debt_index == 0 {
            amount
        } else {
            amount / debt_index
        }
    }

    default fn _calculate_index_with_interest(
        _current_index: u128,
        _rate: u128,
        _elapsed_sec: u128,
    ) -> u128 {
        todo!()
    }

    default fn _set_deposit_paused(&mut self, paused: bool) -> Result<()> {
        self.data().deposit_paused = paused;
        Ok(())
    }

    default fn _set_borrow_paused(&mut self, paused: bool) -> Result<()> {
        self.data().borrow_paused = paused;
        Ok(())
    }

    default fn _assert_deposit_status(&self) -> Result<()> {
        if self.data().deposit_paused {
            return Err(Error::DepositPaused)
        }
        Ok(())
    }

    default fn _assert_borrow_status(&self) -> Result<()> {
        if self.data().borrow_paused {
            return Err(Error::BorrowPaused)
        }
        Ok(())
    }

    default fn _assert_manager(&self) -> Result<()> {
        let manager = RegistryRef::manager(&self.data().registry);
        if manager != Self::env().caller() {
            return Err(Error::CallerIsNotManager)
        }
        Ok(())
    }

    default fn _data(&self) -> Data {
        self.data().clone()
    }
}

pub fn to_psp22_error(e: PSP22Error) -> Error {
    Error::PSP22(e)
}

pub fn to_risk_error(e: u8) -> Error {
    Error::Risk(e)
}
