use openbrush::traits::Balance;

use crate::rate_strategy::BigEndian;

#[openbrush::wrapper]
pub type RateStrategyRef = dyn RateStrategy;

#[openbrush::trait_definition]
pub trait RateStrategy {
    // liquidity, debt
    #[ink(message)]
    fn calculate_rate(
        &self,
        _balance: Balance,
        _liquidity_added: Balance,
        _liquidity_taken: Balance,
        _total_debt: Balance,
        _reserve_factor: Balance,
    ) -> (BigEndian, BigEndian);
}
