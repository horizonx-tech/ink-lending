#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod registry {
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use logics::{
        registry::*,
        traits::registry::*,
    };
    use openbrush::traits::Storage;

    #[ink(event)]
    pub struct PoolRegistered {
        #[ink(topic)]
        asset: AccountId,
        pool: AccountId,
    }

    #[ink(event)]
    pub struct FactoryChanged {
        factory: AccountId,
    }

    #[ink(event)]
    pub struct RateStrategyChanged {
        #[ink(topic)]
        asset: Option<AccountId>,
        strategy: AccountId,
    }

    #[ink(event)]
    pub struct RiskStrategyChanged {
        #[ink(topic)]
        asset: Option<AccountId>,
        strategy: AccountId,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct RegistryContract {
        #[storage_field]
        registry: Data,
    }

    impl RegistryContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl Registry for RegistryContract {}

    impl Internal for RegistryContract {
        fn _emit_pool_registered_event(&self, asset: AccountId, pool: AccountId) {
            self.env().emit_event(PoolRegistered { asset, pool });
        }

        fn _emit_factory_changed_event(&self, factory: AccountId) {
            self.env().emit_event(FactoryChanged { factory });
        }

        fn _emit_rate_strategy_changed_event(&self, strategy: AccountId, asset: Option<AccountId>) {
            self.env()
                .emit_event(RateStrategyChanged { strategy, asset });
        }

        fn _emit_risk_strategy_changed_event(&self, strategy: AccountId, asset: Option<AccountId>) {
            self.env()
                .emit_event(RiskStrategyChanged { strategy, asset });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{
            test::{
                self,
                DefaultAccounts,
            },
            DefaultEnvironment,
        };

        fn default_accounts() -> DefaultAccounts<DefaultEnvironment> {
            test::default_accounts::<DefaultEnvironment>()
        }
        fn set_caller(id: AccountId) {
            test::set_caller::<DefaultEnvironment>(id);
        }

        #[ink::test]
        fn new_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let defaultAccountId = [0u8; 32].into();
            let contract = RegistryContract::default();

            assert_eq!(contract.factory(), defaultAccountId);
            assert_eq!(contract.default_rate_strategy(), defaultAccountId);
            assert_eq!(contract.default_risk_strategy(), defaultAccountId);
        }
    }
}
