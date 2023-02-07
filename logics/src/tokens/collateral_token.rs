use openbrush::contracts::psp22::*;
use openbrush::traits::{AccountId, Balance, Storage};

#[openbrush::wrapper]
pub type PSP22CollateralRef = dyn PSP22Collateral;

#[openbrush::trait_definition]
pub trait PSP22Collateral {
    #[ink(message)]
    fn mint(&mut self, to: AccountId, share: Balance, supply: Balance) -> Result<(), PSP22Error>;
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[openbrush::upgradeable_storage(STORAGE_KEY)]
#[derive(Default, Debug)]
pub struct Data {
    total_share: Balance,
}

impl<T: Storage<Data> + Storage<psp22::Data>> PSP22Collateral for T {
    default fn mint(
        &mut self,
        to: AccountId,
        share: Balance,
        supply: Balance,
    ) -> Result<(), PSP22Error> {
        self._mint(to, share, supply)
    }
}

pub trait Internal {
    fn _mint(&mut self, to: AccountId, share: Balance, supply: Balance) -> Result<(), PSP22Error>;

    fn _total_share(&self) -> Balance;

    fn _share_of(&self, owner: &AccountId) -> Balance;

    fn _accrued_interest(&self) -> Balance;
}

impl<T: Storage<Data> + Storage<psp22::Data>> Internal for T {
    default fn _mint(
        &mut self,
        to: AccountId,
        share: Balance,
        supply: Balance,
    ) -> Result<(), PSP22Error> {
        // TODO only lending pool
        if to == AccountId::default() {
            return Err(PSP22Error::ZeroRecipientAddress);
        }

        let mut balance = self._share_of(&to);
        balance += share;
        self.data::<psp22::Data>().balances.insert(&to, &balance);
        self.data::<psp22::Data>().supply += supply;
        self.data::<Data>().total_share += share;
        self._emit_transfer_event(None, Some(to), share);

        Ok(())
    }

    default fn _total_share(&self) -> Balance {
        self.data::<Data>().total_share.clone()
    }

    default fn _share_of(&self, owner: &AccountId) -> Balance {
        self.data::<psp22::Data>().balances.get(owner).unwrap_or(0)
    }

    default fn _accrued_interest(&self) -> Balance {
        // TODO implement
        0
    }
}
