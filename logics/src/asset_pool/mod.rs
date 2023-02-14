use std::default;

use crate::traits::{
    address_provider::AddressProviderRef, asset_pool::*, rate_strategy::RateStrategyRef,
    tokens::shares_token::SharesRef, validator::ValidatorRef,
};
use openbrush::{
    contracts::traits::psp22::PSP22Ref,
    storage::Mapping,
    traits::{AccountId, Balance, Storage, Timestamp},
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    // config
    address_provider: AccountId,
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
    fn _update_rate(&mut self, liquidit_added: Balance, liqudity_taken: Balance);
    fn _validate_withdraw(
        &self,
        account: AccountId,
        asset: AccountId,
        amount: Balance,
    ) -> Result<()>;
    fn _validate_borrow(&self, account: AccountId, asset: AccountId, amount: Balance)
        -> Result<()>;
    fn _to_share(&self, amount: Balance) -> Balance;
}

impl<T: Storage<Data>> AssetPool for T {
    default fn deposit(&self, account: AccountId, amount: Balance) -> Result<()> {
        self._update_state();
        self._update_rate(amount, 0);

        let collateral_token = self.data().collateral_token;
        if let Err(err) = PSP22Ref::transfer_from(
            &self.data().asset,
            account,
            collateral_token,
            amount,
            Vec::<u8>::new(),
        ) {
            return Err(Error::PSP22(err));
        }

        let share = self._to_share(amount);
        if let Err(err) = SharesRef::mint(&collateral_token, account, share) {
            return Err(Error::PSP22(err));
        }

        Ok(())
    }

    default fn withdraw(&self, account: AccountId, amount: Balance) -> Result<()> {
        self._update_state();
        self._update_rate(0, amount);

        let asset = self.data().asset;
        self._validate_withdraw(account, asset, amount)?;

        let collateral_token = self.data().collateral_token;
        if let Err(err) = SharesRef::burn(&collateral_token, account, amount) {
            return Err(Error::PSP22(err));
        }

        if let Err(err) =
            PSP22Ref::transfer_from(&asset, collateral_token, account, amount, Vec::<u8>::new())
        {
            return Err(Error::PSP22(err));
        }

        Ok(())
    }

    default fn borrow(&self, account: AccountId, amount: Balance) -> Result<()> {
        self._update_state();
        self._update_rate(0, amount);

        let asset = self.data().asset;
        self._validate_borrow(account, asset, amount);

        if let Err(err) = SharesRef::mint(&self.data().variable_debt_token, account, amount) {
            return Err(Error::PSP22(err));
        }

        if let Err(err) = PSP22Ref::transfer_from(
            &asset,
            self.data().collateral_token,
            account,
            amount,
            Vec::<u8>::new(),
        ) {
            return Err(Error::PSP22(err));
        }

        Ok(())
    }

    default fn repay(&self, account: AccountId, amount: Balance) -> Result<()> {
        self._update_state();
        self._update_rate(amount, 0);

        if let Err(err) = SharesRef::transfer_from(
            &self.data().asset,
            account,
            self.data().collateral_token,
            amount,
            Vec::<u8>::new(),
        ) {
            return Err(Error::PSP22(err));
        }

        if let Err(err) = SharesRef::burn(&self.data().variable_debt_token, account, amount) {
            return Err(Error::PSP22(err));
        }

        Ok(())
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _update_state(&mut self) {
        let timestamp = Self::env().block_timestamp();
        // TODO update indexes with accrued interest
        let elapsed = timestamp - self.data().last_update_timestamp;

        self.data().last_update_timestamp = timestamp;
    }

    default fn _update_rate(
        &mut self,
        asset: AccountId,
        liquitidy_added: Balance,
        liquidity_taken: Balance,
    ) {
        let strategy = AddressProviderRef::rate_strategy(&self.data().address_provider, asset);
        let (liquidity_rate, variable_debt_rate) =
            RateStrategyRef::calculate_rate(&strategy, asset, liquitidy_added, liquidity_taken);

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
        let validator = AddressProviderRef::validator(&self.data().address_provider, asset);
        if let Err(err) = ValidatorRef::validate_borrow(&validator, account, asset, amount) {
            return Err(Error::Validator(err));
        }

        Ok(())
    }

    default fn _validate_borrow(
        &self,
        account: AccountId,
        asset: AccountId,
        amount: Balance,
    ) -> Result<()> {
        // TODO reject if borrower collateral insufficient
        // TODO reject if asset balance of collateral token insufficient
        let validator = AddressProviderRef::validator(&self.data().address_provider, asset);
        if let Err(err) = ValidatorRef::validate_withdraw(&validator, account, asset, amount) {
            return Err(Error::Validator(err));
        }

        Ok(())
    }

    default fn _to_share(&self, amount: Balance) -> Balance {
        let liquidity_index = self.data().liquidity_index;
        if liquidity_index == 0 {
            amount
        } else {
            amount / liquidity_index
        }
    }
}
