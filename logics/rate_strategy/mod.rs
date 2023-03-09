use crate::math::{
    percent_mul,
    percentage_factor,
    ray,
    ray_div,
    ray_mul,
    wad_to_ray,
};
pub type BigEndian = Vec<u8>;

pub fn u256_from_be(be: BigEndian) -> U256 {
    u256_from_be_bytes_unchecked(be)
}
use crate::traits::rate_strategy::*;
use core::ops::{
    Add,
    Sub,
};
use ethnum::U256;
use ink::prelude::vec::Vec;
use openbrush::traits::{
    Balance,
    Storage,
};
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    optimal_utilization_rate: BigEndian,
    base_borrow_rate: BigEndian,
    rate_slope_1: BigEndian,
    rate_slope_2: BigEndian,
    excess_utilization_rate: BigEndian,
}

struct DefaultRateStrategyParam {
    optimal_utilization_rate: U256,
    base_borrow_rate: U256,
    rate_slope_1: U256,
    rate_slope_2: U256,
    excess_utilization_rate: U256,
}

struct CalculateInterestRatesInput {
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

#[allow(dead_code)]
struct CalculateInterestRatesOutput {
    current_liquidity_rate: U256,
    current_borrow_rate: U256,
}

fn u256_from_be_bytes_unchecked(vec: Vec<u8>) -> U256 {
    U256::from_be_bytes(into_slice(vec))
}

pub trait Internal {
    fn _calculate_rate(
        &self,
        _balance: Balance,
        _liquidity_added: Balance,
        _liquidity_taken: Balance,
        _total_debt: Balance,
        _reserve_factor: Balance,
    ) -> (BigEndian, BigEndian);
}

impl<T: Storage<Data>> RateStrategy for T {
    default fn calculate_rate(
        &self,
        _balance: Balance,
        _liquidity_added: Balance,
        _liquidity_taken: Balance,
        _total_debt: Balance,
        _reserve_factor: Balance,
    ) -> (BigEndian, BigEndian) {
        self._calculate_rate(
            _balance,
            _liquidity_added,
            _liquidity_taken,
            _total_debt,
            _reserve_factor,
        )
    }
}
impl Default for Data {
    fn default() -> Self {
        Self {
            optimal_utilization_rate: be_frrom_u256(U256::ZERO),
            base_borrow_rate: be_frrom_u256(U256::ZERO),
            rate_slope_1: be_frrom_u256(U256::ZERO),
            rate_slope_2: be_frrom_u256(U256::ZERO),
            excess_utilization_rate: be_frrom_u256(U256::ZERO),
        }
    }
}
impl<T: Storage<Data>> Internal for T {
    default fn _calculate_rate(
        &self,
        _balance: Balance,
        _liquidity_added: Balance,
        _liquidity_taken: Balance,
        _total_debt: Balance,
        _reserve_factor: Balance,
    ) -> (BigEndian, BigEndian) {
        let available_liquidity = _balance.add(_liquidity_added).sub(_liquidity_taken);

        let out = self
            .data()
            .calculate_interest_rates(CalculateInterestRatesInput {
                available_liquidity: U256::new(available_liquidity),
                total_debt: U256::new(_total_debt),
                reserve_factor: U256::new(_reserve_factor),
            });
        (
            be_frrom_u256(out.current_liquidity_rate),
            be_frrom_u256(out.current_borrow_rate),
        )
    }
}

fn into_slice<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
fn be_frrom_u256(val: U256) -> BigEndian {
    Vec::from(val.to_be_bytes())
}

impl Data {
    pub fn new(
        base_borrow_rate: BigEndian,
        optimal_utilization_rate: BigEndian,
        rate_slope_1: BigEndian,
        rate_slope_2: BigEndian,
    ) -> Self {
        Self {
            base_borrow_rate,
            optimal_utilization_rate: optimal_utilization_rate.clone(),
            rate_slope_1,
            rate_slope_2,
            excess_utilization_rate: Vec::from(
                ray()
                    .sub(u256_from_be_bytes_unchecked(optimal_utilization_rate))
                    .to_be_bytes(),
            ),
        }
    }

    fn to_param(&self) -> DefaultRateStrategyParam {
        let into_u256 = |val: Vec<u8>| u256_from_be_bytes_unchecked(val.clone());
        DefaultRateStrategyParam {
            base_borrow_rate: into_u256(self.base_borrow_rate.clone()),
            excess_utilization_rate: into_u256(self.excess_utilization_rate.clone()),
            optimal_utilization_rate: into_u256(self.optimal_utilization_rate.clone()),
            rate_slope_1: into_u256(self.rate_slope_1.clone()),
            rate_slope_2: into_u256(self.rate_slope_2.clone()),
        }
    }
    fn calculate_interest_rates(
        &self,
        input: CalculateInterestRatesInput,
    ) -> CalculateInterestRatesOutput {
        let param = self.to_param();
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
    if total_debt.eq(&U256::ZERO) {
        return U256::ZERO
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

    use crate::rate_strategy::*;
    use ethnum::U256;
    fn reserve_factor() -> U256 {
        U256::from_str_prefixed("1000").unwrap()
    }

    use crate::math::{
        percent_mul,
        percentage_factor,
        ray,
    };

    use super::{
        CalculateInterestRatesInput,
        Data,
        DefaultRateStrategyParam,
    };

    fn rate_strategy_param() -> DefaultRateStrategyParam {
        rate_strategy().to_param()
    }

    fn rate_strategy() -> Data {
        let to_u256 = |src: &str| U256::from_str_prefixed(src).unwrap();
        Data::new(
            be_frrom_u256(U256::ZERO),
            be_frrom_u256(ray().mul(to_u256("8")).div(to_u256("10"))), // 0.8
            be_frrom_u256(ray().mul(to_u256("4")).div(to_u256("100"))), // 4%
            be_frrom_u256(ray().mul(to_u256("75")).div(to_u256("100"))), // 75%
        )
    }
    #[test]
    fn test_with_empty_reserve() {
        let target = rate_strategy();
        let result = target.calculate_interest_rates(CalculateInterestRatesInput {
            available_liquidity: U256::ZERO,
            reserve_factor: reserve_factor(),
            total_debt: U256::ZERO,
        });
        assert_eq!(result.current_borrow_rate, U256::ZERO);
        assert_eq!(result.current_liquidity_rate, U256::ZERO);
    }
    #[test]
    fn test_rate_at_80_percent() {
        let target = rate_strategy();
        let result = target.calculate_interest_rates(CalculateInterestRatesInput {
            available_liquidity: U256::from_str_prefixed("200000000000000000").unwrap(),
            reserve_factor: reserve_factor(),
            total_debt: U256::from_str_prefixed("800000000000000000").unwrap(),
        });
        let expected_borrow_rate = rate_strategy_param().rate_slope_1;
        let expected_liquidity_rate = percent_mul(
            expected_borrow_rate
                .clone()
                .mul(U256::from_str_prefixed("8").unwrap())
                .div(U256::from_str_prefixed("10").unwrap()),
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
            available_liquidity: U256::ZERO,
            reserve_factor: reserve_factor(),
            total_debt: U256::from_str_prefixed("800000000000000000").unwrap(),
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
