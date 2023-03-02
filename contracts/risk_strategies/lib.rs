#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod strategies {
    use logics::traits::{
        price_oracle::PriceOracleRef,
        risk_strategy::*,
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct DefaultRiskStrategyContract {
        price_oracle: AccountId,
    }

    impl RiskStrategy for DefaultRiskStrategyContract {
        #[ink(message)]
        fn price_oracle(&self) -> AccountId {
            self.price_oracle
        }

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
        pub fn new(price_oracle: Option<AccountId>) -> Self {
            Self {
                price_oracle: price_oracle.unwrap_or(AccountId::from([0u8; 32])),
            }
        }

        // TODO permission
        #[ink(message)]
        pub fn set_price_oracle(&mut self, price_oracle: AccountId) {
            self.price_oracle = price_oracle;
        }
    }

    impl DefaultRiskStrategyContract {
        fn _asset_price(&self, asset: AccountId) -> u128 {
            PriceOracleRef::get(&self.price_oracle, asset)
        }
    }
}
