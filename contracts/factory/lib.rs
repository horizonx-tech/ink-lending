#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod factory {
    use asset_pool::pool::AssetPoolContractRef;
    use ink::{
        prelude::vec::Vec,
        ToAccountId,
    };
    use logics::{
        factory::*,
        traits::factory::*,
    };
    use openbrush::traits::Storage;
    use shares_token::token::SharesTokenRef;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct FactoryContract {
        #[storage_field]
        factory: Data,
    }

    impl Factory for FactoryContract {}

    impl FactoryContract {
        #[ink(constructor)]
        pub fn new(registry: AccountId, pool_code_hash: Hash, shares_code_hash: Hash) -> Self {
            Self {
                factory: Data {
                    registry,
                    pool_code_hash,
                    shares_code_hash,
                },
            }
        }
        #[ink(message)]
        pub fn registry(&self) -> AccountId {
            self.factory.registry
        }
        #[ink(message)]
        pub fn pool_code_hash(&self) -> Hash {
            self.factory.pool_code_hash
        }
        #[ink(message)]
        pub fn shares_code_hash(&self) -> Hash {
            self.factory.shares_code_hash
        }
    }

    impl Internal for FactoryContract {
        fn _instantiate(
            &self,
            asset: AccountId,
            salt: &[u8],
            _data: &Vec<u8>,
        ) -> Result<AccountId> {
            // TODO name, symbol
            let collateral =
                SharesTokenRef::new(asset, Some("collateral".into()), Some("c".into()), 18)
                    .endowment(0)
                    .code_hash(self.factory.shares_code_hash)
                    .salt_bytes(&salt[..4])
                    .instantiate();
            let debt = SharesTokenRef::new(asset, Some("debt".into()), Some("vd".into()), 18)
                .endowment(0)
                .code_hash(self.factory.shares_code_hash)
                .salt_bytes(&salt[5..9])
                .instantiate();

            let pool = AssetPoolContractRef::new(
                self.factory.registry,
                asset,
                collateral.to_account_id(),
                debt.to_account_id(),
            )
            .endowment(0)
            .code_hash(self.factory.pool_code_hash)
            .salt_bytes(&salt[..4])
            .instantiate();

            Ok(pool.to_account_id())
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
            let pool_code_hash = [1u8; 32].into();
            let shares_code_hash = [2u8; 32].into();
            let contract = FactoryContract::new(registry, pool_code_hash, shares_code_hash);

            assert_eq!(contract.registry(), registry);
            assert_eq!(contract.pool_code_hash(), pool_code_hash);
            assert_eq!(contract.shares_code_hash(), shares_code_hash);
        }

        #[ink::test]
        fn create_works_cannot_by_not_owner() {
            let accounts = default_accounts();
            set_caller(accounts.bob);

            let contract = FactoryContract::new(
                AccountId::from([0x00; 32]),
                [0u8; 32].into(),
                [0u8; 32].into(),
            );

            set_caller(accounts.charlie);
            assert_eq!(
                contract
                    .create(AccountId::from([0x00; 32]), vec![])
                    .unwrap_err(),
                Error::CallerIsNotManager
            );
        }
    }
}
