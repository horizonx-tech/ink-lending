use openbrush::{
    contracts::psp22::PSP22Error,
    traits::{AccountId, Balance},
};

#[openbrush::wrapper]
pub type RateStrategyRef = dyn RateStrategy;

#[openbrush::trait_definition]
pub trait RateStrategy {
    #[ink(message)]
    fn calculate_rate(
        &self,
        asset: AccountId,
        liquidity_added: Balance,
        liquidity_taken: Balance,
    ) -> Result;
}

// supply, variable_debt
pub type Result = core::result::Result<(u128, u128), u8>;
