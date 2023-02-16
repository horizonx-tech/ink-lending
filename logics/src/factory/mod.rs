use crate::traits::{
    factory::*,
    registry::{self, RegistryRef},
};
use openbrush::traits::{AccountId, Hash, Storage};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    registry: AccountId,
    pool_code_hash: Hash,
}

trait Internal {
    fn _create(&self, asset: AccountId, data: &Vec<u8>) -> Result<AccountId>;
    fn _instantiate(&self, asset: AccountId, data: &Vec<u8>) -> Result<AccountId>;
    fn _on_create_pool(&self, asset: AccountId, pool: AccountId, data: &Vec<u8>) -> Result<()>;
}

impl<T: Storage<Data>> Factory for T {
    default fn create(&self, asset: AccountId, data: Vec<u8>) -> Result<AccountId> {
        let pool = self._create(asset, &data)?;
        self._on_create_pool(asset, pool, &data)?;
        return Ok(pool);
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _create(&self, asset: AccountId, data: &Vec<u8>) -> Result<AccountId> {
        let pool = self._instantiate(asset, data)?;
        RegistryRef::register_pool(&self.data().registry, asset, pool)
            .map_err(to_registry_error)?;

        Ok(pool)
    }

    // must be overriden
    default fn _instantiate(&self, asset: AccountId, data: &Vec<u8>) -> Result<AccountId> {
        Err(Error::PoolImplementationMissing)
    }

    default fn _on_create_pool(
        &self,
        asset: AccountId,
        pool: AccountId,
        data: &Vec<u8>,
    ) -> Result<()> {
        Ok(())
    }
}

pub fn to_registry_error(e: registry::Error) -> Error {
    Error::Registry(e)
}
