use openbrush::traits::{AccountId, Balance};

#[openbrush::wrapper]
pub type ValidatorRef = dyn Validator;

#[openbrush::trait_definition]
pub trait Validator {
    #[ink(message)]
    fn validate_borrow(&self, account: AccountId, asset: AccountId, amount: Balance) -> Result;

    #[ink(message)]
    fn validate_withdraw(&self, account: AccountId, asset: AccountId, amount: Balance) -> Result;
}

pub type Result = core::result::Result<(), u8>;
