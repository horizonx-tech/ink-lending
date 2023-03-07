use core::ops::{
    Add,
    Sub,
};

use ethnum::U256;

use crate::math::{
    percent_mul,
    ray_div,
    ray_mul,
    wad_to_ray,
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
            current_borrow_rate: U256::ZERO,
            current_liquidity_rate: U256::ZERO,
            utilization_rate: U256::ZERO,
        };
        vars.utilization_rate = if vars.total_debt.eq(&U256::ZERO) {
            U256::ZERO
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
            ray_mul(
                // wad_to_ray(vars.current_borrow_rate.clone()).unwrap(),
                // vars.current_borrow_rate.clone(),
                overall_borrow_rate(&vars.total_debt, &vars.current_borrow_rate),
                vars.utilization_rate,
            )
            .unwrap(),
            PERCENTAGE_FACTOR.sub(input.reserve_factor.clone()),
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
    if total_debt.eq(&U256::ZERO) {
        return U256::ZERO
    }
    let rate = ray_mul(wad_to_ray(*total_debt).unwrap(), *borrow_rate).unwrap();
    ray_div(rate, wad_to_ray(*total_debt).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use core::{
        borrow::Borrow,
        iter::empty,
        ops::{
            Div,
            Mul,
            Sub,
        },
    };

    const RESERVE_FACTOR: U256 = U256::new(1000);
    use ethnum::{
        u256,
        U256,
    };

    use crate::{
        math::{
            percent_mul,
            PERCENTAGE_FACTOR,
            RAY,
        },
        traits::rate_strategy::RateStrategy,
    };

    use super::{
        CalculateInterestRatesInput,
        DefaultRateStrategy,
        DefaultRateStrategyCreateParam,
    };

    fn rate_strategy() -> DefaultRateStrategy {
        DefaultRateStrategy::new(DefaultRateStrategyCreateParam {
            base_borrow_rate: U256::ZERO,
            optimal_utilization_rate: RAY.mul(U256::new(8)).div(U256::new(10)), // 0.8
            rate_slope_1: RAY.mul(U256::new(4)).div(U256::new(100)),            // 4%
            rate_slope_2: RAY.mul(U256::new(75)).div(U256::new(100)),           // 75%
        })
    }
    #[test]
    fn test_with_empty_reserve() {
        let target = rate_strategy();
        let result = target.calculate_interest_rates(CalculateInterestRatesInput {
            available_liquidity: U256::ZERO,
            reserve_factor: RESERVE_FACTOR,
            total_debt: U256::ZERO,
        });
        assert_eq!(result.current_borrow_rate, U256::ZERO);
        assert_eq!(result.current_liquidity_rate, U256::ZERO);
    }
    #[test]
    fn test_rate_at_80_percent() {
        let target = rate_strategy();
        let result = target.calculate_interest_rates(CalculateInterestRatesInput {
            available_liquidity: U256::new(200000000000000000),
            reserve_factor: RESERVE_FACTOR,
            total_debt: U256::new(800000000000000000),
        });
        let expected_borrow_rate = rate_strategy().rate_slope_1;
        let expected_liquidity_rate = percent_mul(
            expected_borrow_rate
                .clone()
                .mul(u256::new(8))
                .div(U256::new(10)),
            PERCENTAGE_FACTOR.sub(RESERVE_FACTOR),
        )
        .unwrap();
        assert_eq!(result.current_liquidity_rate, expected_liquidity_rate);
        assert_eq!(result.current_borrow_rate, expected_borrow_rate);
    }
}
