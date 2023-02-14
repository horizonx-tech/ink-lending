use openbrush::{
    contracts::psp22::PSP22Error,
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type AddressProviderRef = dyn AddressProvider;

#[openbrush::trait_definition]
pub trait AddressProvider {
    #[ink(message)]
    fn rate_strategy(&self, asset: AccountId) -> Result;

    #[ink(message)]
    fn validator(&self, asset: AccountId) -> Result;
}

pub type Result = core::result::Result<AccountId, ()>;
