#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod manager {
    use logics::{
        manager::*,
        traits::manager::*,
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct ManagerContract {
        #[storage_field]
        manager: Data,
    }

    impl Manager for ManagerContract {}

    impl ManagerContract {
        #[ink(constructor)]
        pub fn new(pool_admin: AccountId, emergency_admin: AccountId) -> Self {
            Self {
                manager: Data {
                    pool_admin,
                    emergency_admin,
                },
            }
        }
    }
}
