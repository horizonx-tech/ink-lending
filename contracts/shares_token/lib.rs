#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod token {
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use logics::traits::shares::*;
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::extensions::{
                burnable::*,
                metadata::*,
                mintable::*,
            },
        },
        modifiers,
        traits::{
            Storage,
            String,
        },
    };

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[ink(event)]
    pub struct OwnershipTransferred {
        previous: Option<AccountId>,
        new: Option<AccountId>,
    }

    #[ink(storage)]
    #[derive(Storage)]
    pub struct SharesToken {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        asset: AccountId,
    }

    impl SharesToken {
        #[ink(constructor)]
        pub fn new(
            asset: AccountId,
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8,
        ) -> Self {
            let mut instance = Self {
                psp22: psp22::Data::default(),
                ownable: ownable::Data::default(),
                metadata: metadata::Data {
                    name,
                    symbol,
                    decimals,
                    _reserved: None,
                },
                asset,
            };
            instance._init_with_owner(Self::env().caller());

            instance
        }

        #[ink(message)]
        pub fn asset(&self) -> AccountId {
            self.asset
        }

        #[ink(message)]
        pub fn total_share(&self) -> Balance {
            self.psp22.supply.clone()
        }

        #[ink(message)]
        pub fn share_of(&self, owner: AccountId) -> Balance {
            self.psp22.balances.get(&owner).unwrap_or(0)
        }
    }

    impl Shares for SharesToken {}
    impl PSP22 for SharesToken {}
    impl PSP22Metadata for SharesToken {}
    impl PSP22Burnable for SharesToken {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._burn_from(account, amount)
        }
    }
    impl PSP22Mintable for SharesToken {
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            self._mint_to(account, amount)
        }
    }
    impl Ownable for SharesToken {}

    impl psp22::Internal for SharesToken {
        fn _emit_transfer_event(
            &self,
            from: Option<AccountId>,
            to: Option<AccountId>,
            value: Balance,
        ) {
            self.env().emit_event(Transfer { from, to, value });
        }

        fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, value: Balance) {
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
        }
    }

    impl ownable::Internal for SharesToken {
        fn _emit_ownership_transferred_event(
            &self,
            previous: Option<AccountId>,
            new: Option<AccountId>,
        ) {
            self.env()
                .emit_event(OwnershipTransferred { previous, new });
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
        use openbrush::contracts::traits::errors::{
            OwnableError,
            PSP22Error,
        };

        type Event = <SharesToken as ink::reflect::ContractEventBase>::Type;

        fn default_accounts() -> DefaultAccounts<DefaultEnvironment> {
            test::default_accounts::<DefaultEnvironment>()
        }
        fn set_caller(id: AccountId) {
            test::set_caller::<DefaultEnvironment>(id);
        }
        fn get_emitted_events() -> Vec<EmittedEvent> {
            recorded_events().collect::<Vec<_>>()
        }
        fn decode_transfer_event(event: EmittedEvent) -> Transfer {
            if let Ok(Event::Transfer(x)) = <Event as scale::Decode>::decode(&mut &event.data[..]) {
                return x
            }
            panic!("unexpected event kind: expected Transfer event")
        }
        fn decode_ownership_transferred_event(event: EmittedEvent) -> OwnershipTransferred {
            if let Ok(Event::OwnershipTransferred(x)) =
                <Event as scale::Decode>::decode(&mut &event.data[..])
            {
                return x
            }
            panic!("unexpected event kind: expected OwnershipTransferred event")
        }

        #[ink::test]
        fn new_works() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let asset = AccountId::from([0xff; 32]);
            let contract = SharesToken::new(
                asset.clone(),
                Some(String::from("share coin")),
                Some(String::from("sCOIN")),
                8,
            );
            assert_eq!(contract.asset(), asset.clone());
            assert_eq!(contract.token_name().unwrap(), String::from("share coin"));
            assert_eq!(contract.token_symbol().unwrap(), String::from("sCOIN"));
            assert_eq!(contract.token_decimals(), 8);
            assert_eq!(contract.total_supply(), 0);
            assert_eq!(contract.owner(), accounts.bob);

            // emit event
            let events = get_emitted_events();
            assert_eq!(events.len(), 1);
            let event = decode_ownership_transferred_event(events[0].clone());
            assert_eq!(event.previous, None);
            assert_eq!(event.new, Some(accounts.bob));
        }

        #[ink::test]
        fn mint_works() {
            let accounts = default_accounts();
            let alice = accounts.alice;
            let bob = accounts.bob;
            set_caller(bob);
            let mut contract = SharesToken::new(
                AccountId::from([0x00; 32]),
                Some(String::from("share coin")),
                Some(String::from("sCOIN")),
                8,
            );

            assert!(contract.mint(bob, 10_000_000).is_ok());
            assert!(contract.mint(alice, 5_000_000).is_ok());
            assert_eq!(contract.balance_of(bob), 10_000_000);
            assert_eq!(contract.balance_of(alice), 5_000_000);
            assert_eq!(contract.total_supply(), 15_000_000);
        }

        #[ink::test]
        fn mint_works_cannot_by_not_owner() {
            let accounts = default_accounts();
            let alice = accounts.alice;
            let bob = accounts.bob;
            set_caller(bob);
            let mut contract = SharesToken::new(
                AccountId::from([0x00; 32]),
                Some(String::from("share coin")),
                Some(String::from("sCOIN")),
                8,
            );

            set_caller(alice);
            assert_eq!(
                contract.mint(bob, 100_000).unwrap_err(),
                PSP22Error::from(OwnableError::CallerIsNotOwner)
            );
        }

        #[ink::test]
        fn mint_works_event() {
            let accounts = default_accounts();
            let alice = accounts.alice;
            let bob = accounts.bob;
            set_caller(bob);
            let mut contract = SharesToken::new(
                AccountId::from([0x00; 32]),
                Some(String::from("share coin")),
                Some(String::from("sCOIN")),
                8,
            );

            assert!(contract.mint(alice, 10_000_000).is_ok());
            // emit event
            let event = decode_transfer_event(get_emitted_events()[1].clone());
            assert_eq!(event.from, None);
            assert_eq!(event.to, Some(alice));
            assert_eq!(event.value, 10_000_000);
        }

        #[ink::test]
        fn burn_works() {
            let accounts = default_accounts();
            let alice = accounts.alice;
            let bob = accounts.bob;
            set_caller(bob);
            let mut contract = SharesToken::new(
                AccountId::from([0x00; 32]),
                Some(String::from("share coin")),
                Some(String::from("sCOIN")),
                8,
            );
            assert!(contract.mint(bob, 10_000_000).is_ok());
            assert!(contract.mint(alice, 5_000_000).is_ok());

            assert!(contract.burn(bob, 1_000_000).is_ok());
            assert!(contract.burn(alice, 3_000_000).is_ok());
            assert_eq!(contract.balance_of(bob), 9_000_000);
            assert_eq!(contract.balance_of(alice), 2_000_000);
            assert_eq!(contract.total_supply(), 11_000_000);
        }

        #[ink::test]
        fn burn_works_cannot_by_not_owner() {
            let accounts = default_accounts();
            let alice = accounts.alice;
            let bob = accounts.bob;
            set_caller(bob);
            let mut contract = SharesToken::new(
                AccountId::from([0x00; 32]),
                Some(String::from("share coin")),
                Some(String::from("sCOIN")),
                8,
            );
            assert!(contract.mint(bob, 100_000).is_ok());

            set_caller(alice);
            assert_eq!(
                contract.burn(bob, 50_000).unwrap_err(),
                PSP22Error::from(OwnableError::CallerIsNotOwner)
            );
        }

        #[ink::test]
        fn burn_works_event() {
            let accounts = default_accounts();
            let alice = accounts.alice;
            let bob = accounts.bob;
            set_caller(bob);
            let mut contract = SharesToken::new(
                AccountId::from([0x00; 32]),
                Some(String::from("share coin")),
                Some(String::from("sCOIN")),
                8,
            );

            assert!(contract.mint(alice, 10_000_000).is_ok());
            assert!(contract.burn(alice, 7_500_000).is_ok());
            // emit event
            let event = decode_transfer_event(get_emitted_events()[2].clone());
            assert_eq!(event.from, Some(alice));
            assert_eq!(event.to, None);
            assert_eq!(event.value, 7_500_000);
        }
    }
}
