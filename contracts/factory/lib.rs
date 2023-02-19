#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod factory {
    use ink::{prelude::vec::Vec, ToAccountId};
    use logics::factory::*;
    use logics::traits::factory::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct FactoryContract {
        #[storage_field]
        factory: Data,
    }

    impl Factory for FactoryContract {}

    impl FactoryContract {
        #[ink(constructor)]
        pub fn new(registry: AccountId, pool_code_hash: Hash) -> Self {
            Self {
                factory: Data {
                    registry,
                    pool_code_hash,
                },
            }
        }
    }

    impl Internal for FactoryContract {
        // fn _instantiate(&self, asset: AccountId, salt: &[u8], data: &Vec<u8>) -> Result<AccountId> {
        //     let collateral =
        //         SharesTokenRef::new(asset, Some("collateral".into()), Some("c".into()), 18)
        //             .endowment(0)
        //             .code_hash(self.factory.pool_code_hash)
        //             .salt_bytes(&salt[..4])
        //             .instantiate();

        //     Ok(collateral.to_account_id())
        // }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn test_default(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = FactoryContractRef::new(
                ink_e2e::account_id(ink_e2e::AccountKeyring::Bob),
                Hash::default(),
            );

            let acc_id = client
                .instantiate("factory", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            Ok(())
        }
    }
}
