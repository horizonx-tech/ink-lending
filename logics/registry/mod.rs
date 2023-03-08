use crate::traits::registry::*;
use ink::prelude::vec::Vec;
use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        Storage,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub manager: AccountId,
    pub factory: AccountId,
    pub asset_list: Vec<AccountId>,
    pub pools: Mapping<AccountId, AccountId>,
    pub rate_strategies: Mapping<AccountId, AccountId>,
    pub risk_strategies: Mapping<AccountId, AccountId>,
    pub price_oracle: AccountId,
    pub default_rate_strategy: AccountId,
    pub default_risk_strategy: AccountId,
}

pub trait Internal {
    fn _factory(&self) -> AccountId;
    fn _asset_list(&self) -> Vec<AccountId>;
    fn _asset(&self, index: u64) -> Option<AccountId>;
    fn _assets_count(&self) -> u64;
    fn _manager(&self) -> AccountId;
    fn _pool(&self, asset: &AccountId) -> Option<AccountId>;
    fn _rate_strategy(&self, asset: &AccountId) -> AccountId;
    fn _risk_strategy(&self, asset: &AccountId) -> AccountId;
    fn _price_oracle(&self) -> AccountId;
    fn _register_pool(&mut self, asset: &AccountId, pool: &AccountId) -> Result<()>;
    fn _set_factory(&mut self, address: AccountId) -> Result<()>;
    fn _set_manager(&mut self, address: AccountId) -> Result<()>;
    fn _set_rate_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;
    fn _set_risk_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;
    fn _set_price_oracle(&mut self, address: AccountId) -> Result<()>;
    fn _assert_manager(&self) -> Result<()>;

    // event emission
    fn _emit_pool_registered_event(&self, asset: AccountId, pool: AccountId);
    fn _emit_factory_changed_event(&self, factory: AccountId);
    fn _emit_manager_changed_event(&self, manager: AccountId);
    fn _emit_rate_strategy_changed_event(&self, strategy: AccountId, asset: Option<AccountId>);
    fn _emit_risk_strategy_changed_event(&self, strategy: AccountId, asset: Option<AccountId>);
    fn _emit_price_oracle_changed_event(&self, price_oracle: AccountId);
}

impl<T: Storage<Data>> Registry for T {
    default fn factory(&self) -> AccountId {
        self._factory()
    }

    default fn asset_list(&self) -> Vec<AccountId> {
        self._asset_list()
    }

    default fn asset(&self, index: u64) -> Option<AccountId> {
        self._asset(index)
    }

    default fn assets_count(&self) -> u64 {
        self._assets_count()
    }

    default fn manager(&self) -> AccountId {
        self._manager()
    }

    default fn pool(&self, asset: AccountId) -> Option<AccountId> {
        self._pool(&asset)
    }

    default fn rate_strategy(&self, asset: AccountId) -> AccountId {
        self._rate_strategy(&asset)
    }

    default fn risk_strategy(&self, asset: AccountId) -> AccountId {
        self._risk_strategy(&asset)
    }

    default fn default_rate_strategy(&self) -> AccountId {
        self.data().default_rate_strategy
    }

    default fn default_risk_strategy(&self) -> AccountId {
        self.data().default_risk_strategy
    }

    default fn price_oracle(&self) -> AccountId {
        self._price_oracle()
    }

    default fn register_pool(&mut self, asset: AccountId, pool: AccountId) -> Result<()> {
        self._register_pool(&asset, &pool)?;
        self._emit_pool_registered_event(asset, pool);
        Ok(())
    }

    default fn set_factory(&mut self, address: AccountId) -> Result<()> {
        self._assert_manager()?;
        self._set_factory(address)?;
        self._emit_factory_changed_event(address);
        Ok(())
    }

    default fn set_manager(&mut self, address: AccountId) -> Result<()> {
        self._assert_manager()?;
        self._set_manager(address)?;
        self._emit_manager_changed_event(address);
        Ok(())
    }

    default fn set_rate_strategy(
        &mut self,
        address: AccountId,
        asset: Option<AccountId>,
    ) -> Result<()> {
        self._assert_manager()?;
        self._set_rate_strategy(address, asset)?;
        self._emit_rate_strategy_changed_event(address, asset);
        Ok(())
    }

