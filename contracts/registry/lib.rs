#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod registry {
    use logics::{
        registry::*,
        traits::registry::*,
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct RegistryContract {
        #[storage_field]
        registry: Data,
    }

    impl Registry for RegistryContract {}

    impl RegistryContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
