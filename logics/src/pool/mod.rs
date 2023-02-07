use crate::tokens::collateral_token::PSP22CollateralRef;
use openbrush::{
    contracts::traits::psp22::PSP22Ref,
    storage::Mapping,
    traits::{AccountId, Balance, Storage},
};

#[openbrush::wrapper]
pub type PoolRef = dyn Pool;

#[openbrush::trait_definition]
pub trait Pool {
    #[ink(message)]
    fn deposit(&self, asset: AccountId, amount: Balance) -> Result<()>;
    // fn withdraw(&self, asset: AccountId, amount: Balance) -> Result<()>;
    // fn borrow(&self, asset: AccountId, amount: Balance) -> Result<()>;
    // fn repay(&self, asset: AccountId, amount: Balance) -> Result<()>;

    #[ink(message)]
    fn total_deposited(&self, asset: AccountId) -> Result<Balance>;

    #[ink(message)]
    fn total_share(&self, asset: AccountId) -> Result<Balance>;
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    // assets: Mapping<AccountId, Asset>,
    assets: Mapping<AccountId, AccountId>,
    shares: Mapping<AccountId, Balance>,
    protocol_balances: Mapping<AccountId, Balance>,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    TransferAssetFailed,
    MintFailed,
}
type Result<T> = core::result::Result<T, Error>;

impl<T: Storage<Data>> Pool for T {
    default fn deposit(&self, asset: AccountId, amount: Balance) -> Result<()> {
        // TODO only supported asset
        let depositor = Self::env().caller();
        let pool = Self::env().account_id();

        let current_deposited = self.total_deposited(asset)?;
        let current_share = self.total_share(asset)?;
        if PSP22Ref::transfer_from(&asset, depositor, pool, amount, Vec::<u8>::new()).is_err() {
            return Err(Error::TransferAssetFailed);
        }

        let share = if current_deposited == 0 {
            amount * current_share / current_deposited
        } else {
            amount
        };

        // mint
        let collateral_token = self.data().assets.get(&asset).unwrap();
        if PSP22CollateralRef::mint(&collateral_token, depositor, share, amount).is_err() {
            return Err(Error::MintFailed);
        }

        Ok(())
    }

    default fn total_deposited(&self, asset: AccountId) -> Result<Balance> {
        // TODO only supported
        // TODO delegate to collateral token
        let deposited = PSP22Ref::balance_of(&asset, Self::env().account_id());
        // TODO
        let borrowed = 0;
        let protocol_balance = self.data().protocol_balances.get(&asset).unwrap();
        Ok(deposited + borrowed - protocol_balance)
    }

    default fn total_share(&self, asset: AccountId) -> Result<Balance> {
        // TODO only supported
        // TODO delegate to collateral token
        Ok(self.data().shares.get(&asset).unwrap())
    }
}
