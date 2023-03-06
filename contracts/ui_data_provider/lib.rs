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
    use logics::traits::{
        price_oracle::PriceOracleRef,
        registry::RegistryRef,
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
            let pools: Vec<PoolData> = self._pools(&(assets));
            let prices = self._prices(&(assets));

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
                        // TODO
                        loan_to_value: 0,
                        liquidation_threshold: 0,
                    }
                })
                .collect()
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
                let pool: AssetPoolContractRef = FromAccountId::from_account_id(pool_addr);
                return pool.pool_data()
            }
            panic!()
        }
    }
}
