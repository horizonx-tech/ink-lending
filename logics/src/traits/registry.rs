use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type RegistryRef = dyn Registry;

#[openbrush::trait_definition]
pub trait Registry {
    #[ink(message)]
    fn rate_strategy(&self, asset: AccountId) -> AccountId;

    #[ink(message)]
    fn validator(&self, asset: AccountId) -> AccountId;
}
