use crate::traits::registry::*;
use openbrush::{
    storage::Mapping,
    traits::{AccountId, Storage},
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    rate_strategies: Mapping<AccountId, AccountId>,
    validators: Mapping<AccountId, AccountId>,
    default_rate_strategy: AccountId,
    default_validator: AccountId,
}

trait Internal {
    fn _rate_strategy(&self, asset: &AccountId) -> AccountId;
    fn _validator(&self, asset: &AccountId) -> AccountId;
}

impl<T: Storage<Data>> Registry for T {
    fn rate_strategy(&self, asset: AccountId) -> AccountId {
        self._rate_strategy(&asset)
    }

    fn validator(&self, asset: AccountId) -> AccountId {
        self._validator(&asset)
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _rate_strategy(&self, asset: &AccountId) -> AccountId {
        self.data()
            .rate_strategies
            .get(asset)
            .unwrap_or(self.data().default_rate_strategy)
    }
    default fn _validator(&self, asset: &AccountId) -> AccountId {
        self.data()
            .validators
            .get(asset)
            .unwrap_or(self.data().default_validator)
    }
}
