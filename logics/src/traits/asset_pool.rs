use openbrush::{
    contracts::psp22::PSP22Error,
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type AssetPoolRef = dyn AssetPool;

#[openbrush::trait_definition]
pub trait AssetPool {
    #[ink(message)]
    fn deposit(&mut self, account: AccountId, from: AccountId, amount: Balance) -> Result<()>;

    #[ink(message)]
    fn withdraw(&mut self, account: AccountId, to: AccountId, amount: Balance) -> Result<()>;

    #[ink(message)]
    fn borrow(&mut self, account: AccountId, to: AccountId, amount: Balance) -> Result<()>;

    #[ink(message)]
    fn repay(&mut self, account: AccountId, from: AccountId, amount: Balance) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    PSP22(PSP22Error),
    Risk(u8),
}

pub type Result<T> = core::result::Result<T, Error>;
