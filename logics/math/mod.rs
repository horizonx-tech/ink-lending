use core::{
    ops::{
        Add,
        Div,
        Mul,
        Sub,
    },
    str::FromStr,
};

use ethnum::U256;
const ZERO: U256 = U256::ZERO;
#[derive(Debug, PartialEq)]
pub enum Error {
    CalcOverflow,
    DivByZero,
}

trait Precision {
    fn precision(&self) -> U256;
    fn precision_half(&self) -> U256;
}

pub fn wad_div(a: U256, b: U256) -> Result<U256, Error> {
    _div(a, b, &Wad {})
}

pub fn wad_mul(a: U256, b: U256) -> Result<U256, Error> {
    _mul(a, b, &Wad {})
}
pub fn ray_div(a: U256, b: U256) -> Result<U256, Error> {
    _div(a, b, &Ray {})
}

pub fn ray_mul(a: U256, b: U256) -> Result<U256, Error> {
    _mul(a, b, &Ray {})
}

pub fn percentage_factor() -> U256 {
    U256::new(10000)
}

fn half_percent() -> U256 {
    percentage_factor().div(U256::new(2))
}

fn wad_way_ratio() -> U256 {
    U256::new(1000000000)
}

pub fn percent_mul(value: U256, percentage: U256) -> Result<U256, Error> {
    if value.eq(&ZERO) || percentage.clone().eq(&ZERO) {
        return Ok(ZERO)
    } else if value.gt(&U256::MAX.sub(half_percent()).div(percentage.clone())) {
        return Err(Error::CalcOverflow)
    }
    Ok(value
        .mul(percentage)
        .add(half_percent())
        .div(percentage_factor()))
}

pub fn wad_to_ray(a: U256) -> Result<U256, Error> {
    let result = a.clone().mul(wad_way_ratio());
    if result.clone().div(wad_way_ratio()).ne(&a.clone()) {
        return Err(Error::CalcOverflow)
    }
    Ok(result)
}

pub fn percent_div(value: U256, percentage: U256) -> Result<U256, Error> {
    let half_percentage = percentage.clone().div(U256::new(2));
    if percentage.clone().eq(&ZERO) {
        return Err(Error::DivByZero)
    } else if value.gt(&U256::MAX
        .sub(half_percentage.clone())
        .div(percentage_factor()))
    {
        return Err(Error::CalcOverflow)
    }
    Ok(value
        .mul(percentage_factor().add(half_percentage.clone()))
        .div(percentage))
}
pub fn ray() -> U256 {
    U256::new(1000000000000000000000000000)
}
struct Wad {}
struct Ray {}
impl Precision for Wad {
    fn precision(&self) -> U256 {
        U256::new(1000000000000000000)
    }
    fn precision_half(&self) -> U256 {
        U256::new(500000000000000000)
    }
}
impl Precision for Ray {
    fn precision(&self) -> U256 {
        ray()
    }
    fn precision_half(&self) -> U256 {
        ray().div(U256::from_str("2").unwrap())
    }
}

fn _mul(a: U256, b: U256, precision: &dyn Precision) -> Result<U256, Error> {
    if a == ZERO || b == ZERO {
        Ok(ZERO)
    } else if a.gt(&U256::MAX.sub(precision.precision_half()).div(b.clone())) {
        Err(Error::CalcOverflow)
    } else {
        Ok(a.mul(b)
            .add(precision.precision_half())
            .div(precision.precision()))
    }
}

fn _div(a: U256, b: U256, precision: &dyn Precision) -> Result<U256, Error> {
    let half_b = b.clone().div(U256::new(2));
    if b == ZERO {
        Err(Error::DivByZero)
    } else if a.gt(&U256::MAX.sub(half_b.clone().div(precision.precision()))) {
        Err(Error::CalcOverflow)
    } else {
        Ok(a.mul(precision.precision()).add(half_b).div(b))
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // 2 ** 128 -1
    const U128_MAX: u128 = u128::MAX;
    fn one_ether() -> U256 {
        U256::new(1000000000000000000)
    }

    fn two_ether() -> U256 {
        one_ether().mul(U256::new(2))
    }

    #[test]
    fn test_add() {
        assert_eq!(ZERO.add(ZERO), ZERO);
        assert_eq!(ZERO.add(U256::ONE), U256::ONE);
        assert_eq!(U256::ONE.add(U256::ONE), U256::new(2));
    }
    #[test]
    fn test_sub() {
        assert_eq!(ZERO.sub(ZERO), ZERO);
        assert_eq!(U256::ONE.sub(U256::ONE), ZERO);
        assert_eq!(U256::new(2).sub(U256::ONE), U256::ONE);
    }

    #[test]
    fn test_wad_mul_overflow() {
        // u128 max + 1
        let max = U256::new(2).pow(128).add(U256::ONE);
        assert_eq!(
            wad_mul(max.clone(), max.clone()).unwrap_err(),
            Error::CalcOverflow
        )
    }
    #[test]
    fn test_wad_mul() {
        let u128_max_minus_one = U256::new(2).pow(128).sub(U256::ONE);
        assert_eq!(
            wad_mul(u128_max_minus_one.clone(), one_ether()).unwrap(),
            u128_max_minus_one.clone()
        );
        assert_eq!(wad_mul(ZERO, ZERO).unwrap(), ZERO);
        assert_eq!(wad_mul(ZERO, one_ether()).unwrap(), ZERO);
        assert_eq!(wad_mul(one_ether(), one_ether()).unwrap(), one_ether())
    }

    #[test]
    fn tet_wad_mul_fractions() {
        let _02_ether: U256 = U256::new(200000000000000000);
        let _04_ether: U256 = U256::new(400000000000000000);
        assert_eq!(
            wad_mul(one_ether(), _02_ether.clone()).unwrap(),
            _02_ether.clone()
        );
        assert_eq!(
            wad_mul(two_ether(), _02_ether.clone()).unwrap(),
            _04_ether.clone()
        )
    }
    #[test]
    fn test_wad_div_by_zero() {
        assert_eq!(wad_div(one_ether(), ZERO).unwrap_err(), Error::DivByZero);
    }

    #[test]
    fn test_wad_div() {
        assert_eq!(wad_div(ZERO, one_ether()).unwrap(), ZERO);
        assert_eq!(wad_div(one_ether(), one_ether()).unwrap(), one_ether());
    }
    #[test]
    fn test_wad_div_fractions() {
        let _05_ehther: U256 = U256::new(500000000000000000);
        assert_eq!(
            wad_div(one_ether(), two_ether()).unwrap(),
            _05_ehther.clone()
        );
        assert_eq!(wad_div(two_ether(), two_ether()).unwrap(), one_ether());
    }

    #[test]
    fn test_was_mul_rounding() {
        let a: U256 = U256::new(950000000000005647);
        let b: U256 = U256::new(10000000000);
        let c: U256 = U256::new(9500000000);
        assert_eq!(wad_mul(a.clone(), b.clone()).unwrap(), c.clone());
        assert_eq!(wad_mul(b.clone(), a.clone()).unwrap(), c.clone());
    }
}
