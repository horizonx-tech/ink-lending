#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod strategies {
    use logics::traits::{
        price_oracle::PriceOracleRef,
        registry::RegistryRef,
        risk_strategy::*,
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct DefaultRiskStrategyContract {
        registry: AccountId,
    }

    impl RiskStrategy for DefaultRiskStrategyContract {
        #[ink(message)]
        fn validate_borrow(
            &self,
            _account: AccountId,
            _asset: AccountId,
            _amount: Balance,
        ) -> Result<()> {
            // TODO
            Ok(())
        }

        #[ink(message)]
        fn validate_withdraw(
            &self,
            _account: AccountId,
            _asset: AccountId,
            _amount: Balance,
        ) -> Result<()> {
            // TODO
            Ok(())
        }

        #[ink(message)]
        fn validate_liquidation(
            &self,
            _liquidatee: AccountId,
            _collateral_asset: AccountId,
            _debt_asset: AccountId,
            _debt_amount: Balance,
        ) -> Result<Balance> {
            // TODO receivable_amount
            Ok(u128::MAX)
        }
    }

    impl DefaultRiskStrategyContract {
        #[ink(constructor)]
        pub fn new(registry: AccountId) -> Self {
            Self { registry }
        }
    }

    impl DefaultRiskStrategyContract {
        fn _asset_price(&self, asset: AccountId) -> u128 {
            PriceOracleRef::get(&RegistryRef::price_oracle(&self.registry), asset)
        }
    }
}
