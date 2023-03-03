use crate::traits::{
    asset_pool::{
        self,
        AssetPoolRef,
    },
    registry::RegistryRef,
    risk_strategy::RiskStrategyRef,
    service::*,
};
use openbrush::traits::{
    AccountId,
    Balance,
    Storage,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub registry: AccountId,
}

pub trait Internal {
    fn _deposit(
        &mut self,
        asset: AccountId,
        amount: Balance,
        account: Option<AccountId>,
    ) -> Result<()>;
    fn _withdraw(&mut self, asset: AccountId, amount: Balance, to: Option<AccountId>)
        -> Result<()>;
    fn _borrow(
        &mut self,
        asset: AccountId,
        amount: Balance,
        account: Option<AccountId>,
    ) -> Result<()>;
    fn _repay(
        &mut self,
        asset: AccountId,
        amount: Balance,
        account: Option<AccountId>,
    ) -> Result<()>;
    fn _liquidation_call(
        &mut self,
        liquidatee: AccountId,
        collateral_asset: AccountId,
        debt_asset: AccountId,
        debt_amount: Balance,
    ) -> Result<()>;
    fn _pool(&self, asset: AccountId) -> Result<AccountId>;

    // event emission
    fn _emit_deposited_event(
        &self,
        asset: AccountId,
        account: AccountId,
        from: AccountId,
        amount: Balance,
    );
    fn _emit_withdrew_event(
        &self,
        asset: AccountId,
        account: AccountId,
        to: AccountId,
        amount: Balance,
    );
    fn _emit_borrowed_event(
        &self,
        asset: AccountId,
        account: AccountId,
        to: AccountId,
        amount: Balance,
    );
    fn _emit_repaid_event(
        &self,
        asset: AccountId,
        account: AccountId,
        from: AccountId,
        amount: Balance,
    );
    fn _emit_liquidated_event(
        &self,
        liquidator: AccountId,
        liquidatee: AccountId,
        collateral_asset: AccountId,
        collateral_amount: Balance,
        debt_asset: AccountId,
        debt_amount: Balance,
    );
}

impl<T: Storage<Data>> Service for T {
    default fn deposit(
        &mut self,
        asset: AccountId,
        amount: Balance,
        account: Option<AccountId>,
    ) -> Result<()> {
        self._deposit(asset, amount, account)
    }

    default fn withdraw(
        &mut self,
        asset: AccountId,
        amount: Balance,
        to: Option<AccountId>,
    ) -> Result<()> {
        self._withdraw(asset, amount, to)
    }

    default fn borrow(
        &mut self,
        asset: AccountId,
        amount: Balance,
        account: Option<AccountId>,
    ) -> Result<()> {
        self._borrow(asset, amount, account)
    }

    default fn repay(
        &mut self,
        asset: AccountId,
        amount: Balance,
        account: Option<AccountId>,
    ) -> Result<()> {
        self._repay(asset, amount, account)
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
    default fn _deposit(
        &mut self,
        asset: AccountId,
        amount: Balance,
        _account: Option<AccountId>,
    ) -> Result<()> {
        let pool = self._pool(asset)?;
        let from = Self::env().caller();
        let account = _account.unwrap_or(from);
        AssetPoolRef::deposit(&pool, account, from, amount).map_err(to_asset_pool_error)?;

        self._emit_deposited_event(asset, account, from, amount);
        Ok(())
    }

    default fn _withdraw(
        &mut self,
        asset: AccountId,
        amount: Balance,
        _to: Option<AccountId>,
    ) -> Result<()> {
        let pool = self._pool(asset)?;
        let account = Self::env().caller();
        let to = _to.unwrap_or(account);
        AssetPoolRef::withdraw(&pool, account, to, amount).map_err(to_asset_pool_error)?;

        self._emit_withdrew_event(asset, account, to, amount);
        Ok(())
    }

    default fn _borrow(
        &mut self,
        asset: AccountId,
        amount: Balance,
        _to: Option<AccountId>,
    ) -> Result<()> {
        let pool = self._pool(asset)?;
        let account = Self::env().caller();
        let to = _to.unwrap_or(account);
        AssetPoolRef::borrow(&pool, account, to, amount).map_err(to_asset_pool_error)?;

        self._emit_borrowed_event(asset, account, to, amount);
        Ok(())
    }

    default fn _repay(
        &mut self,
        asset: AccountId,
        amount: Balance,
        _account: Option<AccountId>,
    ) -> Result<()> {
        let pool = self._pool(asset)?;
        let from = Self::env().caller();
        let account = _account.unwrap_or(from);
        AssetPoolRef::repay(&pool, account, from, amount).map_err(to_asset_pool_error)?;

        self._emit_repaid_event(asset, account, from, amount);
        Ok(())
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

        let liquidator = Self::env().caller();
        AssetPoolRef::repay(&debt_pool, liquidatee, liquidator, debt_amount)
            .map_err(to_asset_pool_error)?;

        AssetPoolRef::transfer_collateral_on_liquidation(
            &collateral_pool,
            liquidatee,
            liquidator,
            collateral_amount,
        )
        .map_err(to_asset_pool_error)?;

        self._emit_liquidated_event(
            liquidator,
            liquidatee,
            collateral_asset,
            collateral_amount,
            debt_asset,
            debt_amount,
        );
        Ok(())
    }

    default fn _pool(&self, asset: AccountId) -> Result<AccountId> {
        RegistryRef::pool(&self.data().registry, asset).ok_or(Error::PoolNotFound)
    }

    // event emission
    default fn _emit_deposited_event(
        &self,
        _asset: AccountId,
        _account: AccountId,
        _from: AccountId,
        _amount: Balance,
    ) {
    }
    default fn _emit_withdrew_event(
        &self,
        _asset: AccountId,
        _account: AccountId,
        _to: AccountId,
        _amount: Balance,
    ) {
    }
    default fn _emit_borrowed_event(
        &self,
        _asset: AccountId,
        _account: AccountId,
        _to: AccountId,
        _amount: Balance,
    ) {
    }
    default fn _emit_repaid_event(
        &self,
        _asset: AccountId,
        _account: AccountId,
        _from: AccountId,
        _amount: Balance,
    ) {
    }

    default fn _emit_liquidated_event(
        &self,
        _liquidator: AccountId,
        _liquidatee: AccountId,
        _collateral_asset: AccountId,
        _collateral_amount: Balance,
        _debt_asset: AccountId,
        _debt_amount: Balance,
    ) {
    }
}

pub fn to_asset_pool_error(e: asset_pool::Error) -> Error {
    Error::AssetPool(e)
}

pub fn to_risk_error(e: u8) -> Error {
    Error::Risk(e)
}
