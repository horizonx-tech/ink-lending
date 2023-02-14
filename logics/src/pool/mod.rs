use std::default;

use crate::traits::{
    pool::*, tokens::collateral_token::PSP22CollateralRef,
    tokens::variable_debt_token::PSP22VariableDebtRef,
};
use openbrush::{
    contracts::psp22::PSP22Ref,
    storage::Mapping,
    traits::{AccountId, Balance, Storage},
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    // TODO
    ctokens: Mapping<AccountId, AccountId>,
    vdtokens: Mapping<AccountId, AccountId>,
}

impl<T: Storage<Data>> Pool for T {
    default fn deposit(&self, asset: AccountId, amount: Balance) -> Result<()> {
        // TODO only supported asset
        let depositor = Self::env().caller();

        // TODO accrue_interest

        let collateral_token = self.data().ctokens.get(&asset).unwrap();
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

        // TODO accrue_interest

        let collateral_token = self.data().ctokens.get(&asset).unwrap();
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

    default fn borrow(&self, asset: AccountId, amount: Balance) -> Result<()> {
        // TODO only supported asset
        // TODO reject if borrower collateral insufficient
        // TODO reject if asset balance of collateral token insufficient

        let borrower = Self::env().caller();

        // TODO accrue_interest

        let variable_debt_token = self.data().vdtokens.get(&asset).unwrap();
        if let Err(err) = PSP22VariableDebtRef::mint(&variable_debt_token, borrower, amount) {
            return Err(Error::PSP22(err));
        }

        let collateral_token = self.data().ctokens.get(&asset).unwrap();
        if let Err(err) =
            PSP22Ref::transfer_from(&asset, collateral_token, borrower, amount, Vec::<u8>::new())
        {
            return Err(Error::PSP22(err));
        }

        Ok(())
    }

    default fn repay(&self, asset: AccountId, amount: Balance) -> Result<()> {
        // TODO only supported asset
        // TODO reject if repayer balance insufficient

        let repayer = Self::env().caller();

        // TODO accrue_interest

        let collateral_token = self.data().ctokens.get(&asset).unwrap();
        if let Err(err) =
            PSP22Ref::transfer_from(&asset, repayer, collateral_token, amount, Vec::<u8>::new())
        {
            return Err(Error::PSP22(err));
        }

        let variable_debt_token = self.data().vdtokens.get(&asset).unwrap();
        if let Err(err) = PSP22VariableDebtRef::burn(&asset, repayer, amount) {
            return Err(Error::PSP22(err));
        }

        Ok(())
    }
}
