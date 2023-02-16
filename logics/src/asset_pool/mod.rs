use crate::traits::{
    asset_pool::*, rate_strategy::RateStrategyRef, registry::RegistryRef,
    risk_strategy::RiskStrategyRef, shares_token::SharesRef,
};
use openbrush::{
    contracts::{psp22::PSP22Error, traits::psp22::PSP22Ref},
    traits::{AccountId, Balance, Storage, Timestamp},
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    // config
    registry: AccountId,
    asset: AccountId,
    collateral_token: AccountId,
    variable_debt_token: AccountId,
    // state
    liquidity_index: u128,
    liquidity_rate: u128,
    variable_debt_index: u128,
    variable_debt_rate: u128,
    last_update_timestamp: Timestamp,
}

trait Internal {
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
    fn _to_variable_debt_share(&self, amount: Balance) -> Balance;
    fn _calculate_index_with_interest(current_index: u128, rate: u128, elapsed_sec: u128) -> u128;
}

impl<T: Storage<Data>> AssetPool for T {
    default fn deposit(
        &mut self,
        account: AccountId,
        from: AccountId,
        amount: Balance,
    ) -> Result<()> {
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

    default fn withdraw(
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

    default fn borrow(&mut self, account: AccountId, to: AccountId, amount: Balance) -> Result<()> {
        self._update_state();
        let asset = self.data().asset;
        self._update_rate(asset, 0, amount);

        self._validate_borrow(account, asset, amount)?;

        let share = self._to_variable_debt_share(amount);
        SharesRef::mint(&self.data().variable_debt_token, account, share)
            .map_err(to_psp22_error)?;
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

    default fn repay(
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
        let share = self._to_variable_debt_share(amount);
        SharesRef::burn(&self.data().variable_debt_token, account, share)
            .map_err(to_psp22_error)?;

        Ok(())
    }

    default fn transfer_collateral_on_liquidation(
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
}

impl<T: Storage<Data>> Internal for T {
    default fn _update_state(&mut self) {
        let timestamp = Self::env().block_timestamp();
        let elapsed = (timestamp - self.data().last_update_timestamp) as u128;

        self.data().liquidity_index = Self::_calculate_index_with_interest(
            self.data().liquidity_index,
            self.data().liquidity_rate,
            elapsed,
        );
        self.data().variable_debt_index = Self::_calculate_index_with_interest(
            self.data().variable_debt_index,
            self.data().variable_debt_rate,
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
        let (liquidity_rate, variable_debt_rate) =
            RateStrategyRef::calculate_rate(&strategy, asset, liquidity_added, liquidity_taken);

        self.data().liquidity_rate = liquidity_rate;
        self.data().variable_debt_rate = variable_debt_rate;
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

    default fn _to_variable_debt_share(&self, amount: Balance) -> Balance {
        let variable_debt_index = self.data().variable_debt_index;
        if variable_debt_index == 0 {
            amount
        } else {
            amount / variable_debt_index
        }
    }

    default fn _calculate_index_with_interest(
        current_index: u128,
        rate: u128,
        elapsed_sec: u128,
    ) -> u128 {
        todo!()
    }
}

pub fn to_psp22_error(e: PSP22Error) -> Error {
    Error::PSP22(e)
}

pub fn to_risk_error(e: u8) -> Error {
    Error::Risk(e)
}
