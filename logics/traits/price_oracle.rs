use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type PriceOracleRef = dyn PriceOracle;

#[openbrush::trait_definition]
pub trait PriceOracle {
    #[ink(message)]
    fn get(&self, asset: AccountId) -> u128;
}
