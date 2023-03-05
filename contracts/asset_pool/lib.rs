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
    #[derive(Storage)]
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
            Self {
                asset_pool: Data {
                    registry,
                    asset,
                    collateral_token,
                    debt_token,
                    liquidity_index: 0,
                    liquidity_rate: 0,
                    debt_index: 0,
                    debt_rate: 0,
                    last_update_timestamp: 0,
                },
            }
        }
    }
}
