use openbrush::traits::{
    AccountId,
    Balance,
};

#[openbrush::wrapper]
pub type RateStrategyRef = dyn RateStrategy;

#[openbrush::trait_definition]
pub trait RateStrategy {
    // liquidity, variable_debt
    #[ink(message)]
    fn calculate_rate(
        &self,
        asset: AccountId,
        liquidity_added: Balance,
        liquidity_taken: Balance,
    ) -> (u128, u128);
}
