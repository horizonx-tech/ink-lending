use core::ops::{
    Add,
    Div,
    Mul,
    Sub,
};

use numext_fixed_uint::{
    u256,
    U256,
};

pub const WAD: U256 = u256!("1_000_000_000_000_000_000");
const HALF_WAD: U256 = u256!("500_000_000_000_000_000");
pub const RAY: U256 = u256!("1_000_000_000_000_000_000_000_000_000");
const HALF_RAY: U256 = u256!("500_000_000_000_000_000");
const ZERO: U256 = U256::zero();
pub const PERCENTAGE_FACTOR: U256 = u256!("10_000"); // percentage plus two decimals
const HALF_PERCENT: U256 = u256!("5_000");
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

pub fn percent_mul(value: U256, percentage: U256) -> Result<U256, Error> {
    if value.is_zero() || percentage.clone().is_zero() {
        return Ok(U256::zero())
    } else if value.gt(&U256::max_value().sub(HALF_PERCENT).div(percentage.clone())) {
        return Err(Error::CalcOverflow)
    }
    Ok(value
        .mul(percentage.add(HALF_PERCENT))
        .div(PERCENTAGE_FACTOR))
}

pub fn percent_div(value: U256, percentage: U256) -> Result<U256, Error> {
    let half_percentage = percentage.clone().div(u256!("2"));
    if percentage.clone().is_zero() {
        return Err(Error::DivByZero)
    } else if value.gt(&U256::max_value()
        .sub(half_percentage.clone())
        .div(PERCENTAGE_FACTOR))
    {
        return Err(Error::CalcOverflow)
    }
    Ok(value
        .mul(PERCENTAGE_FACTOR.add(half_percentage.clone()))
        .div(percentage))
}

struct Wad {}
struct Ray {}
impl Precision for Wad {
    fn precision(&self) -> U256 {
        WAD
    }
    fn precision_half(&self) -> U256 {
        HALF_WAD
    }
}
impl Precision for Ray {
    fn precision(&self) -> U256 {
        RAY
    }
    fn precision_half(&self) -> U256 {
        HALF_RAY
    }
}

fn _mul(a: U256, b: U256, precision: &dyn Precision) -> Result<U256, Error> {
    if a == ZERO || b == ZERO {
        Ok(ZERO)
    } else if a.gt(&U256::max_value()
        .sub(precision.precision_half())
        .div(b.clone()))
    {
        Err(Error::CalcOverflow)
    } else {
        Ok(a.mul(b).add(precision.precision_half()).div(WAD))
    }
}

fn _div(a: U256, b: U256, precision: &dyn Precision) -> Result<U256, Error> {
    let half_b = b.clone().div(u256!("2"));
    if b == ZERO {
        Err(Error::DivByZero)
    } else if a.gt(&U256::max_value().sub(half_b.clone().div(precision.precision()))) {
        Err(Error::CalcOverflow)
    } else {
        Ok(a.mul(precision.precision()).add(half_b).div(b))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const ONE_ETHER: U256 = u256!("1_000_000_000_000_000_000");
    const TWO_ETHER: U256 = u256!("2_000_000_000_000_000_000");
    // 2 ** 128 -1
    const U128_MAX: U256 = u256!("340282366920938463463374607431768211455");
    #[test]
    fn test_add() {
        assert_eq!(ZERO.add(ZERO), ZERO);
        assert_eq!(ZERO.add(u256!("1")), u256!("1"));
        assert_eq!(u256!("1").add(u256!("1")), u256!("2"));
    }
    #[test]
    fn test_sub() {
        assert_eq!(ZERO.sub(ZERO), ZERO);
        assert_eq!(u256!("1").sub(u256!("1")), ZERO);
        assert_eq!(u256!("2").sub(u256!("1")), u256!("1"));
    }

    #[test]
    fn test_wad_mul_overflow() {
        let max = u256!("340282366920938463463374607431768211456");
        assert_eq!(
            wad_mul(max.clone(), max.clone()).unwrap_err(),
            Error::CalcOverflow
        )
    }
    #[test]
    fn test_wad_mul() {
        assert_eq!(wad_mul(U128_MAX, ONE_ETHER).unwrap(), U128_MAX);
        assert_eq!(wad_mul(ZERO, ZERO).unwrap(), ZERO);
        assert_eq!(wad_mul(ZERO, ONE_ETHER).unwrap(), ZERO);
        assert_eq!(wad_mul(ONE_ETHER, ONE_ETHER).unwrap(), ONE_ETHER)
    }

    #[test]
    fn tet_wad_mul_fractions() {
        let _02_ether: U256 = u256!("200_000_000_000_000_000");
        let _04_ether: U256 = u256!("400_000_000_000_000_000");
        assert_eq!(
            wad_mul(ONE_ETHER, _02_ether.clone()).unwrap(),
            _02_ether.clone()
        );
        assert_eq!(
            wad_mul(TWO_ETHER, _02_ether.clone()).unwrap(),
            _04_ether.clone()
        )
    }
    #[test]
    fn test_wad_div_by_zero() {
        assert_eq!(wad_div(ONE_ETHER, ZERO).unwrap_err(), Error::DivByZero);
    }

    #[test]
    fn test_wad_div() {
        assert_eq!(wad_div(ZERO, ONE_ETHER).unwrap(), ZERO);
        assert_eq!(wad_div(ONE_ETHER, ONE_ETHER).unwrap(), ONE_ETHER);
    }
    #[test]
    fn test_wad_div_fractions() {
        let _05_ehther: U256 = u256!("500_000_000_000_000_000");
        assert_eq!(wad_div(ONE_ETHER, TWO_ETHER).unwrap(), _05_ehther.clone());
        assert_eq!(wad_div(TWO_ETHER, TWO_ETHER).unwrap(), ONE_ETHER);
    }

    #[test]
    fn test_was_mul_rounding() {
        let a: U256 = u256!("950_000_000_000_005_647");
        let b: U256 = u256!("10000000000");
        let c: U256 = u256!("9500000000");
        assert_eq!(wad_mul(a.clone(), b.clone()).unwrap(), c.clone());
        assert_eq!(wad_mul(b.clone(), a.clone()).unwrap(), c.clone());
    }
}
