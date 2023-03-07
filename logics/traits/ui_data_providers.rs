use openbrush::traits::{
    AccountId,
    Timestamp,
};
use scale::{
    Decode,
    Encode,
};

#[derive(Decode, Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct PoolData {
    pub registry: AccountId,
    pub asset: AccountId,
    pub collateral_token: AccountId,
    pub debt_token: AccountId,
    pub liquidity_share: u128,
    pub liquidity_index: u128,
    pub liquidity_rate: u128,
    pub debt_share: u128,
    pub debt_index: u128,
    pub debt_rate: u128,
    pub last_update_timestamp: Timestamp,
}

#[openbrush::wrapper]
pub type UIPoolDataProviderRef = dyn UIPoolDataProvider;

#[openbrush::trait_definition]
pub trait UIPoolDataProvider {
    #[ink(message)]
    fn pool_data(&self) -> PoolData;
}
