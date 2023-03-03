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
}

#[cfg(test)]
mod tests {
    use ink::env;
    use openbrush::traits::{AccountId, String};
    use openbrush::{
        contracts::{
            ownable::*,
            psp22::extensions::{
                metadata::*,
            },
        },
    };
    use super::token;

    fn default_accounts() -> env::test::DefaultAccounts<env::DefaultEnvironment> {
        env::test::default_accounts::<env::DefaultEnvironment>()
    }

    #[ink::test]
    fn new_works() {
        let accounts = default_accounts();
        env::test::set_caller::<env::DefaultEnvironment>(accounts.bob);

        let contract = token::SharesToken::new(
            AccountId::from([0x00; 32]),
            Some(String::from("share coin")),
            Some(String::from("sCOIN")),
            8,
        );
        assert_eq!(contract.token_name().unwrap(), String::from("share coin"));
        assert_eq!(contract.token_symbol().unwrap(), String::from("sCOIN"));
        assert_eq!(contract.total_supply(), 0);
        assert_eq!(contract.owner(), accounts.bob);
    }
}
