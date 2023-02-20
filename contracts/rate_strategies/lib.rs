#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod strategies {
    use logics::traits::rate_strategy::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DefaultRateStrategyContract {}

    impl RateStrategy for DefaultRateStrategyContract {
        #[ink(message)]
        fn calculate_rate(
            &self,
            _asset: AccountId,
            _liquidity_added: Balance,
            _liquidity_taken: Balance,
        ) -> (u128, u128) {
            // TODO
            (0, 0)
        }
    }

    impl DefaultRateStrategyContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
