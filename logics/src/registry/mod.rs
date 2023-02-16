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
    risk_strategies: Mapping<AccountId, AccountId>,
    default_rate_strategy: AccountId,
    default_risk_strategy: AccountId,
}

trait Internal {
    fn _rate_strategy(&self, asset: &AccountId) -> AccountId;
    fn _risk_strategy(&self, asset: &AccountId) -> AccountId;
}

impl<T: Storage<Data>> Registry for T {
    fn rate_strategy(&self, asset: AccountId) -> AccountId {
        self._rate_strategy(&asset)
    }

    fn risk_strategy(&self, asset: AccountId) -> AccountId {
        self._risk_strategy(&asset)
    }
}

impl<T: Storage<Data>> Internal for T {
    default fn _rate_strategy(&self, asset: &AccountId) -> AccountId {
        self.data()
            .rate_strategies
            .get(asset)
            .unwrap_or(self.data().default_rate_strategy)
    }
    default fn _risk_strategy(&self, asset: &AccountId) -> AccountId {
        self.data()
            .risk_strategies
            .get(asset)
            .unwrap_or(self.data().default_risk_strategy)
    }
}