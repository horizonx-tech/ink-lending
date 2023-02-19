#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod shares_token {
    pub use logics::traits::shares_token::*;
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::extensions::{burnable::*, metadata::*, mintable::*},
        },
        modifiers,
        traits::{Storage, String},
    };

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

    impl Ownable for SharesToken {}
    impl PSP22 for SharesToken {}
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
    impl Shares for SharesToken {}

    impl SharesToken {
        #[ink(constructor)]
        pub fn new(
            asset: AccountId,
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8,
        ) -> Self {
            let owner = Self::env().caller();
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
            instance._init_with_owner(owner);

            instance
        }
    }

    // FIXME
    // #[cfg(all(test, feature = "e2e-tests"))]
    // mod e2e_tests {
    //     use super::*;

    //     type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    //     #[ink_e2e::test]
    //     async fn test_default(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    //         let constructor = SharesTokenRef::new(
    //             ink_e2e::account_id(ink_e2e::AccountKeyring::Bob),
    //             None,
    //             None,
    //             18,
    //         );

    //         let acc_id = client
    //             .instantiate("shares", &ink_e2e::alice(), constructor, 0, None)
    //             .await
    //             .expect("instantiate failed")
    //             .account_id;

    //         Ok(())
    //     }
    // }
}
