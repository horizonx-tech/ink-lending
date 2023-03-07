use ink::prelude::vec::Vec;
use openbrush::{
    contracts::access_control,
    traits::{
        AccountId,
        Storage,
    },
};

use crate::traits::{
    factory::{
        self,
        FactoryRef,
    },
    manager::*,
    registry::{
        self,
        RegistryRef,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub registry: AccountId,
}

pub trait Internal {
    fn _registry(&self) -> AccountId;
    fn _factory(&self) -> AccountId;
    fn _create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId>;
    fn _update_rate_strategy(&mut self, address: AccountId, asset: Option<AccountId>)
        -> Result<()>;
    fn _update_risk_strategy(&mut self, address: AccountId, asset: Option<AccountId>)
        -> Result<()>;
}

impl<T: Storage<Data> + Storage<access_control::Data>> Manager for T {
    default fn registry(&self) -> AccountId {
        self._registry()
    }

    default fn create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId> {
        self._create_pool(asset, data.clone())
    }

    default fn update_rate_strategy(
        &mut self,
        address: AccountId,
        asset: Option<AccountId>,
    ) -> Result<()> {
        self._update_rate_strategy(address, asset)
    }

    default fn update_risk_strategy(
        &mut self,
        address: AccountId,
        asset: Option<AccountId>,
    ) -> Result<()> {
        self._update_risk_strategy(address, asset)
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _registry(&self) -> AccountId {
        self.data().registry
    }

    default fn _factory(&self) -> AccountId {
        RegistryRef::factory(&self._registry())
    }

    default fn _create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId> {
        FactoryRef::create(&self._factory(), asset, data).map_err(to_factory_error)
    }

    default fn _update_rate_strategy(
        &mut self,
        address: AccountId,
        asset: Option<AccountId>,
    ) -> Result<()> {
        RegistryRef::set_rate_strategy(&self._factory(), address, asset)
            .map_err(to_registry_error)
    }

    default fn _update_risk_strategy(
        &mut self,
        address: AccountId,
        asset: Option<AccountId>,
    ) -> Result<()> {
        RegistryRef::set_risk_strategy(&self._factory(), address, asset)
            .map_err(to_registry_error)
    }
}

pub fn to_factory_error(e: factory::Error) -> Error {
    Error::Factory(e)
}

pub fn to_registry_error(e: registry::Error) -> Error {
    Error::Registry(e)
}
