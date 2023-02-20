use crate::traits::registry::*;
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
    pub factory: AccountId,
    pub pools: Mapping<AccountId, AccountId>,
    pub rate_strategies: Mapping<AccountId, AccountId>,
    pub risk_strategies: Mapping<AccountId, AccountId>,
    pub default_rate_strategy: AccountId,
    pub default_risk_strategy: AccountId,
}

trait Internal {
    fn _factory(&self) -> AccountId;
    fn _pool(&self, asset: &AccountId) -> Option<AccountId>;
    fn _rate_strategy(&self, asset: &AccountId) -> AccountId;
    fn _risk_strategy(&self, asset: &AccountId) -> AccountId;
    fn _register_pool(&mut self, asset: &AccountId, pool: &AccountId) -> Result<()>;
    fn _set_factory(&mut self, address: AccountId) -> Result<()>;
    fn _set_rate_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;
    fn _set_risk_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;
}

impl<T: Storage<Data>> Registry for T {
    default fn factory(&self) -> AccountId {
        self._factory()
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

    default fn register_pool(&mut self, asset: AccountId, pool: AccountId) -> Result<()> {
        self._register_pool(&asset, &pool)
    }

    default fn set_factory(&mut self, address: AccountId) -> Result<()> {
        self._set_factory(address)
    }

    default fn set_rate_strategy(
        &mut self,
        address: AccountId,
        asset: Option<AccountId>,
    ) -> Result<()> {
        self._set_rate_strategy(address, asset)
    }

    default fn set_risk_strategy(
        &mut self,
        address: AccountId,
        asset: Option<AccountId>,
    ) -> Result<()> {
        self._set_risk_strategy(address, asset)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            factory: [0u8; 32].into(),
            pools: Default::default(),
            rate_strategies: Default::default(),
            risk_strategies: Default::default(),
            default_rate_strategy: [0u8; 32].into(),
            default_risk_strategy: [0u8; 32].into(),
        }
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _factory(&self) -> AccountId {
        self.data().factory
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

    default fn _register_pool(&mut self, asset: &AccountId, pool: &AccountId) -> Result<()> {
        if self._pool(asset).is_some() {
            return Err(Error::PoolAlreadyExists)
        }
        self.data().pools.insert(asset, pool);

        Ok(())
    }

    default fn _set_factory(&mut self, address: AccountId) -> Result<()> {
        self.data().factory = address;
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
}
