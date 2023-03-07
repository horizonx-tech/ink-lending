#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod dummy_pool {
    use logics::{
        asset_pool::*,
        traits::asset_pool::*,
        ui_data_providers::pool_data_provider,
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct AssetPoolContract {
        #[storage_field]
        asset_pool: Data,
    }

    impl AssetPool for AssetPoolContract {}
    impl pool_data_provider::UIPoolDataProvider for AssetPoolContract {}

    impl Internal for AssetPoolContract {
        fn _deposit(
            &mut self,
            _account: AccountId,
            _from: AccountId,
            _amount: Balance,
        ) -> Result<()> {
            Ok(())
        }

        fn _withdraw(
            &mut self,
            _account: AccountId,
            _to: AccountId,
            _amount: Balance,
        ) -> Result<()> {
            Ok(())
        }

        fn _borrow(&mut self, _account: AccountId, _to: AccountId, _amount: Balance) -> Result<()> {
            Ok(())
        }

        fn _repay(
            &mut self,
            _account: AccountId,
            _from: AccountId,
            _amount: Balance,
        ) -> Result<()> {
            Ok(())
        }

        fn _transfer_collateral_on_liquidation(
            &mut self,
            _liquidatee: AccountId,
            _receiver: AccountId,
            _amount: Balance,
        ) -> Result<()> {
            Ok(())
        }
    }

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
        #[ink(message)]
        pub fn set_values(
            &mut self,
            registry: Option<AccountId>,
            asset: Option<AccountId>,
            collateral_token: Option<AccountId>,
            debt_token: Option<AccountId>,
            liquidity_index: Option<u128>,
            liquidity_rate: Option<u128>,
            debt_index: Option<u128>,
            debt_rate: Option<u128>,
            last_update_timestamp: Option<Timestamp>,
        ) {
            if let Some(v) = registry {
                self.asset_pool.registry = v;
            };
            if let Some(v) = asset {
                self.asset_pool.asset = v;
            };
            if let Some(v) = collateral_token {
                self.asset_pool.collateral_token = v;
            };
            if let Some(v) = debt_token {
                self.asset_pool.debt_token = v;
            };
            if let Some(v) = liquidity_index {
                self.asset_pool.liquidity_index = v;
            };
            if let Some(v) = liquidity_rate {
                self.asset_pool.liquidity_rate = v;
            };
            if let Some(v) = debt_index {
                self.asset_pool.debt_index = v;
            };
            if let Some(v) = debt_rate {
                self.asset_pool.debt_rate = v;
            };
            if let Some(v) = last_update_timestamp {
                self.asset_pool.last_update_timestamp = v;
            };
        }
    }
}
