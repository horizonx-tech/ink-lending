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

    impl Shares for SharesToken {}
    impl Ownable for SharesToken {}
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
    impl SharesToken {
        #[ink(constructor)]
        pub fn new(
            asset: AccountId,
            name: Option<String>,
            symbol: Option<String>,
            decimal: u8,
        ) -> Self {
            let mut _instance = Self::default();
            _instance.asset = asset;
            _instance._init_with_owner(_instance.env().caller());
            _instance.metadata.name = name;
            _instance.metadata.symbol = symbol;
            _instance.metadata.decimals = decimal;
            _instance
        }
    }
}
