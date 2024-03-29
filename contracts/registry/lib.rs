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
    pub struct ManagerChanged {
        manager: AccountId,
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

    #[ink(event)]
    pub struct PriceOracleChanged {
        price_oracle: AccountId,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct RegistryContract {
        #[storage_field]
        registry: Data,
    }

    impl RegistryContract {
        #[ink(constructor)]
        pub fn new(manager: Option<AccountId>) -> Self {
            let mut instance = Self::default();
            instance._set_manager(manager.unwrap_or(Self::env().caller())).unwrap();
            instance
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

        fn _emit_manager_changed_event(&self, manager: AccountId) {
            self.env().emit_event(ManagerChanged { manager });
        }

        fn _emit_rate_strategy_changed_event(&self, strategy: AccountId, asset: Option<AccountId>) {
            self.env()
                .emit_event(RateStrategyChanged { strategy, asset });
        }

        fn _emit_risk_strategy_changed_event(&self, strategy: AccountId, asset: Option<AccountId>) {
            self.env()
                .emit_event(RiskStrategyChanged { strategy, asset });
        }

        fn _emit_price_oracle_changed_event(&self, price_oracle: AccountId) {
            self.env().emit_event(PriceOracleChanged { price_oracle });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{
            test::{
                self,
                recorded_events,
                DefaultAccounts,
                EmittedEvent,
            },
            DefaultEnvironment,
        };
        use logics::traits::registry::Error;

        type Event = <RegistryContract as ink::reflect::ContractEventBase>::Type;

        fn default_accounts() -> DefaultAccounts<DefaultEnvironment> {
            test::default_accounts::<DefaultEnvironment>()
        }
        fn set_caller(id: AccountId) {
            test::set_caller::<DefaultEnvironment>(id);
        }
        fn get_emitted_events() -> Vec<EmittedEvent> {
            recorded_events().collect::<Vec<_>>()
        }
        fn decode_pool_registered_event(event: EmittedEvent) -> PoolRegistered {
            if let Ok(Event::PoolRegistered(x)) =
                <Event as scale::Decode>::decode(&mut &event.data[..])
            {
                return x
            }
            panic!("unexpected event kind: expected PoolRegistered event")
        }
        fn decode_factory_changed_event(event: EmittedEvent) -> FactoryChanged {
            if let Ok(Event::FactoryChanged(x)) =
                <Event as scale::Decode>::decode(&mut &event.data[..])
            {
                return x
            }
            panic!("unexpected event kind: expected FactoryChanged event")
        }
        fn decode_manager_changed_event(event: EmittedEvent) -> ManagerChanged {
            if let Ok(Event::ManagerChanged(x)) =
                <Event as scale::Decode>::decode(&mut &event.data[..])
            {
                return x
            }
            panic!("unexpected event kind: expected ManagerChanged event")
        }
        fn decode_risk_strategy_changed_event(event: EmittedEvent) -> RiskStrategyChanged {
            if let Ok(Event::RiskStrategyChanged(x)) =
                <Event as scale::Decode>::decode(&mut &event.data[..])
            {
                return x
            }
            panic!("unexpected event kind: expected RiskStrategyChanged event")
        }
        fn decode_rate_strategy_changed_event(event: EmittedEvent) -> RateStrategyChanged {
            if let Ok(Event::RateStrategyChanged(x)) =
                <Event as scale::Decode>::decode(&mut &event.data[..])
            {
                return x
            }
            panic!("unexpected event kind: expected RateStrategyChanged event")
        }
        fn decode_price_oracle_changed_event(event: EmittedEvent) -> PriceOracleChanged {
            if let Ok(Event::PriceOracleChanged(x)) =
                <Event as scale::Decode>::decode(&mut &event.data[..])
            {
                return x
            }
            panic!("unexpected event kind: expected PriceOracleChanged event")
        }

        #[ink::test]
        fn default_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let default_account_id = [0u8; 32].into();
            let contract = RegistryContract::default();

            assert_eq!(contract.factory(), default_account_id);
            assert_eq!(contract.manager(), default_account_id);
            assert_eq!(contract.asset_list(), vec![]);
            assert_eq!(contract.assets_count(), 0);
            assert_eq!(contract.default_rate_strategy(), default_account_id);
            assert_eq!(contract.default_risk_strategy(), default_account_id);
        }

        #[ink::test]
        fn new_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let manager = AccountId::from([0xf1; 32]);
            let default_account_id = [0u8; 32].into();
            let contract = RegistryContract::new(Some(manager));

            assert_eq!(contract.factory(), default_account_id);
            assert_eq!(contract.manager(), manager);
            assert_eq!(contract.default_rate_strategy(), default_account_id);
            assert_eq!(contract.default_risk_strategy(), default_account_id);
        }

        #[ink::test]
        fn registry_pool_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.bob));
            assert!(contract.set_factory(accounts.bob).is_ok());

            let asset1 = AccountId::from([0xa1; 32]);
            let asset2 = AccountId::from([0xa2; 32]);
            assert_eq!(contract.pool(asset1), None);
            assert_eq!(contract.pool(asset2), None);

            // first
            let pool1 = AccountId::from([0xf1; 32]);
            assert!(contract.register_pool(asset1, pool1).is_ok());
            assert_eq!(contract.pool(asset1), Some(pool1));
            assert_eq!(contract.assets_count(), 1);
            assert_eq!(contract.asset_list(), vec![asset1]);
            // second
            let pool2 = AccountId::from([0xf2; 32]);
            assert!(contract.register_pool(asset2, pool2).is_ok());
            assert_eq!(contract.pool(asset2), Some(pool2));
            assert_eq!(contract.assets_count(), 2);
            assert_eq!(contract.asset_list(), vec![asset1, asset2]);

            // duplicated
            assert_eq!(
                contract
                    .register_pool(asset1, AccountId::from([0xee; 32]))
                    .unwrap_err(),
                Error::PoolAlreadyExists
            );
        }

        #[ink::test]
        fn registry_pool_works_cannot_by_not_factory() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.bob));
            assert_eq!(contract.factory(), AccountId::from([0x00; 32]));

            assert_eq!(
                contract
                    .register_pool(AccountId::from([0x00; 32]), AccountId::from([0x00; 32]))
                    .unwrap_err(),
                Error::CallerIsNotFactory
            );
        }

        #[ink::test]
        fn registry_pool_works_event() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.bob));
            assert!(contract.set_factory(accounts.bob).is_ok());

            let asset = AccountId::from([0xaa; 32]);
            let pool = AccountId::from([0xff; 32]);
            assert!(contract.register_pool(asset, pool).is_ok());

            let event = decode_pool_registered_event(get_emitted_events()[1].clone());
            assert_eq!(event.asset, asset);
            assert_eq!(event.pool, pool);
        }

        #[ink::test]
        fn set_factory_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.bob));

            let factory = AccountId::from([0xaa; 32]);
            assert!(contract.set_factory(factory).is_ok());
            assert_eq!(contract.factory(), factory);

            let event = decode_factory_changed_event(get_emitted_events()[0].clone());
            assert_eq!(event.factory, factory);
        }

        #[ink::test]
        fn set_factory_works_cannot_by_not_manager() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::default();

            let factory = AccountId::from([0xaa; 32]);
            assert_eq!(
                contract.set_factory(factory).unwrap_err(),
                Error::CallerIsNotManager
            );
        }

        #[ink::test]
        fn set_manager_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.charlie));

            set_caller(accounts.charlie);
            assert!(contract.set_manager(accounts.alice).is_ok());
            assert_eq!(contract.manager(), accounts.alice);

            let event = decode_manager_changed_event(get_emitted_events()[0].clone());
            assert_eq!(event.manager, accounts.alice);
        }

        #[ink::test]
        fn set_manager_works_cannot_by_not_manager() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let mut contract = RegistryContract::default();
            assert_eq!(
                contract.set_manager(accounts.alice).unwrap_err(),
                Error::CallerIsNotManager
            );
        }

        #[ink::test]
        fn set_risk_strategy_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.bob));

            let asset = AccountId::from([0xaa; 32]);
            let default_strategy = contract.default_risk_strategy();
            assert_eq!(contract.risk_strategy(asset), default_strategy);

            let strategy = AccountId::from([0xff; 32]);
            assert!(contract.set_risk_strategy(strategy, Some(asset)).is_ok());
            assert_eq!(contract.risk_strategy(asset), strategy);
        }

        #[ink::test]
        fn set_risk_strategy_works_cannot_by_not_manager() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.bob));

            let asset = AccountId::from([0xaa; 32]);
            let strategy = AccountId::from([0xff; 32]);
            set_caller(accounts.charlie);
            assert_eq!(
                contract
                    .set_risk_strategy(strategy, Some(asset))
                    .unwrap_err(),
                Error::CallerIsNotManager
            );
            assert_ne!(contract.risk_strategy(asset), strategy)
        }

        #[ink::test]
        fn set_risk_strategy_works_event() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.bob));

            let asset = AccountId::from([0xa1; 32]);
            let strategy = AccountId::from([0xf1; 32]);
            let default_strategy = AccountId::from([0xff; 32]);
            assert!(contract.set_risk_strategy(strategy, Some(asset)).is_ok());
            assert!(contract.set_risk_strategy(default_strategy, None).is_ok());
            let event = decode_risk_strategy_changed_event(get_emitted_events()[0].clone());
            assert_eq!(event.asset, Some(asset));
            assert_eq!(event.strategy, strategy);
            let event = decode_risk_strategy_changed_event(get_emitted_events()[1].clone());
            assert_eq!(event.asset, None);
            assert_eq!(event.strategy, default_strategy);
        }

        #[ink::test]
        fn set_rate_strategy_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.bob));

            let asset = AccountId::from([0xaa; 32]);
            let default_strategy = contract.default_rate_strategy();
            assert_eq!(contract.rate_strategy(asset), default_strategy);

            let strategy = AccountId::from([0xff; 32]);
            assert!(contract.set_rate_strategy(strategy, Some(asset)).is_ok());
            assert_eq!(contract.rate_strategy(asset), strategy);
        }

        #[ink::test]
        fn set_rate_strategy_works_cannot_by_not_manager() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.bob));

            let asset = AccountId::from([0xaa; 32]);
            let strategy = AccountId::from([0xff; 32]);
            set_caller(accounts.charlie);
            assert_eq!(
                contract
                    .set_rate_strategy(strategy, Some(asset))
                    .unwrap_err(),
                Error::CallerIsNotManager
            );
            assert_ne!(contract.rate_strategy(asset), strategy)
        }

        #[ink::test]
        fn set_rate_strategy_works_event() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.bob));

            let asset = AccountId::from([0xa1; 32]);
            let strategy = AccountId::from([0xf1; 32]);
            let default_strategy = AccountId::from([0xff; 32]);
            assert!(contract.set_rate_strategy(strategy, Some(asset)).is_ok());
            assert!(contract.set_rate_strategy(default_strategy, None).is_ok());
            let event = decode_rate_strategy_changed_event(get_emitted_events()[0].clone());
            assert_eq!(event.asset, Some(asset));
            assert_eq!(event.strategy, strategy);
            let event = decode_rate_strategy_changed_event(get_emitted_events()[1].clone());
            assert_eq!(event.asset, None);
            assert_eq!(event.strategy, default_strategy);
        }

        #[ink::test]
        fn set_price_oracle_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::new(Some(accounts.charlie));

            set_caller(accounts.charlie);
            assert!(contract.set_price_oracle(accounts.alice).is_ok());
            assert_eq!(contract.price_oracle(), accounts.alice);

            let event = decode_price_oracle_changed_event(get_emitted_events()[0].clone());
            assert_eq!(event.price_oracle, accounts.alice);
        }

        #[ink::test]
        fn set_price_oracle_works_cannot_by_not_manager() {
            let accounts = default_accounts();
            set_caller(accounts.bob);
            let mut contract = RegistryContract::default();

            assert_eq!(
                contract.set_price_oracle(accounts.alice).unwrap_err(),
                Error::CallerIsNotManager
            );
        }
    }
}
