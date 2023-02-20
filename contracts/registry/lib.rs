#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod registry {
    use logics::{
        registry::*,
        traits::registry::*,
    };
    use openbrush::{
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Storage)]
    pub struct RegistryContract {
        #[storage_field]
        registry: Data,
    }

    impl Registry for RegistryContract {}

    impl RegistryContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                registry: Data {
                    pools: Default::default(),
                    rate_strategies: Default::default(),
                    risk_strategies: Default::default(),
                    default_rate_strategy: [0u8; 32].into(),
                    default_risk_strategy: [0u8; 32].into(),
                },
            }
        }
    }
}
