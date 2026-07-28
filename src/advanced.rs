#![allow(unsafe_op_in_unsafe_fn)]

use crate::functions::*;
use rust_decimal::{Decimal, MathematicalOps};
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One};
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[pyfunction]
/// Returns the logarithm of`number` to `base`.
/// ### Arguments
/// `base` - a number<br>
/// `number` - a number
/// ### Examples
/// ```python
/// print(log(100, 10)) # it will print 2
/// ```
/// ```python
/// print(log(8, 2)) # it will print 3
/// ```
pub fn log(number: Decimal, base: Decimal) -> PyResult<Decimal> {
    Ok((number.ln() / base.ln()).round_dp(12).normalize())
}

#[pyfunction]
/// Returns the result of tetration of `base` to height `n`.
/// ### Arguments
/// `a` - a positive integer (base)
/// `n` - a non-negative integer (the height)
/// ### Examples
/// ```python
/// print(tetration(2, 3)) # it will print 16
/// ```
/// ```python
/// print(tetration(3, 4)) 
/// # it will print:
/// # print(tetration(3, 4))
/// #       ~~~~~~~~~^^^^^^
/// # ValueError: Result is too large to calculate (Overflow)!
/// ```
/// ```python
/// print(tetration(3, 3)) # it will print 7625597484987
/// ```
pub fn tetration(a: i64, n: u32) -> PyResult<BigInt> {
    if a <= 0 {
        return Err(PyValueError::new_err("The base must be a positive integer."));
    }
    
    let base = BigInt::from_i64(a).unwrap();

    match n {
        0 => Ok(BigInt::one()),
        1 => Ok(base),
        2 => Ok(pow_bigint(&base, &base)), 
        _ => {
            let sub_height = tetration(a, n - 1)?;
            
            if sub_height > BigInt::from(100_000) {
                return Err(PyValueError::new_err("Result is too large to calculate (Overflow)!"));
            }

            Ok(pow_bigint(&base, &sub_height))
        }
    }
}