    default fn set_risk_strategy(
        &mut self,
        address: AccountId,
        asset: Option<AccountId>,
    ) -> Result<()> {
        self._assert_manager()?;
        self._set_risk_strategy(address, asset)?;
        self._emit_risk_strategy_changed_event(address, asset);
        Ok(())
    }

    default fn set_price_oracle(&mut self, address: AccountId) -> Result<()> {
        self._assert_manager()?;
        self._set_price_oracle(address)?;
        self._emit_price_oracle_changed_event(address);
        Ok(())
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            manager: [0u8; 32].into(),
            factory: [0u8; 32].into(),
            asset_list: Vec::new(),
            pools: Default::default(),
            rate_strategies: Default::default(),
            risk_strategies: Default::default(),
            price_oracle: [0u8; 32].into(),
            default_rate_strategy: [0u8; 32].into(),
            default_risk_strategy: [0u8; 32].into(),
        }
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _factory(&self) -> AccountId {
        self.data().factory
    }

    default fn _asset_list(&self) -> Vec<AccountId> {
        self.data().asset_list.clone()
    }

    default fn _asset(&self, index: u64) -> Option<AccountId> {
        self.data().asset_list.get(index as usize).cloned()
    }

    default fn _assets_count(&self) -> u64 {
        self.data().asset_list.len() as u64
    }

    default fn _manager(&self) -> AccountId {
        self.data().manager
    }

    default fn _pool(&self, asset: &AccountId) -> Option<AccountId> {
        self.data().pools.get(asset)
    }

    default fn _rate_strategy(&self, asset: &AccountId) -> AccountId {
        self.data()
            .rate_strategies
            .get(asset)
            .unwrap_or(self.data().default_rate_strategy)
    }

    default fn _risk_strategy(&self, asset: &AccountId) -> AccountId {
        self.data()
            .risk_strategies
            .get(asset)
            .unwrap_or(self.data().default_risk_strategy)
    }

    default fn _price_oracle(&self) -> AccountId {
        self.data().price_oracle
    }

    default fn _register_pool(&mut self, asset: &AccountId, pool: &AccountId) -> Result<()> {
        if self._pool(asset).is_some() {
            return Err(Error::PoolAlreadyExists)
        }
        self.data().asset_list.push(*asset);
        self.data().pools.insert(asset, pool);

        Ok(())
    }

    default fn _set_factory(&mut self, address: AccountId) -> Result<()> {
        self.data().factory = address;
        Ok(())
    }

    default fn _set_manager(&mut self, address: AccountId) -> Result<()> {
        self.data().manager = address;
        Ok(())
    }

    default fn _set_rate_strategy(
        &mut self,
        address: AccountId,
        asset: Option<AccountId>,
    ) -> Result<()> {
        if asset.is_some() {
            self.data()
                .rate_strategies
                .insert(&asset.unwrap(), &address)
        } else {
            self.data().default_rate_strategy = address;
        }
        Ok(())
    }

    default fn _set_risk_strategy(
        &mut self,
        address: AccountId,
        asset: Option<AccountId>,
    ) -> Result<()> {
        if asset.is_some() {
            self.data()
                .risk_strategies
                .insert(&asset.unwrap(), &address)
        } else {
            self.data().default_risk_strategy = address;
        }
        Ok(())
    }

    default fn _set_price_oracle(&mut self, address: AccountId) -> Result<()> {
        self.data().price_oracle = address;
        Ok(())
    }

    default fn _assert_manager(&self) -> Result<()> {
        if self.data().manager != Self::env().caller() {
            return Err(Error::CallerIsNotManager)
        }
        Ok(())
    }

    // event emission
    default fn _emit_pool_registered_event(&self, _asset: AccountId, _pool: AccountId) {}
    default fn _emit_factory_changed_event(&self, _factory: AccountId) {}
    default fn _emit_manager_changed_event(&self, _manager: AccountId) {}
    default fn _emit_rate_strategy_changed_event(
        &self,
        _strategy: AccountId,
        _asset: Option<AccountId>,
    ) {
    }
    default fn _emit_risk_strategy_changed_event(
        &self,
        _strategy: AccountId,
        _asset: Option<AccountId>,
    ) {
    }
    default fn _emit_price_oracle_changed_event(&self, _price_oracle: AccountId) {}
}
