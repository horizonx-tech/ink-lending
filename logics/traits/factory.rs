use ink::prelude::vec::Vec;
use openbrush::traits::AccountId;

use super::registry;

#[openbrush::wrapper]
pub type FactoryRef = dyn Factory;

#[openbrush::trait_definition]
pub trait Factory {
    #[ink(message)]
    fn create(&self, asset: AccountId, data: Vec<u8>) -> Result<AccountId>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    PoolImplementationMissing,
    CallerIsNotManager,
    Registry(registry::Error),
    Custom(u8),
}

pub type Result<T> = core::result::Result<T, Error>;
