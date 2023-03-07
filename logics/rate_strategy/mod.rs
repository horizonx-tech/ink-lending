use core::ops::{
    Add,
    Sub,
};

use openbrush::traits::String;
use primitive_types::U256;
use std::string::String as stdStr;

use crate::math::{
    percent_mul,
    percentage_factor,
    ray,
    ray_div,
    ray_mul,
    wad_to_ray,
};
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(DefaultRateStrategy);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct DefaultRateStrategy {
    optimal_utilization_rate: String,
    base_borrow_rate: String,
    rate_slope_1: String,
    rate_slope_2: String,
    excess_utilization_rate: String,
}

pub struct DefaultRateStrategyParam {
    optimal_utilization_rate: U256,
    base_borrow_rate: U256,
    rate_slope_1: U256,
    rate_slope_2: U256,
    excess_utilization_rate: U256,
}

pub struct DefaultRateStrategyCreateParam {
    optimal_utilization_rate: U256,
    base_borrow_rate: U256,
    rate_slope_1: U256,
    rate_slope_2: U256,
}

pub struct CalculateInterestRatesInput {
    available_liquidity: U256,
    total_debt: U256,
    reserve_factor: U256,
}

struct CalcInterestRatesLocalVars {
    total_debt: U256,
    current_borrow_rate: U256,
    current_liquidity_rate: U256,
    utilization_rate: U256,
}

pub struct CalculateInterestRatesOutput {
    current_liquidity_rate: U256,
    current_borrow_rate: U256,
}

fn u256_from_str_unchecked(str: &String) -> U256 {
    U256::from_dec_str(&stdStr::from_utf8_lossy(str)).unwrap()
}

impl DefaultRateStrategy {
    pub fn new(param: DefaultRateStrategyCreateParam) -> Self {
        Self {
            base_borrow_rate: String::from(param.base_borrow_rate.to_string()),
            optimal_utilization_rate: String::from(
                param.optimal_utilization_rate.clone().to_string(),
            ),
            rate_slope_1: String::from(param.rate_slope_1.to_string()),
            rate_slope_2: String::from(param.rate_slope_2.to_string()),
            excess_utilization_rate: String::from(
                ray().sub(param.optimal_utilization_rate).to_string(),
            ),
        }
    }

    fn to_param(&self) -> DefaultRateStrategyParam {
        DefaultRateStrategyParam {
            base_borrow_rate: u256_from_str_unchecked(&self.base_borrow_rate),
            excess_utilization_rate: u256_from_str_unchecked(&self.excess_utilization_rate),
            optimal_utilization_rate: u256_from_str_unchecked(&self.optimal_utilization_rate),
            rate_slope_1: u256_from_str_unchecked(&self.rate_slope_1),
            rate_slope_2: u256_from_str_unchecked(&self.rate_slope_2),
        }
    }
    pub fn calculate_interest_rates(
        &self,
        input: CalculateInterestRatesInput,
    ) -> CalculateInterestRatesOutput {
        let param = self.to_param();
        let mut vars = CalcInterestRatesLocalVars {
            total_debt: input.total_debt,
            current_borrow_rate: U256::zero(),
            current_liquidity_rate: U256::zero(),
            utilization_rate: U256::zero(),
        };
        vars.utilization_rate = if vars.total_debt.eq(&U256::zero()) {
            U256::zero()
        } else {
            ray_div(
                vars.total_debt.clone(),
                input.available_liquidity.add(vars.total_debt.clone()),
            )
            .unwrap()
        };
        if vars
            .utilization_rate
            .clone()
            .gt(&param.optimal_utilization_rate)
        {
            let excess_utilization_rate_ratio = ray_div(
                vars.utilization_rate
                    .clone()
                    .sub(param.optimal_utilization_rate.clone()),
                param.excess_utilization_rate.clone(),
            )
            .unwrap();
            vars.current_borrow_rate = ray_mul(
                param
                    .base_borrow_rate
                    .clone()
                    .add(param.rate_slope_1.clone())
                    .add(param.rate_slope_2.clone()),
                excess_utilization_rate_ratio,
            )
            .unwrap()
        } else {
            vars.current_borrow_rate = ray_div(
                ray_mul(
                    param
                        .base_borrow_rate
                        .clone()
                        .add(vars.utilization_rate.clone()),
                    param.rate_slope_1.clone(),
                )
                .unwrap(),
                param.optimal_utilization_rate.clone(),
            )
            .unwrap()
        }
        vars.current_liquidity_rate = percent_mul(
            ray_mul(
                overall_borrow_rate(&vars.total_debt, &vars.current_borrow_rate),
                vars.utilization_rate,
            )
            .unwrap(),
            percentage_factor().sub(input.reserve_factor.clone()),
        )
        .unwrap();
        CalculateInterestRatesOutput {
            current_borrow_rate: vars.current_borrow_rate,
            current_liquidity_rate: vars.current_liquidity_rate,
        }
    }
}

