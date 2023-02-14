use openbrush::contracts::traits::psp22::PSP22Error;
use openbrush::traits::{AccountId, Balance};

#[openbrush::wrapper]
pub type PSP22CollateralRef = dyn PSP22Collateral;

#[openbrush::trait_definition]
pub trait PSP22Collateral {
    #[ink(message)]
    fn total_share(&self) -> Balance;

    #[ink(message)]
    fn to_share(&self, amount: Balance) -> Balance;

    #[ink(message)]
    fn mint(&mut self, to: AccountId, supply: Balance) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn burn(&mut self, to: AccountId, amount: Balance) -> Result<(), PSP22Error>;

    // PSP22
    #[ink(message)]
    fn total_supply(&self) -> Balance;

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> Balance;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        value: Balance,
        data: Vec<u8>,
    ) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn increase_allowance(
        &mut self,
        spender: AccountId,
        delta_value: Balance,
    ) -> Result<(), PSP22Error>;

    #[ink(message)]
    fn decrease_allowance(
        &mut self,
        spender: AccountId,
        delta_value: Balance,
    ) -> Result<(), PSP22Error>;
}
