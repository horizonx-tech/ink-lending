#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod strategies {
    use logics::traits::risk_strategy::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct DefaultRiskStrategyContract {}

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
        pub fn new() -> Self {
            Self::default()
        }
    }
}
