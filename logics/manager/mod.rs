use openbrush::{
    contracts::access_control,
    traits::{AccountId, Storage}
};

use crate::{
    traits::manager::*,
    traits::factory::{
        self,
        FactoryRef,
    },
    traits::registry::{
        self,
        RegistryRef,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub factory: AccountId,
    pub registry: AccountId,
}

pub trait Internal {
    fn _factory(&self) -> AccountId;
    fn _registry(&self) -> AccountId;
    fn _set_factory(&mut self, address: AccountId) -> Result<()>;
    fn _set_registry(&mut self, address: AccountId) -> Result<()>;

    fn _create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId>;
    fn _update_rate_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;
    fn _update_risk_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;
}

impl<T: Storage<Data> + Storage<access_control::Data>> Manager for T {
    default fn factory(&self) -> AccountId {
        self._factory()
    }

    default fn registry(&self) -> AccountId {
        self._registry()
    }

    default fn set_factory(&mut self, id: AccountId) -> Result<()> {
        self._set_factory(id)
    }

    default fn set_registry(&mut self, id: AccountId) -> Result<()> {
        self._set_registry(id)
    }

    default fn create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId> {
        self._create_pool(asset, data.clone())
    }

    default fn update_rate_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()> {
        self._update_rate_strategy(address, asset)
    }

    default fn update_risk_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()> {
        self._update_risk_strategy(address, asset)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            factory: [0u8; 32].into(),
            registry: [0u8; 32].into(),
        }
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _factory(&self) -> AccountId {
        self.data().factory
    }

    default fn _registry(&self) -> AccountId {
        self.data().registry
    }

    default fn _set_factory(&mut self, id: AccountId) -> Result<()> {
        // let previous = self._factory();
        self.data().factory = id;
        // self._emit_factory_changed_event(Some(previous), Some(id));
        Ok(())
    }

    default fn _set_registry(&mut self, id: AccountId) -> Result<()> {
        // let previous = self._registry();
        self.data().registry = id;
        // self._emit_registry_changed_event(Some(previous), Some(id));
        Ok(())
    }

    default fn _create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId> {
        FactoryRef::create(&self.data().factory, asset, data)
            .map_err(to_factory_error)
    }

    default fn _update_rate_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()> {
        RegistryRef::set_rate_strategy(&self.data().factory, address, asset)
            .map_err(to_registry_error)
    }

    default fn _update_risk_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()> {
        RegistryRef::set_risk_strategy(&self.data().factory, address, asset)
            .map_err(to_registry_error)
    }
}

pub fn to_factory_error(e: factory::Error) -> Error {
    Error::FactoryError(e)
}

pub fn to_registry_error(e: registry::Error) -> Error {
    Error::RegistryError(e)
}
