#![feature(min_specialization)]

#[openbrush::contract]
pub mod collateral_token {
    use logics::tokens::collateral_token::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct CollateralToken {
        #[storage_field]
        data: Data,
    }

    impl PSP22Collateral for CollateralToken {}

    impl CollateralToken {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        // TODO
        #[ink(message)]
        pub fn accrue_interest(&mut self, interest: Balance) -> Result<(), ()> {
            // TODO only lending pool
            self.data.total_supply += interest;

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::{test::default_accounts, DefaultEnvironment};

        #[ink::test]
        fn test_default() {
            let accounts = default_accounts::<DefaultEnvironment>();

            let token = CollateralToken::new();
            let owner = accounts.alice;

            assert_eq!(token.total_supply(), 0);
            assert_eq!(token.balance_of(owner), 0);
        }

        #[ink::test]
        fn test_share_initial_mint() {
            let accounts = default_accounts::<DefaultEnvironment>();

            let mut token = CollateralToken::new();
            let owner = accounts.alice;
            let supply = 100;
            let expected = supply;

            assert!(token.mint(owner, supply).is_ok());
            assert_eq!(token.total_supply(), expected);
            assert_eq!(token.balance_of(owner), expected);
        }

        #[ink::test]
        fn test_share_interest_accrued() {
            let accounts = default_accounts::<DefaultEnvironment>();

            let mut token = CollateralToken::new();
            let owner = accounts.alice;
            let supply = 100;
            let accrued_interest = 20;

            let expected_share = supply;
            let expected_balance = supply + accrued_interest;

            assert!(token.mint(owner, supply).is_ok());
            assert!(token.accrue_interest(accrued_interest).is_ok());

            assert_eq!(token.total_supply(), expected_balance);
            assert_eq!(token._total_share(), expected_share);
            assert_eq!(token._share_of(&owner), expected_share);
            assert_eq!(token.balance_of(owner), expected_balance);
        }

        #[ink::test]
        fn test_share_interest_accrued_and_deposited() {
            let accounts = default_accounts::<DefaultEnvironment>();

            let mut token = CollateralToken::new();
            let supply_alice = 100;
            let accrued_interest = 20;
            let supply_bob = 12;

            let expected_share_alice = supply_alice;
            let expected_alice_balance = supply_alice + accrued_interest;
            let expected_share_bob = 10; // 12 * 100 (total_share) / 120 (total_supply)
            let expected_bob_balance = supply_bob;

            assert!(token.mint(accounts.alice, supply_alice).is_ok());
            assert!(token.accrue_interest(accrued_interest).is_ok());
            assert!(token.mint(accounts.bob, supply_bob).is_ok());

            assert_eq!(token._share_of(&accounts.alice), expected_share_alice);
            assert_eq!(token.balance_of(accounts.alice), expected_alice_balance);
            assert_eq!(token._share_of(&accounts.bob), expected_share_bob);
            assert_eq!(token.balance_of(accounts.bob), expected_bob_balance);
        }

        #[ink::test]
        fn test_share_burned() {
            let accounts = default_accounts::<DefaultEnvironment>();

            let mut token = CollateralToken::new();
            let supply_alice = 100;
            let accrued_interest = 20;
            let supply_bob = 12;
            let burn_alice = 66;

            let expected_share_alice = 45; // 100 - 110 * 66 / 132
            let expected_balance_alice = supply_alice + accrued_interest - burn_alice;
            let expected_share_bob = 10; // 12 * 100 (total_share) / 120 (total_supply)
            let expected_balance_bob = supply_bob;

            assert!(token.mint(accounts.alice, supply_alice).is_ok());
            assert!(token.accrue_interest(accrued_interest).is_ok());
            assert!(token.mint(accounts.bob, supply_bob).is_ok());
            assert!(token.burn(accounts.alice, burn_alice).is_ok());

            assert_eq!(token._share_of(&accounts.alice), expected_share_alice);
            assert_eq!(token.balance_of(accounts.alice), expected_balance_alice);
            assert_eq!(token._share_of(&accounts.bob), expected_share_bob);
            assert_eq!(token.balance_of(accounts.bob), expected_balance_bob);
        }
    }
}
