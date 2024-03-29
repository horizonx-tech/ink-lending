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
    use openbrush::{
        contracts::{
            ownable::OwnableRef,
            psp22::extensions::metadata::PSP22MetadataRef
        },
        traits::Storage,
    };
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
            let base_name = PSP22MetadataRef::token_name(&asset).unwrap();
            let base_symbol = PSP22MetadataRef::token_symbol(&asset).unwrap();
            let mut collateral_name = "collateral ".as_bytes().to_vec();
            collateral_name.append(&mut base_name.clone());
            let mut collateral_symbol = "c".as_bytes().to_vec();
            collateral_symbol.append(&mut base_symbol.clone());
            let collateral =
                SharesTokenRef::new(asset, Some(collateral_name), Some(collateral_symbol), 18)
                    .endowment(0)
                    .code_hash(self.factory.shares_code_hash)
                    .salt_bytes(&salt[..4])
                    .instantiate();
            let mut debt_name = "debt ".as_bytes().to_vec();
            debt_name.append(&mut base_name.clone());
            let mut debt_symbol = "vd".as_bytes().to_vec();
            debt_symbol.append(&mut base_symbol.clone());    
            let debt = SharesTokenRef::new(asset, Some(debt_name), Some(debt_symbol), 18)
                .endowment(0)
                .code_hash(self.factory.shares_code_hash)
                .salt_bytes(&salt[5..9])
                .instantiate();

            let pool = AssetPoolContractRef::new(
                self.factory.registry,
                asset,
                collateral.to_account_id(),
                debt.to_account_id(),
                None,
                None
            )
            .endowment(0)
            .code_hash(self.factory.pool_code_hash)
            .salt_bytes(&salt[..4])
            .instantiate();

            OwnableRef::transfer_ownership(&collateral.to_account_id(), pool.to_account_id()).unwrap();
            OwnableRef::transfer_ownership(&debt.to_account_id(), pool.to_account_id()).unwrap();

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
    }
}
