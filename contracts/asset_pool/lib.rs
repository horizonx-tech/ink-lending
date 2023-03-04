#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pool {
    use logics::{
        asset_pool::*,
        traits::asset_pool::*,
    };
    use openbrush::traits::Storage;
    use scale::{
        Decode,
        Encode,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct AssetPoolContract {
        #[storage_field]
        asset_pool: Data,
    }

    #[derive(Decode, Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct PoolData {
        pub registry: AccountId,
        pub asset: AccountId,
        pub collateral_token: AccountId,
        pub debt_token: AccountId,
        pub liquidity_index: u128,
        pub liquidity_rate: u128,
        pub debt_index: u128,
        pub debt_rate: u128,
        pub last_update_timestamp: Timestamp,
    }

    impl From<Data> for PoolData {
        fn from(item: Data) -> PoolData {
            PoolData {
                registry: item.registry,
                asset: item.asset,
                collateral_token: item.collateral_token,
                debt_token: item.debt_token,
                liquidity_index: item.liquidity_index,
                liquidity_rate: item.liquidity_rate,
                debt_index: item.debt_index,
                debt_rate: item.debt_rate,
                last_update_timestamp: item.last_update_timestamp,
            }
        }
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

        #[ink(message)]
        pub fn pool_data(&self) -> PoolData {
            self._pool_data().into()
        }
    }
}
