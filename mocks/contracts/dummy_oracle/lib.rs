#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod price_oracle {
    use logics::traits::price_oracle::*;
    use openbrush::{
        storage::Mapping,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DummyOracleContract {
        prices: Mapping<AccountId, u128>,
    }

    impl DummyOracleContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn set_price(&mut self, asset: AccountId, price: u128) -> Result<(), ()> {
            self.prices.insert(&asset, &price);
            Ok(())
        }
    }

    impl PriceOracle for DummyOracleContract {
        #[ink(message)]
        fn get(&self, asset: AccountId) -> u128 {
            self.prices.get(&asset).unwrap_or_default()
        }
    }
}
