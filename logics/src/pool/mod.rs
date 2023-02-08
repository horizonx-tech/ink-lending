use crate::traits::tokens::collateral_token::PSP22CollateralRef;
use openbrush::{
    contracts::psp22::{PSP22Ref, PSP22Error},
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
}

trait Internal {
    fn _to_share(&self, asset: &AccountId, amount: Balance) -> Result<Balance>;
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    // assets: Mapping<AccountId, Asset>,
    assets: Mapping<AccountId, AccountId>,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    PSP22(PSP22Error)
}
type Result<T> = core::result::Result<T, Error>;

impl<T: Storage<Data>> Pool for T {
    default fn deposit(&self, asset: AccountId, amount: Balance) -> Result<()> {
        // TODO only supported asset
        let depositor = Self::env().caller();
        let pool = Self::env().account_id();

        if let Err(err) = PSP22Ref::transfer_from(&asset, depositor, pool, amount, Vec::<u8>::new()) {
            return Err(Error::PSP22(err));
        }

        let collateral_token = self.data().assets.get(&asset).unwrap();
        if let Err(err) = PSP22CollateralRef::mint(&collateral_token, depositor, amount) {
            return Err(Error::PSP22(err));
        }

        Ok(())
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _to_share(&self, asset: &AccountId, amount: Balance) -> Result<Balance> {
        // TODO only supported
        Ok(PSP22CollateralRef::to_share(asset, amount))
    }
}
