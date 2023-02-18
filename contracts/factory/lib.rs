#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod factory {

    use ink::ToAccountId;
    use logics::factory::*;
    use openbrush::traits::Storage;
    use shares_token::shares_token::SharesTokenRef;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct FactoryContract {
        #[storage_field]
        data: Data,
    }

    impl Factory for FactoryContract {}

    impl FactoryContract {
        #[ink(constructor)]
        pub fn new(registry: AccountId, pool_code_hash: Hash) -> Self {
            Self {
                data: Data {
                    registry,
                    pool_code_hash,
                },
            }
        }
    }

    impl Internal for FactoryContract {
        fn _instantiate(&self, asset: AccountId, salt: &[u8], data: &Vec<u8>) -> Result<AccountId> {
            let collateral =
                SharesTokenRef::new(asset, Some("collateral".into()), Some("c".into()), 18)
                    .endowment(0)
                    .code_hash(self.data.pool_code_hash)
                    .salt_bytes(&salt[..4])
                    .instantiate()
                    .expect("failed to instantiate collateral");

            Ok(collateral.to_account_id())
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink::env::{test::default_accounts, DefaultEnvironment};

        #[ink_e2e::test]
        async fn test_default(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = FactoryContract::new(AccountId::default(), Hash::default());

            let acc_id = client.instantiate("factory", &ink_e2e::alice(), constructor, 0, None);
            // let res = factory.create(AccountId::default(), Vec::<u8>::new());
        }
        // fn test_default() {
        //     let accounts = default_accounts::<DefaultEnvironment>();

        //     let mut factory = FactoryContract::new(AccountId::default(), Hash::default());
        //     let res = factory.create(AccountId::default(), Vec::<u8>::new());

        //     assert!(res.is_ok());
        //     assert_eq!(res.unwrap(), AccountId::default());
        // }
    }
}
