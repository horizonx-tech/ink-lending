#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[ink::contract]
pub mod ui_data_provider {
    use asset_pool::pool::{
        AssetPoolContractRef,
        PoolData,
    };
    use ink::{
        env::call::FromAccountId,
        prelude::vec::Vec,
    };
    use logics::traits::registry::RegistryRef;

    #[ink(storage)]
    pub struct UIDataProvider {
        registry: AccountId,
    }

    impl UIDataProvider {
        #[ink(constructor)]
        pub fn new(registry: AccountId) -> Self {
            Self { registry }
        }

        #[ink(message)]
        pub fn assets(&self) -> Vec<AccountId> {
            self._assets()
        }

        #[ink(message)]
        pub fn pools(&self) -> Vec<PoolData> {
            self._assets()
                .iter()
                .map(|asset| self._pool(*asset))
                .collect()
        }

        fn _assets(&self) -> Vec<AccountId> {
            RegistryRef::asset_list(&self.registry)
        }

        fn _pool(&self, asset: AccountId) -> PoolData {
            if let Some(pool_addr) = RegistryRef::pool(&self.registry, asset) {
                let pool: AssetPoolContractRef = FromAccountId::from_account_id(pool_addr);
                return pool.pool_data()
            }
            panic!()
        }
    }
}
