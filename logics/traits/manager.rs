use openbrush::traits::AccountId;
use openbrush::contracts::traits::ownable::*;

#[openbrush::wrapper]
pub type ManagerRef = dyn Manager;

#[openbrush::trait_definition]
pub trait Manager: Ownable {
    #[ink(message)]
    fn pool_admin(&self) -> AccountId;

    #[ink(message)]
    fn set_pool_admin(&mut self, id: AccountId) -> Result<()>;

    #[ink(message)]
    fn emergency_admin(&self) -> AccountId;

    #[ink(message)]
    fn set_emergency_admin(&mut self, id: AccountId) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    NewOwnerIsZero,
}

pub type Result<T> = core::result::Result<T, Error>;