use core::ops::{
    Add,
    Div,
    Mul,
    Sub,
};

use ethnum::{
    u256,
    U256,
};

const WAD: U256 = u256::new(1_000_000_000_000_000_000);
const HALF_WAD: U256 = u256::new(500_000_000_000_000_000);
const RAY: U256 = u256::new(1_000_000_000_000_000_000_000_000_000);
const HALF_RAY: U256 = u256::new(500_000_000_000_000_000);
const ZERO: U256 = U256::ZERO;

#[derive(Debug, PartialEq)]
pub enum Error {
    MulOverflow,
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
    } else if a.gt(&U256::MAX.sub(precision.precision_half()).div(b.clone())) {
        Err(Error::MulOverflow)
    } else {
        Ok(a.mul(b).add(precision.precision_half()).div(WAD))
    }
}

fn _div(a: U256, b: U256, precision: &dyn Precision) -> Result<U256, Error> {
    let half_b = b.clone().div(u256::new(2));
    if b == ZERO {
        Err(Error::DivByZero)
    } else if a.gt(&U256::MAX.sub(half_b.clone().div(precision.precision()))) {
        Err(Error::MulOverflow)
    } else {
        Ok(a.mul(precision.precision()).add(half_b).div(b))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const ONE_ETHER: U256 = u256::new(1_000_000_000_000_000_000);
    const TWO_ETHER: U256 = u256::new(2_000_000_000_000_000_000);
    // 2 ** 128 -1
    const U128_MAX: U256 = u256::new(340_282_366_920_938_463_463_374_607_431_768_211_455u128);
    #[test]
    fn test_add() {
        assert_eq!(ZERO.add(ZERO), ZERO);
        assert_eq!(ZERO.add(u256::ONE), u256::ONE);
        assert_eq!(u256::ONE.add(u256::ONE), u256::new(2));
    }
    #[test]
    fn test_sub() {
        assert_eq!(ZERO.sub(ZERO), ZERO);
        assert_eq!(u256::ONE.sub(u256::ONE), ZERO);
        assert_eq!(u256::new(2).sub(u256::ONE), u256::ONE);
    }

    #[test]
    fn test_wad_mul_overflow() {
        // u128 max + 1
        let max =
            u256::new(340_282_366_920_938_463_463_374_607_431_768_211_455u128).add(u256::new(1));
        assert_eq!(
            wad_mul(max.clone(), max.clone()).unwrap_err(),
            Error::MulOverflow
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
        let _02_ether: U256 = u256::new(200_000_000_000_000_000);
        let _04_ether: U256 = u256::new(400_000_000_000_000_000);
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
        let _05_ehther: U256 = u256::new(500_000_000_000_000_000);
        assert_eq!(wad_div(ONE_ETHER, TWO_ETHER).unwrap(), _05_ehther.clone());
        assert_eq!(wad_div(TWO_ETHER, TWO_ETHER).unwrap(), ONE_ETHER);
    }

    #[test]
    fn test_was_mul_rounding() {
        let a: U256 = u256::new(950_000_000_000_005_647);
        let b: U256 = u256::new(10000000000);
        let c: U256 = u256::new(9500000000);
        assert_eq!(wad_mul(a.clone(), b.clone()).unwrap(), c.clone());
        assert_eq!(wad_mul(b.clone(), a.clone()).unwrap(), c.clone());
    }
}
