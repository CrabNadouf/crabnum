#![allow(unsafe_op_in_unsafe_fn)]

use num_bigint::{BigInt};
use num_traits::{FromPrimitive, One};
use pyo3::prelude::*;
use crate::functions::*;


#[pyfunction]
/// Returns the logarithm of`number` to `base`.
/// ### Arguments
/// `base` - a float number<br>
/// `number` - a float number
/// ### Examples
/// ```python
/// print(log(10, 100)) # it will print 2.0
/// ```
/// ```python
/// print(log(2, 8)) # it will print 3.0
/// ```
pub fn log(base: f64, number: f64) -> PyResult<f64> {
    check_is_finite(base)?;
    check_is_finite(number)?;
    Ok(number.log(base))
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
