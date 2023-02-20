#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod factory {
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
        pub fn new(registry: AccountId, pool_code_hash: Hash) -> Self {
            Self {
                factory: Data {
                    registry,
                    pool_code_hash,
                },
            }
        }
        #[ink(message)]
        pub fn registry(&self) -> AccountId {
            self.factory.registry
        }
    }

    impl Internal for FactoryContract {
        fn _instantiate(
            &self,
            asset: AccountId,
            salt: &[u8],
            _data: &Vec<u8>,
        ) -> Result<AccountId> {
            let collateral =
                SharesTokenRef::new(asset, Some("collateral".into()), Some("c".into()), 18)
                    .endowment(0)
                    .code_hash(self.factory.pool_code_hash)
                    .salt_bytes(&salt[..4])
                    .instantiate();

            Ok(collateral.to_account_id())
        }
    }
}
