#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pool {
    use logics::{
        asset_pool::*,
        traits::asset_pool::*,
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct AssetPoolContract {
        #[storage_field]
        asset_pool: Data,
    }

    impl AssetPool for AssetPoolContract {}

    impl AssetPoolContract {
        #[ink(constructor)]
        pub fn new(
            registry: AccountId,
            asset: AccountId,
            collateral_token: AccountId,
            debt_token: AccountId,
        ) -> Self {
            let mut instance = Self::default();
            instance.asset_pool.registry = registry;
            instance.asset_pool.asset = asset;
            instance.asset_pool.collateral_token = collateral_token;
            instance.asset_pool.debt_token = debt_token;

            instance
        }
        }
    }
}
