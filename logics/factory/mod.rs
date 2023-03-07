use crate::traits::{
    factory::*,
    registry::{
        self,
        RegistryRef,
    },
};
use ink::{
    env::hash::Blake2x256,
    prelude::vec::Vec,
};
use openbrush::traits::{
    AccountId,
    Hash,
    Storage,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub registry: AccountId,
    pub pool_code_hash: Hash,
    pub shares_code_hash: Hash,
}

pub trait Internal {
    fn _create(&self, asset: AccountId, data: &Vec<u8>) -> Result<AccountId>;
    fn _instantiate(&self, asset: AccountId, salt: &[u8], data: &Vec<u8>) -> Result<AccountId>;
    fn _on_create_pool(&self, asset: AccountId, pool: AccountId, data: &Vec<u8>) -> Result<()>;
    fn _assert_manager(&self) -> Result<()>;
    fn _manager(&self) -> AccountId;
}

impl<T: Storage<Data>> Factory for T {
    default fn create(&self, asset: AccountId, data: Vec<u8>) -> Result<AccountId> {
        self._assert_manager()?;
        let pool = self._create(asset, &data)?;
        self._on_create_pool(asset, pool, &data)?;
        return Ok(pool)
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _create(&self, asset: AccountId, data: &Vec<u8>) -> Result<AccountId> {
        let salt = Self::env().hash_encoded::<Blake2x256, _>(&asset);
        let pool = self._instantiate(asset, salt.as_ref(), data)?;
        RegistryRef::register_pool(&self.data().registry, asset, pool)
            .map_err(to_registry_error)?;

        Ok(pool)
    }

    // must be overriden
    default fn _instantiate(
        &self,
        _asset: AccountId,
        _salt: &[u8],
        _data: &Vec<u8>,
    ) -> Result<AccountId> {
        Err(Error::PoolImplementationMissing)
    }

    default fn _on_create_pool(
        &self,
        _asset: AccountId,
        _pool: AccountId,
        _data: &Vec<u8>,
    ) -> Result<()> {
        Ok(())
    }

    default fn _assert_manager(&self) -> Result<()> {
        if self._manager() != Self::env().caller() {
            return Err(Error::CallerIsNotManager)
        }
        Ok(())
    }

    default fn _manager(&self) -> AccountId {
        RegistryRef::manager(&self.data().registry)
    }
}

pub fn to_registry_error(e: registry::Error) -> Error {
    Error::Registry(e)
}
