use core::ops::{
    Add,
    Sub,
};

use numext_fixed_uint::U256;

use crate::math::{
    percent_mul,
    ray_div,
    ray_mul,
    PERCENTAGE_FACTOR,
    RAY,
};

pub struct DefaultRateStrategy {
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

impl DefaultRateStrategy {
    pub fn new(param: DefaultRateStrategyCreateParam) -> Self {
        Self {
            base_borrow_rate: param.base_borrow_rate,
            optimal_utilization_rate: param.optimal_utilization_rate.clone(),
            rate_slope_1: param.rate_slope_1,
            rate_slope_2: param.rate_slope_2,
            excess_utilization_rate: RAY.sub(param.optimal_utilization_rate),
        }
    }
    pub fn calculate_interest_rates(
        &self,
        input: CalculateInterestRatesInput,
    ) -> CalculateInterestRatesOutput {
        let mut vars = CalcInterestRatesLocalVars {
            total_debt: input.total_debt,
            current_borrow_rate: U256::zero(),
            current_liquidity_rate: U256::zero(),
            utilization_rate: U256::zero(),
        };
        vars.utilization_rate = if vars.total_debt.is_zero() {
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
            .gt(&self.optimal_utilization_rate)
        {
            let excess_utilization_rate_ratio = ray_div(
                vars.utilization_rate
                    .clone()
                    .sub(self.optimal_utilization_rate.clone()),
                self.excess_utilization_rate.clone(),
            )
            .unwrap();
            vars.current_borrow_rate = ray_mul(
                self.base_borrow_rate
                    .clone()
                    .add(self.rate_slope_1.clone())
                    .add(self.rate_slope_2.clone()),
                excess_utilization_rate_ratio,
            )
            .unwrap()
        } else {
            vars.current_borrow_rate = ray_div(
                ray_mul(
                    self.base_borrow_rate
                        .clone()
                        .add(vars.utilization_rate.clone()),
                    self.rate_slope_1.clone(),
                )
                .unwrap(),
                self.optimal_utilization_rate.clone(),
            )
            .unwrap()
        }
        vars.current_liquidity_rate = percent_mul(
            ray_mul(vars.current_borrow_rate.clone(), vars.utilization_rate).unwrap(),
            PERCENTAGE_FACTOR.sub(input.reserve_factor.clone()),
        )
        .unwrap();
        CalculateInterestRatesOutput {
            current_borrow_rate: vars.current_borrow_rate,
            current_liquidity_rate: vars.current_liquidity_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use core::{
        iter::empty,
        ops::{
            Div,
            Mul,
        },
    };

    use numext_fixed_uint::{
        u256,
        U256,
    };

    const ONE: U256 = u256!("1");

    use crate::{
        math::RAY,
        traits::rate_strategy::RateStrategy,
    };

    use super::{
        CalculateInterestRatesInput,
        DefaultRateStrategy,
        DefaultRateStrategyCreateParam,
    };

    fn rate_strategy() -> DefaultRateStrategy {
        DefaultRateStrategy::new(DefaultRateStrategyCreateParam {
            base_borrow_rate: U256::zero(),
            optimal_utilization_rate: RAY.mul(u256!("8")).div(u256!("10")), // 0.8
            rate_slope_1: RAY.mul(u256!("4")).div(u256!("100")),            // 4%
            rate_slope_2: RAY.mul(u256!("75")).div(u256!("100")),           // 75%
        })
    }
    #[test]
    fn test_with_empty_reserve() {
        let target = rate_strategy();
        let result = target.calculate_interest_rates(CalculateInterestRatesInput {
            available_liquidity: U256::zero(),
            reserve_factor: u256!("1000"),
            total_debt: U256::zero(),
        });
        assert_eq!(result.current_borrow_rate, u256!("0"));
        assert_eq!(result.current_liquidity_rate, u256!("0"));
    }
    #[test]
    fn test_rate_at_80_percent() {
        let target = rate_strategy();
        let result = target.calculate_interest_rates(CalculateInterestRatesInput {
            available_liquidity: u256!("200000000000000000"),
            reserve_factor: u256!("1000"),
            total_debt: u256!("800000000000000000"),
        });
        assert_eq!(result.current_borrow_rate, u256!("0"));
        // assert_eq!(result.current_liquidity_rate, u256!("0"));
    }
}
