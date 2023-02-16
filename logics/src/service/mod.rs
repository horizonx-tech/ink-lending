use crate::traits::{
    asset_pool::AssetPoolRef, registry::RegistryRef, risk_strategy::RiskStrategyRef, service::*,
};
use openbrush::traits::{AccountId, Balance, Storage};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
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
    fn deposit(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        self._deposit(asset, amount)
    }

    fn withdraw(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        self._withdraw(asset, amount)
    }

    fn borrow(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        self._borrow(asset, amount)
    }

    fn repay(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        self._repay(asset, amount)
    }

    fn liquidation_call(
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
    fn _deposit(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        let pool = self._pool(asset)?;
        let caller = Self::env().caller();
        match AssetPoolRef::deposit(&pool, caller, caller, amount) {
            Ok(..) => Ok(()),
            Err(e) => Err(Error::Pool(e)),
        }
    }

    fn _withdraw(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        let pool = self._pool(asset)?;
        let caller = Self::env().caller();
        match AssetPoolRef::withdraw(&pool, caller, caller, amount) {
            Ok(..) => Ok(()),
            Err(e) => Err(Error::Pool(e)),
        }
    }

    fn _borrow(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        let pool = self._pool(asset)?;
        let caller = Self::env().caller();
        match AssetPoolRef::borrow(&pool, caller, caller, amount) {
            Ok(..) => Ok(()),
            Err(e) => Err(Error::Pool(e)),
        }
    }

    fn _repay(&mut self, asset: AccountId, amount: Balance) -> Result<()> {
        let pool = self._pool(asset)?;
        let caller = Self::env().caller();
        match AssetPoolRef::repay(&pool, caller, caller, amount) {
            Ok(..) => Ok(()),
            Err(e) => Err(Error::Pool(e)),
        }
    }

    fn _liquidation_call(
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
            Err(e) => return Err(Error::Risk(e)),
        };

        if let Err(e) =
            AssetPoolRef::repay(&debt_pool, liquidatee, Self::env().caller(), debt_amount)
        {
            return Err(Error::Pool(e));
        }

        if let Err(e) = AssetPoolRef::transfer_collateral_on_liquidation(
            &collateral_pool,
            liquidatee,
            Self::env().caller(),
            collateral_amount,
        ) {
            return Err(Error::Pool(e));
        }

        Ok(())
    }

    fn _pool(&self, asset: AccountId) -> Result<AccountId> {
        match RegistryRef::pool(&self.data().registry, asset) {
            Ok(pool) => Ok(pool),
            Err(e) => Err(Error::Registry(e)),
        }
    }
}
