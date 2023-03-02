use openbrush::traits::{
    AccountId,
    Balance,
};

#[openbrush::wrapper]
pub type RiskStrategyRef = dyn RiskStrategy;

#[openbrush::trait_definition]
pub trait RiskStrategy {
    #[ink(message)]
    fn price_oracle(&self) -> AccountId;

    #[ink(message)]
    fn validate_borrow(&self, account: AccountId, asset: AccountId, amount: Balance) -> Result<()>;

    #[ink(message)]
    fn validate_withdraw(
        &self,
        account: AccountId,
        asset: AccountId,
        amount: Balance,
    ) -> Result<()>;

    // receivable_amount
    #[ink(message)]
    fn validate_liquidation(
        &self,
        liquidatee: AccountId,
        collateral_asset: AccountId,
        debt_asset: AccountId,
        debt_amount: Balance,
    ) -> Result<Balance>;
}

pub type Result<T> = core::result::Result<T, u8>;
