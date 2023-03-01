use openbrush::traits::{
    AccountId,
    Balance,
};

use super::asset_pool;

#[openbrush::wrapper]
pub type ServiceRef = dyn Service;

#[openbrush::trait_definition]
pub trait Service {
    #[ink(message)]
    fn deposit(
        &mut self,
        asset: AccountId,
        amount: Balance,
        account: Option<AccountId>,
    ) -> Result<()>;

    #[ink(message)]
    fn withdraw(&mut self, asset: AccountId, amount: Balance, to: Option<AccountId>) -> Result<()>;

    #[ink(message)]
    fn borrow(
        &mut self,
        asset: AccountId,
        amount: Balance,
        account: Option<AccountId>,
    ) -> Result<()>;

    #[ink(message)]
    fn repay(
        &mut self,
        asset: AccountId,
        amount: Balance,
        account: Option<AccountId>,
    ) -> Result<()>;

    #[ink(message)]
    fn liquidation_call(
        &mut self,
        liquidatee: AccountId,
        collateral_asset: AccountId,
        debt_asset: AccountId,
        debt_amount: Balance,
    ) -> Result<()>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    PoolNotFound,
    AssetPool(asset_pool::Error),
    Risk(u8),
}

pub type Result<T> = core::result::Result<T, Error>;
