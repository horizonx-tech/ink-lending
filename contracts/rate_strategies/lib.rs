#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod strategies {
    use logics::{
        rate_strategy::*,
        traits::rate_strategy::*,
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DefaultRateStrategyContract {
        #[storage_field]
        strategy: Data,
    }

    impl RateStrategy for DefaultRateStrategyContract {
        #[ink(message)]
        fn calculate_rate(
            &self,
            _balance: Balance,
            _liquidity_added: Balance,
            _liquidity_taken: Balance,
            _total_debt: Balance,
            _reserve_factor: Balance,
        ) -> (BigEndian, BigEndian) {
            self.strategy.calculate_rate(
                _balance,
                _liquidity_added,
                _liquidity_taken,
                _total_debt,
                _reserve_factor,
            )
        }
    }

    impl DefaultRateStrategyContract {
        #[ink(constructor)]
        pub fn new(
            base_borrow_rate: BigEndian,
            optimal_utilization_rate: BigEndian,
            rate_slope_1: BigEndian,
            rate_slope_2: BigEndian,
        ) -> Self {
            let mut instance = Self::default();
            instance.strategy = Data::new(
                base_borrow_rate,
                optimal_utilization_rate,
                rate_slope_1,
                rate_slope_2,
            );
            instance
        }
    }
}
