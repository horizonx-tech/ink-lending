pub use crate::traits::ui_data_providers::*;
use crate::{
    asset_pool,
    traits::{
        asset_pool::AssetPool,
        shares::SharesRef,
    },
};

pub trait Internal {
    fn _pool_data(&self) -> PoolData;
}

impl<T: AssetPool + asset_pool::Internal> UIPoolDataProvider for T {
    default fn pool_data(&self) -> PoolData {
        self._pool_data()
    }
}

impl<T: AssetPool + asset_pool::Internal> Internal for T {
    default fn _pool_data(&self) -> PoolData {
        let mut data: PoolData = self._data().into();
        data.liquidity_share = SharesRef::total_share(&data.collateral_token);
        data.debt_share = SharesRef::total_share(&data.debt_token);
        data
    }
}

impl From<asset_pool::Data> for PoolData {
    fn from(item: asset_pool::Data) -> PoolData {
        PoolData {
            registry: item.registry,
            asset: item.asset,
            collateral_token: item.collateral_token,
            debt_token: item.debt_token,
            liquidity_share: 0,
            liquidity_index: item.liquidity_index,
            liquidity_rate: item.liquidity_rate,
            debt_share: 0,
            debt_index: item.debt_index,
            debt_rate: item.debt_rate,
            last_update_timestamp: item.last_update_timestamp,
        }
    }
}
