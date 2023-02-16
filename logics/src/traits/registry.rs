use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type RegistryRef = dyn Registry;

#[openbrush::trait_definition]
pub trait Registry {
    #[ink(message)]
    fn pool(&self, asset: AccountId) -> Option<AccountId>;

    #[ink(message)]
    fn rate_strategy(&self, asset: AccountId) -> AccountId;

    #[ink(message)]
    fn risk_strategy(&self, asset: AccountId) -> AccountId;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
}

pub type Result<T> = core::result::Result<T, Error>;
