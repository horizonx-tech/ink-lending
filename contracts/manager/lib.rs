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
        contracts::ownable::Internal as OwnableInternal,
        modifiers,
        traits::Storage,
    };

    #[ink(storage)]
    #[derive(Storage)]
    pub struct ManagerContract {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        manager: manager::Data,
    }

    #[ink(event)]
    pub struct OwnershipTransferred {
        previous: Option<AccountId>,
        new: Option<AccountId>,
    }

    #[ink(event)]
    pub struct PoolAdminOwnershipTransferred {
        previous: Option<AccountId>,
        new: Option<AccountId>,
    }

    #[ink(event)]
    pub struct EmergencyAdminOwnershipTransferred {
        previous: Option<AccountId>,
        new: Option<AccountId>,
    }

    impl Manager for ManagerContract {
        #[ink(message)]
        #[modifiers(ownable::only_owner)]
        fn set_pool_admin(&mut self, id: AccountId) -> Result<()> {
            self._set_pool_admin(id)
        }

        #[ink(message)]
        #[modifiers(ownable::only_owner)]
        fn set_emergency_admin(&mut self, id: AccountId) -> Result<()> {
            self._set_emergency_admin(id)
        }
    }
    impl ownable::Ownable for ManagerContract {}

    impl manager::Internal for ManagerContract {
        fn _emit_pool_admin_ownership_transferred_event(
            &self,
            previous: Option<AccountId>,
            new: Option<AccountId>
        ) {
            self.env()
                .emit_event(PoolAdminOwnershipTransferred { previous, new });
        }

        fn _emit_emergency_admin_ownership_transferred_event(
            &self,
            previous: Option<AccountId>,
            new: Option<AccountId>
        ) {
            self.env()
                .emit_event(EmergencyAdminOwnershipTransferred { previous, new });
        }
    }

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

    impl ManagerContract {
        #[ink(constructor)]
        pub fn new(pool_admin: AccountId, emergency_admin: AccountId) -> Self {
            let mut instance = Self {
                ownable: ownable::Data::default(),
                manager: manager::Data {
                    pool_admin,
                    emergency_admin,
                },
            };
            instance._init_with_owner(Self::env().caller());
            instance._emit_pool_admin_ownership_transferred_event(
                None,
                Some(pool_admin)
            );
            instance._emit_emergency_admin_ownership_transferred_event(
                None,
                Some(emergency_admin)
            );
            instance
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env;
        use openbrush::contracts::ownable::Ownable;
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
        fn decode_ownership_transferred_event(event: env::test::EmittedEvent) -> OwnershipTransferred {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..]);
            match decoded_event {
                Ok(Event::OwnershipTransferred(x)) => return x,
                _ => panic!("unexpected event kind: expected OwnershipTransferred event")
            }
        }
        fn decode_pool_admin_ownership_transferred_event(event: env::test::EmittedEvent) -> PoolAdminOwnershipTransferred {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..]);
            match decoded_event {
                Ok(Event::PoolAdminOwnershipTransferred(x)) => return x,
                _ => panic!("unexpected event kind: expected PoolAdminOwnershipTransferred event")
            }
        }
        fn decode_emergency_admin_ownership_transferred_event(event: env::test::EmittedEvent) -> EmergencyAdminOwnershipTransferred {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..]);
            match decoded_event {
                Ok(Event::EmergencyAdminOwnershipTransferred(x)) => return x,
                _ => panic!("unexpected event kind: expected EmergencyAdminOwnershipTransferred event")
            }
        }

        #[ink::test]
        fn new_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let pool_admin = AccountId::from([0xf0; 32]);
            let emergency_admin = AccountId::from([0xf1; 32]);
            let contract = ManagerContract::new(
                pool_admin,
                emergency_admin
            );
            assert_eq!(contract.owner(), accounts.bob);
            assert_eq!(contract.pool_admin(), pool_admin);
            assert_eq!(contract.emergency_admin(), emergency_admin);

            let events = get_emitted_events();
            assert_eq!(events.len(), 3);
            let event = decode_ownership_transferred_event(events[0].clone());
            assert_eq!(event.previous, None);
            assert_eq!(event.new, Some(accounts.bob));
            let event = decode_pool_admin_ownership_transferred_event(events[1].clone());
            assert_eq!(event.previous, None);
            assert_eq!(event.new, Some(pool_admin));
            let event = decode_emergency_admin_ownership_transferred_event(events[2].clone());
            assert_eq!(event.previous, None);
            assert_eq!(event.new, Some(emergency_admin));
        }

        #[ink::test]
        fn set_pool_admin_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let pool_admin = AccountId::from([0x00; 32]);
            let mut contract = ManagerContract::new(
                pool_admin,
                AccountId::from([0x00; 32])
            );

            let new_pool_admin = AccountId::from([0xff; 32]);
            assert!(contract.set_pool_admin(new_pool_admin).is_ok());

            assert_eq!(contract.pool_admin(), new_pool_admin);

            let event = decode_pool_admin_ownership_transferred_event(get_emitted_events()[3].clone());
            assert_eq!(event.previous, Some(pool_admin));
            assert_eq!(event.new, Some(new_pool_admin));
        }


        #[ink::test]
        fn set_pool_admin_works_cannot_by_not_owner() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let previous_pool_admin = AccountId::from([0x00; 32]);
            let mut contract = ManagerContract::new(
                previous_pool_admin,
                AccountId::from([0x00; 32])
            );

            set_caller(accounts.charlie);
            assert_eq!(
                contract.set_pool_admin(AccountId::from([0xff; 32])).unwrap_err(),
                Error::OwnableError(ownable::OwnableError::CallerIsNotOwner)
            );
            assert_eq!(contract.pool_admin(), previous_pool_admin);
        }

        #[ink::test]
        fn set_emergency_admin_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let emergency_admin = AccountId::from([0x00; 32]);
            let mut contract = ManagerContract::new(
                AccountId::from([0xff; 32]),
                emergency_admin
            );

            let new_emergency_admin = AccountId::from([0xff; 32]);
            assert!(contract.set_emergency_admin(new_emergency_admin).is_ok());

            assert_eq!(contract.emergency_admin(), new_emergency_admin);

            let event = decode_emergency_admin_ownership_transferred_event(get_emitted_events()[3].clone());
            assert_eq!(event.previous, Some(emergency_admin));
            assert_eq!(event.new, Some(new_emergency_admin));
        }


        #[ink::test]
        fn set_emergency_admin_works_cannot_by_not_owner() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let previous_emergency_admin = AccountId::from([0x00; 32]);
            let mut contract = ManagerContract::new(
                AccountId::from([0xff; 32]),
                previous_emergency_admin
            );

            set_caller(accounts.charlie);
            assert_eq!(
                contract.set_emergency_admin(AccountId::from([0xff; 32])).unwrap_err(),
                Error::OwnableError(ownable::OwnableError::CallerIsNotOwner)
            );
            assert_eq!(contract.emergency_admin(), previous_emergency_admin);
        }
    }
}
