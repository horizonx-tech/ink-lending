#![feature(min_specialization)]

#[openbrush::contract]
pub mod collateral_token {
    use logics::tokens::collateral_token::{self, Internal, *};
    use openbrush::contracts::psp22::*;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct CollateralToken {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        data: collateral_token::Data,
    }

    impl PSP22 for CollateralToken {}
    impl PSP22Collateral for CollateralToken {}

    impl psp22::Internal for CollateralToken {
        fn _balance_of(&self, owner: &AccountId) -> Balance {
            let total_share = self._total_share();
            if total_share == 0 {
                return 0;
            }
            self._share_of(owner) * self.total_supply() / total_share
        }
    }

    impl CollateralToken {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        // TODO
        #[ink(message)]
        pub fn accrue_interest(&mut self, interest: Balance) -> Result<(), ()> {
            // TODO only lending pool
            self.psp22.supply += interest;

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
            let share = 100;
            let expected = share;

            assert!(token.mint(owner, share, supply).is_ok());
            assert_eq!(token.total_supply(), expected);
            assert_eq!(token.balance_of(owner), expected);
        }

        #[ink::test]
        fn test_share_interest_accrued() {
            let accounts = default_accounts::<DefaultEnvironment>();

            let mut token = CollateralToken::new();
            let owner = accounts.alice;
            let supply = 100;
            let share = 100;
            let accrued_interest = 20;
            let expected_balance = share + accrued_interest;

            assert!(token.mint(owner, share, supply).is_ok());
            assert!(token.accrue_interest(accrued_interest).is_ok());

            assert_eq!(token.total_supply(), expected_balance);
            assert_eq!(token._total_share(), share);
            assert_eq!(token._share_of(&owner), share);
            assert_eq!(token.balance_of(owner), expected_balance);
        }

        #[ink::test]
        fn test_share_interest_accrued_and_deposited() {
            let accounts = default_accounts::<DefaultEnvironment>();

            let mut token = CollateralToken::new();
            let supply_alice = 100;
            let share_alice = 100;
            let accrued_interest = 20;
            let expected_alice_balance = share_alice + accrued_interest;
            let supply_bob = 12;
            let share_bob = 10; // 12 * 100 (total_share) / 120 (total_supply)
            let expected_bob_balance = supply_bob;

            assert!(token
                .mint(accounts.alice, share_alice, supply_alice)
                .is_ok());
            assert!(token.accrue_interest(accrued_interest).is_ok());
            assert!(token.mint(accounts.bob, share_bob, supply_bob).is_ok());

            assert_eq!(token._share_of(&accounts.alice), share_alice);
            assert_eq!(token.balance_of(accounts.alice), expected_alice_balance);
            assert_eq!(token._share_of(&accounts.bob), share_bob);
            assert_eq!(token.balance_of(accounts.bob), expected_bob_balance);
        }
    }
}
