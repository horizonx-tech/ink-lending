use openbrush::traits::AccountId;
use openbrush::contracts::traits::ownable::{*, OwnableError};
use super::{factory, registry};

#[openbrush::wrapper]
pub type ManagerRef = dyn Manager;

#[openbrush::trait_definition]
pub trait Manager: Ownable + {
    #[ink(message)]
    fn factory(&self) -> AccountId;

    #[ink(message)]
    fn registry(&self) -> AccountId;

    #[ink(message)]
    fn set_factory(&mut self, id: AccountId) -> Result<()>;

    #[ink(message)]
    fn set_registry(&mut self, id: AccountId) -> Result<()>;

    #[ink(message)]
    fn create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId>;

    #[ink(message)]
    fn update_rate_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;

    #[ink(message)]
    fn update_risk_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    OwnableError(OwnableError),
    FactoryError(factory::Error),
    RegistryError(registry::Error)
}

impl From<OwnableError> for Error {
    fn from(error: OwnableError) -> Self {
        Error::OwnableError(error)
    }
}

pub type Result<T> = core::result::Result<T, Error>;