#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pool {
    use logics::{
        asset_pool::*,
        traits::asset_pool::*,
        ui_data_providers::pool_data_provider::*,
    };
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct AssetPoolContract {
        #[storage_field]
        asset_pool: Data,
    }

    impl AssetPool for AssetPoolContract {}

    impl UIPoolDataProvider for AssetPoolContract {}

    impl AssetPoolContract {
        #[ink(constructor)]
        pub fn new(
            registry: AccountId,
            asset: AccountId,
            collateral_token: AccountId,
            debt_token: AccountId,
        ) -> Self {
            let mut instance = Self::default();
            instance.asset_pool.registry = registry;
            instance.asset_pool.asset = asset;
            instance.asset_pool.collateral_token = collateral_token;
            instance.asset_pool.debt_token = debt_token;

            instance
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
            assert_eq!(contract.liquidity_rate(), Vec::new());
            assert_eq!(contract.debt_index(), 0);
            assert_eq!(contract.debt_rate(), Vec::new());
            assert_eq!(contract.last_update_timestamp(), 0);
        }
    }
}
