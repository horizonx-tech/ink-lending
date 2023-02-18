use crate::traits::{
    asset_pool::{self, AssetPoolRef},
    registry::RegistryRef,
    risk_strategy::RiskStrategyRef,
    service::*,
};
use openbrush::traits::{AccountId, Balance, Storage};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    registry: AccountId,
}

pub trait Internal {
    fn _deposit(&mut self, asset: AccountId, amount: Balance) -> Result<()>;
    fn _withdraw(&mut self, asset: AccountId, amount: Balance) -> Result<()>;
    fn _borrow(&mut self, asset: AccountId, amount: Balance) -> Result<()>;
    fn _repay(&mut self, asset: AccountId, amount: Balance) -> Result<()>;
    fn _liquidation_call(
        &mut self,
        liquidatee: AccountId,
        collateral_asset: AccountId,
        debt_asset: AccountId,
        debt_amount: Balance,
    ) -> Result<()>;
    fn _pool(&self, asset: AccountId) -> Result<AccountId>;
}

impl<T: Storage<Data>> Service for T {
    default fn deposit(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        self._deposit(asset, amount)
    }

    default fn withdraw(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        self._withdraw(asset, amount)
    }

    default fn borrow(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        self._borrow(asset, amount)
    }

    default fn repay(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        self._repay(asset, amount)
    }

    default fn liquidation_call(
        &mut self,
        liquidatee: AccountId,
        collateral_asset: AccountId,
        debt_asset: AccountId,
        debt_amount: Balance,
    ) -> Result<()> {
        self._liquidation_call(liquidatee, collateral_asset, debt_asset, debt_amount)
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _deposit(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        let pool = self._pool(asset)?;
        let caller = Self::env().caller();
        AssetPoolRef::deposit(&pool, caller, caller, amount).map_err(to_asset_pool_error)
    }

    default fn _withdraw(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        let pool = self._pool(asset)?;
        let caller = Self::env().caller();
        AssetPoolRef::withdraw(&pool, caller, caller, amount).map_err(to_asset_pool_error)
    }

    default fn _borrow(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        let pool = self._pool(asset)?;
        let caller = Self::env().caller();
        AssetPoolRef::borrow(&pool, caller, caller, amount).map_err(to_asset_pool_error)
    }

    default fn _repay(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        let pool = self._pool(asset)?;
        let caller = Self::env().caller();
        AssetPoolRef::repay(&pool, caller, caller, amount).map_err(to_asset_pool_error)
    }

    default fn _liquidation_call(
        &mut self,
        liquidatee: AccountId,
        collateral_asset: AccountId,
        debt_asset: AccountId,
        debt_amount: Balance,
    ) -> Result<()> {
        let collateral_pool = self._pool(collateral_asset)?;
        let debt_pool = self._pool(debt_asset)?;

        let strategy = RegistryRef::risk_strategy(&self.data().registry, debt_asset);
        let collateral_amount = match RiskStrategyRef::validate_liquidation(
            &strategy,
            liquidatee,
            collateral_asset,
            debt_asset,
            debt_amount,
        ) {
            Ok(amount) => amount,
            Err(e) => return Err(to_risk_error(e)),
        };

        AssetPoolRef::repay(&debt_pool, liquidatee, Self::env().caller(), debt_amount)
            .map_err(to_asset_pool_error)?;
        AssetPoolRef::transfer_collateral_on_liquidation(
            &collateral_pool,
            liquidatee,
            Self::env().caller(),
            collateral_amount,
        )
        .map_err(to_asset_pool_error)?;

        Ok(())
    }

    default fn _pool(&self, asset: AccountId) -> Result<AccountId> {
        RegistryRef::pool(&self.data().registry, asset).ok_or(Error::PoolNotFound)
    }
}

pub fn to_asset_pool_error(e: asset_pool::Error) -> Error {
    Error::AssetPool(e)
}

pub fn to_risk_error(e: u8) -> Error {
    Error::Risk(e)
}
