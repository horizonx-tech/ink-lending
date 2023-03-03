#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod manager {
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use logics::{
        manager::*,
        traits::manager::*,
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct ManagerContract {
        #[storage_field]
        manager: Data,
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

    impl Manager for ManagerContract {}

    impl Internal for ManagerContract {
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

    impl ManagerContract {
        #[ink(constructor)]
        pub fn new(pool_admin: AccountId, emergency_admin: AccountId) -> Self {
            let instance = Self {
                manager: Data {
                    pool_admin,
                    emergency_admin,
                },
            };
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
}
