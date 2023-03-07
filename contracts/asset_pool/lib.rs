#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod pool {
    use ink::env::call::FromAccountId;
    use logics::{
        asset_pool::*,
        traits::asset_pool::*,
    };
    use openbrush::traits::Storage;
    use scale::{
        Decode,
        Encode,
    };
    use shares_token::token::SharesTokenRef;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct AssetPoolContract {
        #[storage_field]
        asset_pool: Data,
    }

    #[derive(Decode, Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct PoolData {
        pub registry: AccountId,
        pub asset: AccountId,
        pub collateral_token: AccountId,
        pub debt_token: AccountId,
        pub liquidity_share: u128,
        pub liquidity_index: u128,
        pub liquidity_rate: u128,
        pub debt_share: u128,
        pub debt_index: u128,
        pub debt_rate: u128,
        pub last_update_timestamp: Timestamp,
    }

    impl From<Data> for PoolData {
        fn from(item: Data) -> PoolData {
            PoolData {
                registry: item.registry,
                asset: item.asset,
                collateral_token: item.collateral_token,
                debt_token: item.debt_token,
                liquidity_share: 0,
                liquidity_index: item.liquidity_index,
                liquidity_rate: item.liquidity_rate,
                debt_share: 0,
                debt_index: item.debt_index,
                debt_rate: item.debt_rate,
                last_update_timestamp: item.last_update_timestamp,
            }
        }
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
            let mut instance = Self::default();
            instance.asset_pool.registry = registry;
            instance.asset_pool.asset = asset;
            instance.asset_pool.collateral_token = collateral_token;
            instance.asset_pool.debt_token = debt_token;

            instance
        }

        #[ink(message)]
        pub fn pool_data(&self) -> PoolData {
            let mut data: PoolData = self._pool_data().into();
            let collateral: SharesTokenRef = FromAccountId::from_account_id(data.collateral_token);
            data.liquidity_share = collateral.total_share();
            let debt: SharesTokenRef = FromAccountId::from_account_id(data.debt_token);
            data.debt_share = debt.total_share();

            data
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
