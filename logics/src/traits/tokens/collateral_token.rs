pub use crate::traits::tokens::psp22::PSP22;
use openbrush::contracts::traits::psp22::PSP22Error;
use openbrush::traits::{AccountId, Balance};

#[openbrush::wrapper]
pub type PSP22CollateralRef = dyn PSP22Collateral;

#[openbrush::trait_definition]
pub trait PSP22Collateral: PSP22 {
    #[ink(message)]
    fn mint(&mut self, to: AccountId, share: Balance, supply: Balance) -> Result<(), PSP22Error>;
}