// TODO: refactor: unnecessary calculation
fn overall_borrow_rate(total_debt: &U256, borrow_rate: &U256) -> U256 {
    if total_debt.eq(&U256::zero()) {
        return U256::zero()
    }
    let rate = ray_mul(wad_to_ray(*total_debt).unwrap(), *borrow_rate).unwrap();
    ray_div(rate, wad_to_ray(*total_debt).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use core::ops::{
        Add,
        Div,
        Mul,
        Sub,
    };

    fn reserve_factor() -> U256 {
        U256::from_dec_str("1000").unwrap()
    }

    use primitive_types::U256;

    use crate::{
        math::{
            percent_mul,
            percentage_factor,
            ray,
        },
        traits::rate_strategy::RateStrategy,
    };

    use super::{
        CalculateInterestRatesInput,
        DefaultRateStrategy,
        DefaultRateStrategyCreateParam,
        DefaultRateStrategyParam,
    };

    fn rate_strategy_param() -> DefaultRateStrategyParam {
        rate_strategy().to_param()
    }

    fn rate_strategy() -> DefaultRateStrategy {
        DefaultRateStrategy::new(DefaultRateStrategyCreateParam {
            base_borrow_rate: U256::zero(),
            optimal_utilization_rate: ray()
                .mul(U256::from_dec_str("8").unwrap())
                .div(U256::from_dec_str("10").unwrap()), // 0.8
            rate_slope_1: ray()
                .mul(U256::from_dec_str("4").unwrap())
                .div(U256::from_dec_str("100").unwrap()), // 4%
            rate_slope_2: ray()
                .mul(U256::from_dec_str("75").unwrap())
                .div(U256::from_dec_str("100").unwrap()), // 75%
        })
    }
    #[test]
    fn test_with_empty_reserve() {
        let target = rate_strategy();
        let result = target.calculate_interest_rates(CalculateInterestRatesInput {
            available_liquidity: U256::zero(),
            reserve_factor: reserve_factor(),
            total_debt: U256::zero(),
        });
        assert_eq!(result.current_borrow_rate, U256::zero());
        assert_eq!(result.current_liquidity_rate, U256::zero());
    }
    #[test]
    fn test_rate_at_80_percent() {
        let target = rate_strategy();
        let result = target.calculate_interest_rates(CalculateInterestRatesInput {
            available_liquidity: U256::from_dec_str("200000000000000000").unwrap(),
            reserve_factor: reserve_factor(),
            total_debt: U256::from_dec_str("800000000000000000").unwrap(),
        });
        let expected_borrow_rate = rate_strategy_param().rate_slope_1;
        let expected_liquidity_rate = percent_mul(
            expected_borrow_rate
                .clone()
                .mul(U256::from_dec_str("8").unwrap())
                .div(U256::from_dec_str("10").unwrap()),
            percentage_factor().sub(reserve_factor()),
        )
        .unwrap();
        assert_eq!(result.current_liquidity_rate, expected_liquidity_rate);
        assert_eq!(result.current_borrow_rate, expected_borrow_rate);
    }
    #[test]
    fn test_rate_at_100_percent() {
        let target = rate_strategy();
        let result = target.calculate_interest_rates(CalculateInterestRatesInput {
            available_liquidity: U256::zero(),
            reserve_factor: reserve_factor(),
            total_debt: U256::from_dec_str("800000000000000000").unwrap(),
        });
        let expected_borrow_rate = rate_strategy_param()
            .rate_slope_1
            .add(rate_strategy_param().rate_slope_2);
        let expected_liquidity_rate = percent_mul(
            expected_borrow_rate.clone(),
            percentage_factor().sub(reserve_factor()),
        )
        .unwrap();
        assert_eq!(result.current_liquidity_rate, expected_liquidity_rate);
        assert_eq!(result.current_borrow_rate, expected_borrow_rate);
    }
}
