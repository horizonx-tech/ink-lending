use openbrush::{
    contracts::ownable,
    traits::{AccountId, Storage}
};

use crate::{
    traits::manager::*,
    traits::factory::{
        self,
        FactoryRef,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub pool_admin: AccountId,
    pub emergency_admin: AccountId,
    pub factory: AccountId,
}

pub trait Internal {
    fn _pool_admin(&self) -> AccountId;
    fn _emergency_admin(&self) -> AccountId;
    fn _factory(&self) -> AccountId;
    fn _set_pool_admin(&mut self, address: AccountId) -> Result<()>;
    fn _set_emergency_admin(&mut self, address: AccountId) -> Result<()>;
    fn _set_factory(&mut self, address: AccountId) -> Result<()>;

    fn _create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId>;

    // event emission
    fn _emit_pool_admin_ownership_transferred_event(&self, previous: Option<AccountId>, new: Option<AccountId>);
    fn _emit_emergency_admin_ownership_transferred_event(&self, previous: Option<AccountId>, new: Option<AccountId>);
}

impl<T: Storage<Data> + Storage<ownable::Data>> Manager for T {
    default fn pool_admin(&self) -> AccountId {
        self._pool_admin()
    }

    default fn emergency_admin(&self) -> AccountId {
        self._emergency_admin()
    }

    default fn factory(&self) -> AccountId {
        self._factory()
    }

    default fn set_pool_admin(&mut self, id: AccountId) -> Result<()> {
        self._set_pool_admin(id)
    }

    default fn set_emergency_admin(&mut self, id: AccountId) -> Result<()> {
        self._set_emergency_admin(id)
    }

    default fn set_factory(&mut self, id: AccountId) -> Result<()> {
        self._set_factory(id)
    }

    default fn create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId> {
        self._create_pool(asset, data.clone())
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            pool_admin: [0u8; 32].into(),
            emergency_admin: [0u8; 32].into(),
            factory: [0u8; 32].into(),
        }
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _pool_admin(&self) -> AccountId {
        self.data().pool_admin
    }

    default fn _emergency_admin(&self) -> AccountId {
        self.data().emergency_admin
    }

    default fn _factory(&self) -> AccountId {
        self.data().factory
    }

    default fn _set_pool_admin(&mut self, id: AccountId) -> Result<()> {
        let previous = self._pool_admin();
        self.data().pool_admin = id;
        self._emit_pool_admin_ownership_transferred_event(Some(previous), Some(id));
        Ok(())
    }

    default fn _set_emergency_admin(&mut self, id: AccountId) -> Result<()> {
        let previous = self._emergency_admin();
        self.data().emergency_admin = id;
        self._emit_emergency_admin_ownership_transferred_event(Some(previous), Some(id));
        Ok(())
    }

    default fn _set_factory(&mut self, id: AccountId) -> Result<()> {
        // let previous = self._factory();
        self.data().factory = id;
        // self._emit_factory_changed_event(Some(previous), Some(id));
        Ok(())
    }

    default fn _create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId> {
        FactoryRef::create(&self.data().factory, asset, data)
            .map_err(to_factory_error)
    }

    default fn _emit_pool_admin_ownership_transferred_event(
        &self,
        _previous: Option<AccountId>,
        _new: Option<AccountId>
    ) {}

    default fn _emit_emergency_admin_ownership_transferred_event(
        &self,
        _previous: Option<AccountId>,
        _new: Option<AccountId>
    ) {}
}

pub fn to_factory_error(e: factory::Error) -> Error {
    Error::FactoryError(e)
}
