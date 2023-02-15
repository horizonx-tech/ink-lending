use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type AddressProviderRef = dyn AddressProvider;

#[openbrush::trait_definition]
pub trait AddressProvider {
    #[ink(message)]
    fn rate_strategy(&self, asset: AccountId) -> AccountId;

    #[ink(message)]
    fn validator(&self, asset: AccountId) -> AccountId;
}
