use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type RegistryRef = dyn Registry;

#[openbrush::trait_definition]
pub trait Registry {
    #[ink(message)]
    fn factory(&self) -> AccountId;

    #[ink(message)]
    fn pool(&self, asset: AccountId) -> Option<AccountId>;

    #[ink(message)]
    fn rate_strategy(&self, asset: AccountId) -> AccountId;

    #[ink(message)]
    fn risk_strategy(&self, asset: AccountId) -> AccountId;

    #[ink(message)]
    fn register_pool(&mut self, asset: AccountId, pool: AccountId) -> Result<()>;

    #[ink(message)]
    fn set_factory(&mut self, address: AccountId) -> Result<()>;

    #[ink(message)]
    fn set_rate_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;

    #[ink(message)]
    fn set_risk_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    PoolAlreadyExists,
}

pub type Result<T> = core::result::Result<T, Error>;
