#![allow(unsafe_op_in_unsafe_fn)]

use num_bigint::BigInt;
use num_traits::{One, ToPrimitive, Zero};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

// checks if number is finite
pub(crate) fn check_is_finite(number: f64) -> PyResult<f64> {
    if number.is_finite() {
        Ok(number)
    } else {
        Err(PyValueError::new_err("Number must be finite"))
    }
}

pub(crate) fn empty<T>(args: &[T]) -> PyResult<&[T]> {
    if args.is_empty() {
        return Err(PyValueError::new_err(
            "Function takes at least one argument.",
        ));
    } else {
        Ok(args)
    }
}

pub(crate) fn pow_bigint(base: &BigInt, exponent: &BigInt) -> BigInt {
    if exponent.is_zero() {
        return BigInt::one();
    }
    if exponent.is_one() {
        return base.clone();
    }

    if let Some(exp_u32) = exponent.to_u32() {
        return base.pow(exp_u32);
    }

    let mut result = BigInt::one();
    let mut base_power = base.clone();
    let mut exp = exponent.clone();

    while !exp.is_zero() {
        if (&exp & &BigInt::one()).is_one() {
            result *= &base_power;
        }
        base_power = &base_power * &base_power;
        exp >>= 1;
    }
    result
}
