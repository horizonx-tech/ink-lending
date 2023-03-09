#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[ink::contract]
pub mod ui_data_provider {
    use ink::prelude::vec::Vec;
    use logics::{
        traits::{
            price_oracle::PriceOracleRef,
            registry::RegistryRef,
        },
        ui_data_providers::pool_data_provider::{
            AccountData,
            PoolData,
            UIPoolDataProviderRef,
        },
    };
    use scale::{
        Decode,
        Encode,
    };

    #[ink(storage)]
    pub struct UIDataProvider {
        registry: AccountId,
    }

    #[derive(Decode, Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct MarketData {
        asset: AccountId,
        price: u128,
        liquidity_share: Balance,
        liquidity_index: Balance,
        liquidity_interest_rate: u128,
        debt_share: Balance,
        debt_index: Balance,
        debt_interest_rate: u128,
        pool_last_update_timestamp: Timestamp,
        loan_to_value: u64,
        liquidation_threshold: u64,
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
        pub fn pools(&self, assets: Option<Vec<AccountId>>) -> Vec<PoolData> {
            self._pools(&(assets.unwrap_or(self._assets())))
        }

        #[ink(message)]
        pub fn market_data(&self, _assets: Option<Vec<AccountId>>) -> Vec<MarketData> {
            let assets = _assets.unwrap_or(self._assets());
            let pools: Vec<PoolData> = self._pools(&assets);
            let prices = self._prices(&assets);

            assets
                .iter()
                .enumerate()
                .map(|(index, asset)| {
                    let pool: &PoolData = &pools[index];
                    MarketData {
                        asset: *asset,
                        price: prices[index],
                        liquidity_share: pool.liquidity_share,
                        liquidity_index: pool.liquidity_index,
                        liquidity_interest_rate: pool.liquidity_rate,
                        debt_share: pool.debt_share,
                        debt_index: pool.debt_index,
                        debt_interest_rate: pool.debt_rate,
                        pool_last_update_timestamp: pool.last_update_timestamp,
                        // TODO
                        loan_to_value: 0,
                        liquidation_threshold: 0,
                    }
                })
                .collect()
        }

        #[ink(message)]
        pub fn account_data(
            &self,
            _account: Option<AccountId>,
            _assets: Option<Vec<AccountId>>,
        ) -> Vec<AccountData> {
            let account = _account.unwrap_or(Self::env().caller());
            let assets = _assets.unwrap_or(self._assets());
            self._pools_account_data(&assets, account)
        }

        fn _assets(&self) -> Vec<AccountId> {
            RegistryRef::asset_list(&self.registry)
        }

        fn _prices(&self, assets: &Vec<AccountId>) -> Vec<u128> {
            let price_oracle = RegistryRef::price_oracle(&self.registry);
            assets
                .iter()
                .map(|asset| self._price(&price_oracle, asset))
                .collect()
        }

        fn _price(&self, price_oracle: &AccountId, asset: &AccountId) -> u128 {
            PriceOracleRef::get(price_oracle, *asset)
        }

        fn _pools(&self, assets: &Vec<AccountId>) -> Vec<PoolData> {
            assets.iter().map(|asset| self._pool(*asset)).collect()
        }

        fn _pool(&self, asset: AccountId) -> PoolData {
            if let Some(pool_addr) = RegistryRef::pool(&self.registry, asset) {
                return UIPoolDataProviderRef::pool_data(&pool_addr)
            }
            panic!()
        }

        fn _pools_account_data(
            &self,
            assets: &Vec<AccountId>,
            account: AccountId,
        ) -> Vec<AccountData> {
            assets
                .iter()
                .map(|asset| self._pool_account_data(*asset, account))
                .collect()
        }

        fn _pool_account_data(&self, asset: AccountId, account: AccountId) -> AccountData {
            if let Some(pool_addr) = RegistryRef::pool(&self.registry, asset) {
                return UIPoolDataProviderRef::account_data(&pool_addr, account)
            }
            panic!()
        }
    }
}
