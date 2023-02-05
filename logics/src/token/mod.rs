// TODO seperate contract from logic
#[openbrush::contract]
mod collateral_token {
    use openbrush::contracts::psp22::{extensions::mintable::*, PSP22Error};
    use openbrush::traits::{Storage};

    pub trait Internal {
        // TODO move to PSP22
        fn _total_supply(&self) -> Balance;

        fn _total_share(&self) -> Balance;

        fn _share_of(&self, owner: &AccountId) -> Balance;

        fn _accrued_interest(&self) -> Balance;
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct CollateralToken {
        #[storage_field]
        psp22: psp22::Data,
    }

    impl PSP22 for CollateralToken {}
    impl PSP22Mintable for CollateralToken {}

    impl psp22::Internal for CollateralToken {
        fn _balance_of(&self, owner: &AccountId) -> Balance {
            self._share_of(owner) * self._total_supply() / self._total_share()
        }

        fn _mint_to(&mut self, to: AccountId, amount: Balance) -> Result<(), PSP22Error> {
            // TODO only lending pool
            if to == AccountId::default() {
                return Err(PSP22Error::ZeroRecipientAddress);
            }

            let mut balance = self._share_of(&to);
            balance += amount;
            self.psp22.balances.insert(&to, &balance);
            self.psp22.supply += amount;
            self._emit_transfer_event(None, Some(to), amount);

            Ok(())
        }
    }

    impl Internal for CollateralToken {
        // TODO override PSP22 default implementation
        fn _total_supply(&self) -> Balance {
            let total_share = self._total_share();
            total_share + self._accrued_interest()
        }

        fn _total_share(&self) -> Balance {
            self.psp22.supply.clone()
        }

        fn _share_of(&self, owner: &AccountId) -> Balance {
            self.psp22.balances.get(owner).unwrap_or(0)
        }

        fn _accrued_interest(&self) -> Balance {
            // TODO implement
            0
        }
    }

    impl CollateralToken {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }
}
