use super::{
    factory::Error as FactoryError,
    registry::Error as RegistryError,
};
use ink::prelude::vec::Vec;
use openbrush::{
    contracts::traits::access_control::AccessControlError,
    traits::AccountId,
};

#[openbrush::wrapper]
pub type ManagerRef = dyn Manager;

#[openbrush::trait_definition]
pub trait Manager {
    #[ink(message)]
    fn registry(&self) -> AccountId;

    #[ink(message)]
    fn create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId>;

    #[ink(message)]
    fn update_rate_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;

    #[ink(message)]
    fn update_risk_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;

    #[ink(message)]
    fn set_factory(&mut self, address: AccountId) -> Result<()>;

    #[ink(message)]
    fn set_price_oracle(&mut self, address: AccountId) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Factory(FactoryError),
    Registry(RegistryError),
    AccessControl(AccessControlError),
}

impl From<AccessControlError> for Error {
    fn from(error: AccessControlError) -> Self {
        Error::AccessControl(error)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
