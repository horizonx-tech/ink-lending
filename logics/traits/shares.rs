use openbrush::{
    contracts::traits::{
        ownable::*,
        psp22::{
            extensions::{
                burnable::*,
                metadata::*,
                mintable::*,
            },
            *,
        },
    },
    traits::{
        AccountId,
        Balance,
    },
};

#[openbrush::wrapper]
pub type SharesRef =
    dyn PSP22 + PSP22Mintable + PSP22Burnable + PSP22Shareable + PSP22Metadata + Ownable;

#[openbrush::trait_definition]
pub trait Shares:
    PSP22 + PSP22Mintable + PSP22Burnable + PSP22Shareable + PSP22Metadata + Ownable
{
}

#[openbrush::trait_definition]
pub trait PSP22Shareable {
    #[ink(message)]
    fn total_share(&self) -> Balance;

    #[ink(message)]
    fn share_of(&self, owner: AccountId) -> Balance;
}
