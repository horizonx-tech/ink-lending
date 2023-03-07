#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pool {

    use logics::{
        asset_pool::*,
        traits::asset_pool::*,
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct AssetPoolContract {
        #[storage_field]
        asset_pool: Data,
    }

    impl AssetPool for AssetPoolContract {}

    impl AssetPoolContract {
        #[ink(constructor)]
        pub fn new(
            registry: AccountId,
            asset: AccountId,
            collateral_token: AccountId,
            debt_token: AccountId,
        ) -> Self {
            Self {
                asset_pool: Data {
                    registry,
                    asset,
                    collateral_token,
                    debt_token,
                    liquidity_index: 0,
                    liquidity_rate: 0,
                    debt_index: 0,
                    debt_rate: 0,
                    last_update_timestamp: 0,
                },
            }
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

            let registry = AccountId::from([0xfa; 32]);
            let asset = AccountId::from([0xfb; 32]);
            let collateral_token = AccountId::from([0xfc; 32]);
            let debt_token = AccountId::from([0xfd; 32]);
            let contract = AssetPoolContract::new(registry, asset, collateral_token, debt_token);

            assert_eq!(contract.registry(), registry);
            assert_eq!(contract.asset(), asset);
            assert_eq!(contract.collateral_token(), collateral_token);
            assert_eq!(contract.debt_token(), debt_token);
            assert_eq!(contract.liquidity_index(), 0);
            assert_eq!(contract.liquidity_rate(), 0);
            assert_eq!(contract.debt_index(), 0);
            assert_eq!(contract.debt_rate(), 0);
            assert_eq!(contract.last_update_timestamp(), 0);
        }
    }
}
