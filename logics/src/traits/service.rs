use openbrush::traits::{AccountId, Balance};

use super::{asset_pool, registry};

#[openbrush::wrapper]
pub type ServiceRef = dyn Service;

#[openbrush::trait_definition]
pub trait Service {
    #[ink(message)]
    fn deposit(&mut self, asset: AccountId, amount: Balance) -> Result<()>;

    #[ink(message)]
    fn withdraw(&mut self, asset: AccountId, amount: Balance) -> Result<()>;

    #[ink(message)]
    fn borrow(&mut self, asset: AccountId, amount: Balance) -> Result<()>;

    #[ink(message)]
    fn repay(&mut self, asset: AccountId, amount: Balance) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Pool(asset_pool::Error),
    Registry(registry::Error),
    Risk(u8),
}

pub type Result<T> = core::result::Result<T, Error>;
