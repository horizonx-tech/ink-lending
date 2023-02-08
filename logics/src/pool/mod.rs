use crate::traits::{pool::*, tokens::collateral_token::PSP22CollateralRef};
use openbrush::{
    contracts::psp22::PSP22Ref,
    storage::Mapping,
    traits::{AccountId, Balance, Storage},
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    assets: Mapping<AccountId, AccountId>,
}

impl<T: Storage<Data>> Pool for T {
    default fn deposit(&self, asset: AccountId, amount: Balance) -> Result<()> {
        // TODO only supported asset
        let depositor = Self::env().caller();
        let pool = Self::env().account_id();

        let collateral_token = self.data().assets.get(&asset).unwrap();
        if let Err(err) = PSP22Ref::transfer_from(
            &asset,
            depositor,
            collateral_token,
            amount,
            Vec::<u8>::new(),
        ) {
            return Err(Error::PSP22(err));
        }

        if let Err(err) = PSP22CollateralRef::mint(&collateral_token, depositor, amount) {
            return Err(Error::PSP22(err));
        }

        Ok(())
    }

    default fn withdraw(&self, asset: AccountId, amount: Balance) -> Result<()> {
        // TODO only supported asset
        // TODO reject if withdrawer balance insufficient
        // TODO reject if withdrawer collateral insufficient
        // TODO reject if asset balance of collateral token insufficient

        let withdrawer = Self::env().caller();
        let pool = Self::env().account_id();

        let collateral_token = self.data().assets.get(&asset).unwrap();
        if let Err(err) = PSP22CollateralRef::burn(&asset, withdrawer, amount) {
            return Err(Error::PSP22(err));
        }

        if let Err(err) = PSP22Ref::transfer_from(
            &asset,
            collateral_token,
            withdrawer,
            amount,
            Vec::<u8>::new(),
        ) {
            return Err(Error::PSP22(err));
        }

        Ok(())
    }
}

trait Internal {
    fn _to_share(&self, asset: &AccountId, amount: Balance) -> Result<Balance>;
}

impl<T: Storage<Data>> Internal for T {
    default fn _to_share(&self, asset: &AccountId, amount: Balance) -> Result<Balance> {
        // TODO only supported
        Ok(PSP22CollateralRef::to_share(asset, amount))
    }
}
