use openbrush::{
    contracts::psp22::PSP22Error,
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type PoolRef = dyn Pool;

#[openbrush::trait_definition]
pub trait Pool {
    #[ink(message)]
    fn deposit(&self, asset: AccountId, amount: Balance) -> Result<()>;

    #[ink(message)]
    fn withdraw(&self, asset: AccountId, amount: Balance) -> Result<()>;

    #[ink(message)]
    fn borrow(&self, asset: AccountId, amount: Balance) -> Result<()>;

    #[ink(message)]
    fn repay(&self, asset: AccountId, amount: Balance) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    PSP22(PSP22Error),
}

pub type Result<T> = core::result::Result<T, Error>;
