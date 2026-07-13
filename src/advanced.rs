#![allow(unsafe_op_in_unsafe_fn)]

use crate::functions::*;
use rust_decimal::{Decimal, MathematicalOps};
use num_bigint::BigInt;
use num_traits::{FromPrimitive, One};
use pyo3::prelude::*;

#[pyfunction]
/// Returns the logarithm of`number` to `base`.
/// ### Arguments
/// `base` - a number<br>
/// `number` - a number
/// ### Examples
/// ```python
/// print(log(100, 10)) # it will print 2.0
/// ```
/// ```python
/// print(log(8, 2)) # it will print 3.0
/// ```
pub fn log(number: Decimal, base: Decimal) -> PyResult<Decimal> {
    Ok(number.ln() / base.ln())
}

#[pyfunction]
/// Returns the result of tetration of `base` to height `n`.
/// ### Arguments
/// `a` - a positive integer (base)
/// `n` - a non-negative integer (the height)
pub fn tetration(a: i64, n: u32) -> PyResult<BigInt> {
    if a <= 0 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "The base must be positive integer.",
        ));
    }
    let base = BigInt::from_i64(a).unwrap();

    match n {
        0 => Ok(BigInt::one()),
        1 => Ok(base),
        _ => {
            let mut result = base.clone();
            for _ in 1..n {
                result = pow_bigint(&base, &result);
            }
            Ok(result)
        }
    }
}
