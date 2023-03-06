#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod manager {
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use logics::{
        manager,
        manager::Internal as ManagerInternal,
        traits::manager::*,
    };
    use openbrush::{
        contracts::ownable,
        contracts::access_control::{
            self,
            RoleType,
            DEFAULT_ADMIN_ROLE,
            Internal as AccessControlInternal
        },
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Storage)]
    pub struct ManagerContract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        manager: manager::Data,
    }

    #[ink(event)]
    pub struct OwnershipTransferred {
        previous: Option<AccountId>,
        new: Option<AccountId>,
    }

    #[ink(event)]
    pub struct RoleAdminChanged {
        #[ink(topic)]
        role: RoleType,
        #[ink(topic)]
        previous_admin_role: RoleType,
        #[ink(topic)]
        new_admin_role: RoleType,
    }

    #[ink(event)]
    pub struct RoleGranted {
        #[ink(topic)]
        role: RoleType,
        #[ink(topic)]
        grantee: AccountId,
        #[ink(topic)]
        grantor: Option<AccountId>,
    }

    #[ink(event)]
    pub struct RoleRevoked {
        #[ink(topic)]
        role: RoleType,
        #[ink(topic)]
        account: AccountId,
        #[ink(topic)]
        admin: AccountId,
    }

    impl Manager for ManagerContract {
        #[ink(message)]
        #[modifiers(access_control::only_role(DEFAULT_ADMIN_ROLE))]
        fn set_factory(&mut self, id: AccountId) -> Result<()> {
            self._set_factory(id)
        }

        #[ink(message)]
        #[modifiers(access_control::only_role(DEFAULT_ADMIN_ROLE))]
        fn set_registry(&mut self, id: AccountId) -> Result<()> {
            self._set_registry(id)
        }

        #[ink(message)]
        #[modifiers(access_control::only_role(DEFAULT_ADMIN_ROLE))] // temp
        fn create_pool(&mut self, asset: AccountId, data: Vec<u8>) -> Result<AccountId> {
            self._create_pool(asset, data.clone())
        }

        #[ink(message)]
        #[modifiers(access_control::only_role(DEFAULT_ADMIN_ROLE))] // temp
        fn update_rate_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()> {
            self._update_rate_strategy(address, asset)
        }
    
        #[ink(message)]
        #[modifiers(access_control::only_role(DEFAULT_ADMIN_ROLE))] // temp
        fn update_risk_strategy(&mut self, address: AccountId, asset: Option<AccountId>) -> Result<()> {
            self._update_risk_strategy(address, asset)
        }
    }
    impl ownable::Ownable for ManagerContract {}
    impl access_control::AccessControl for ManagerContract {}

    impl ownable::Internal for ManagerContract {
        fn _emit_ownership_transferred_event(
            &self,
            previous: Option<AccountId>,
            new: Option<AccountId>,
        ) {
            self.env()
                .emit_event(OwnershipTransferred { previous, new });
        }
    }

    impl access_control::Internal for ManagerContract {
        fn _emit_role_admin_changed(&mut self, role: u32, previous_admin_role: u32, new_admin_role: u32) {
            self.env().emit_event(RoleAdminChanged {
                role,
                previous_admin_role,
                new_admin_role,
            })
        }

        fn _emit_role_granted(&mut self, role: u32, grantee: AccountId, grantor: Option<AccountId>) {
            self.env().emit_event(RoleGranted { role, grantee, grantor })
        }

        fn _emit_role_revoked(&mut self, role: u32, account: AccountId, sender: AccountId) {
            self.env().emit_event(RoleRevoked {
                role,
                account,
                admin: sender,
            })
        }
    }

    impl ManagerContract {
        #[ink(constructor)]
        pub fn new(
            factory: AccountId,
            registry: AccountId
        ) -> Self {
            let mut instance = Self {
                ownable: ownable::Data::default(),
                access: access_control::Data::default(),
                manager: manager::Data {
                    factory,
                    registry
                },
            };
            instance._init_with_caller();
            instance
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env;
        use openbrush::contracts::access_control::AccessControl;
        use logics::traits::manager::Error;

        type Event = <ManagerContract as ink::reflect::ContractEventBase>::Type;

        fn default_accounts() -> env::test::DefaultAccounts<env::DefaultEnvironment> {
            env::test::default_accounts::<env::DefaultEnvironment>()
        }
        fn set_caller(id: AccountId) {
            env::test::set_caller::<env::DefaultEnvironment>(id);
        }
        fn get_emitted_events() -> Vec<env::test::EmittedEvent> {
            ink::env::test::recorded_events().collect::<Vec<_>>()
        }
        fn decode_role_granted_event(event: env::test::EmittedEvent) -> RoleGranted {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..]);
            match decoded_event {
                Ok(Event::RoleGranted(x)) => return x,
                _ => panic!("unexpected event kind: expected RoleGranted event")
            }
        }

        #[ink::test]
        fn new_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let factory = AccountId::from([0xf0; 32]);
            let registry = AccountId::from([0xf1; 32]);
            let contract = ManagerContract::new(
                factory,
                registry
            );
            assert!(contract.has_role(DEFAULT_ADMIN_ROLE, accounts.bob));
            assert_eq!(contract.factory(), factory);
            assert_eq!(contract.registry(), registry);

            let events = get_emitted_events();
            assert_eq!(events.len(), 1);
            let event = decode_role_granted_event(events[0].clone());
            assert_eq!(event.role, DEFAULT_ADMIN_ROLE);
            assert_eq!(event.grantee, accounts.bob);
            assert_eq!(event.grantor, None);
        }

        #[ink::test]
        fn set_factory_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let mut contract = ManagerContract::new(
                AccountId::from([0x00; 32]),
                AccountId::from([0x00; 32]),
            );

            let new_factory = AccountId::from([0xff; 32]);
            assert!(contract.set_factory(new_factory).is_ok());
            assert_eq!(contract.factory(), new_factory);
        }

        #[ink::test]
        fn set_factory_works_cannot_by_not_owner() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let previous_factory = AccountId::from([0x00; 32]);
            let mut contract = ManagerContract::new(
                previous_factory,
                AccountId::from([0x00; 32]),
            );

            set_caller(accounts.charlie);
            assert_eq!(
                contract.set_factory(AccountId::from([0xff; 32])).unwrap_err(),
                Error::AccessControlError(access_control::AccessControlError::MissingRole)
            );
            assert_eq!(contract.factory(), previous_factory);
        }

        #[ink::test]
        fn set_registry_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let mut contract = ManagerContract::new(
                AccountId::from([0x00; 32]),
                AccountId::from([0x00; 32]),
            );

            let new_registry = AccountId::from([0xff; 32]);
            assert!(contract.set_registry(new_registry).is_ok());
            assert_eq!(contract.registry(), new_registry);
        }

        #[ink::test]
        fn set_registry_works_cannot_by_not_owner() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let previous_registry = AccountId::from([0x00; 32]);
            let mut contract = ManagerContract::new(
                AccountId::from([0x00; 32]),
                previous_registry,
            );

            set_caller(accounts.charlie);
            assert_eq!(
                contract.set_registry(AccountId::from([0xff; 32])).unwrap_err(),
                Error::AccessControlError(access_control::AccessControlError::MissingRole)
            );
            assert_eq!(contract.registry(), previous_registry);
        }
    }
}
