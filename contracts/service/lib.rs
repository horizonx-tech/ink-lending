#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod service {
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use logics::{
        service::*,
        traits::service::*,
    };
    use openbrush::traits::Storage;

    #[ink(event)]
    pub struct Deposited {
        #[ink(topic)]
        asset: AccountId,
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        from: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Withdrew {
        #[ink(topic)]
        asset: AccountId,
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        to: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Borrowed {
        #[ink(topic)]
        asset: AccountId,
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        to: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Repaid {
        #[ink(topic)]
        asset: AccountId,
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        from: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Liquidated {
        #[ink(topic)]
        liquidator: AccountId,
        #[ink(topic)]
        liquidatee: AccountId,
        collateral_asset: AccountId,
        collateral_amount: Balance,
        #[ink(topic)]
        debt_asset: AccountId,
        debt_amount: Balance,
    }

    #[ink(storage)]
    #[derive(Storage)]
    pub struct ServiceContract {
        #[storage_field]
        service: Data,
    }

    impl ServiceContract {
        #[ink(constructor)]
        pub fn new(registry: AccountId) -> Self {
            Self {
                service: Data { registry },
            }
        }
    }

    impl Service for ServiceContract {}

    impl Internal for ServiceContract {
        fn _emit_deposited_event(
            &self,
            asset: AccountId,
            account: AccountId,
            from: AccountId,
            amount: Balance,
        ) {
            self.env().emit_event(Deposited {
                asset,
                account,
                from,
                amount,
            })
        }

        fn _emit_withdrew_event(
            &self,
            asset: AccountId,
            account: AccountId,
            to: AccountId,
            amount: Balance,
        ) {
            self.env().emit_event(Withdrew {
                asset,
                account,
                to,
                amount,
            })
        }

        fn _emit_borrowed_event(
            &self,
            asset: AccountId,
            account: AccountId,
            to: AccountId,
            amount: Balance,
        ) {
            self.env().emit_event(Borrowed {
                asset,
                account,
                to,
                amount,
            })
        }

        fn _emit_repaid_event(
            &self,
            asset: AccountId,
            account: AccountId,
            from: AccountId,
            amount: Balance,
        ) {
            self.env().emit_event(Repaid {
                asset,
                account,
                from,
                amount,
            })
        }

        fn _emit_liquidated_event(
            &self,
            liquidator: AccountId,
            liquidatee: AccountId,
            collateral_asset: AccountId,
            collateral_amount: Balance,
            debt_asset: AccountId,
            debt_amount: Balance,
        ) {
            self.env().emit_event(Liquidated {
                liquidator,
                liquidatee,
                collateral_asset,
                collateral_amount,
                debt_asset,
                debt_amount,
            })
        }
    }
}
