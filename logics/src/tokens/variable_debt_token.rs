pub use crate::traits::tokens::variable_debt_token::*;
use openbrush::{
    contracts::psp22::PSP22Error,
    storage::Mapping,
    traits::{AccountId, Balance, Storage},
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub total_supply: Balance,
    pub total_share: Balance,
    pub shares: Mapping<AccountId, Balance>,
}

impl<T: Storage<Data>> PSP22VariableDebt for T {
    default fn total_share(&self) -> Balance {
        self._total_share()
    }

    default fn to_share(&self, amount: Balance) -> Balance {
        self._to_share(amount)
    }

    default fn mint(&mut self, to: AccountId, supply: Balance) -> Result<(), PSP22Error> {
        self._mint(to, supply)
    }

    default fn burn(&mut self, from: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        self._burn(from, amount)
    }

    default fn total_supply(&self) -> Balance {
        self._total_supply()
    }

    default fn balance_of(&self, owner: AccountId) -> Balance {
        self._balance_of(&owner)
    }

    default fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
        todo!()
    }

    default fn transfer(
        &mut self,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        todo!()
    }

    default fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error> {
        todo!()
    }

    default fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error> {
        todo!()
    }

    default fn increase_allowance(
        &mut self,
        spender: AccountId,
        delta_value: Balance,
    ) -> Result<(), PSP22Error> {
        todo!()
    }

    default fn decrease_allowance(
        &mut self,
        spender: AccountId,
        delta_value: Balance,
    ) -> Result<(), PSP22Error> {
        todo!()
    }
}

pub trait Internal {
    fn _total_supply(&self) -> Balance;

    fn _balance_of(&self, owner: &AccountId) -> Balance;

    fn _total_share(&self) -> Balance;

    fn _share_of(&self, owner: &AccountId) -> Balance;

    fn _to_share(&self, amount: Balance) -> Balance;

    fn _accrued_interest(&self) -> Balance;

    fn _mint(&mut self, to: AccountId, supply: Balance) -> Result<(), PSP22Error>;

    fn _burn(&mut self, from: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    /// User must override those methods in their contract.
    fn _emit_transfer_event(
        &self,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _amount: Balance,
    );
    // fn _emit_approval_event(&self, _owner: AccountId, _spender: AccountId, _amount: Balance);
}

impl<T: Storage<Data>> Internal for T {
    default fn _total_supply(&self) -> Balance {
        self.data().total_supply.clone()
    }

    default fn _balance_of(&self, owner: &AccountId) -> Balance {
        let total_share = self._total_share();
        if total_share == 0 {
            return 0;
        }
        self._share_of(owner) * self.total_supply() / total_share
    }

    default fn _to_share(&self, amount: Balance) -> Balance {
        let total_supply = self._total_supply();
        if total_supply == 0 {
            amount
        } else {
            amount * self._total_share() / total_supply
        }
    }

    default fn _mint(&mut self, to: AccountId, supply: Balance) -> Result<(), PSP22Error> {
        // TODO only lending pool
        if to == AccountId::default() {
            return Err(PSP22Error::ZeroRecipientAddress);
        }

        let share = self._to_share(supply);
        let mut new_share = self._share_of(&to);
        new_share += share;
        self.data().shares.insert(&to, &new_share);
        self.data().total_supply += supply;
        self.data().total_share += share;
        self._emit_transfer_event(None, Some(to), share);

        Ok(())
    }

    default fn _burn(&mut self, from: AccountId, amount: Balance) -> Result<(), PSP22Error> {
        // TODO only lending pool
        if from == AccountId::default() {
            return Err(PSP22Error::ZeroRecipientAddress);
        }

        let share = self._to_share(amount);
        let mut new_share = self._share_of(&from);
        new_share -= share;
        self.data().shares.insert(&from, &new_share);
        self.data().total_supply -= amount;
        self.data().total_share -= share;

        Ok(())
    }

    default fn _total_share(&self) -> Balance {
        self.data().total_share.clone()
    }

    default fn _share_of(&self, owner: &AccountId) -> Balance {
        self.data().shares.get(owner).unwrap_or(0)
    }

    default fn _accrued_interest(&self) -> Balance {
        // TODO implement
        0
    }

    default fn _emit_transfer_event(
        &self,
        _from: Option<AccountId>,
        _to: Option<AccountId>,
        _amount: Balance,
    ) {
    }
}
