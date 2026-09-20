#![allow(unsafe_op_in_unsafe_fn)]

use rust_decimal::Decimal;
use num_traits::ToPrimitive;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[pyfunction]
/// Returns `number` rounded down.
/// ### Arguments
/// `number` - a number
/// ### Examples
/// ```python
/// # 1
/// print(floor(5.7)) # it will print 5
/// 
/// # 2
/// print(floor(99.99)) # it will print 99
/// ```
pub fn floor(mut number: Decimal) -> PyResult<i64> {
    number = number.floor();
    Ok(number.to_i64().ok_or_else(|| PyValueError::new_err("Number is too large for i64"))?)
}

#[pyfunction]
/// Returns `number` rounded up.
/// ### Arguments
/// `number` - a number
/// ### Examples
/// ```python
/// # 1
/// print(ceil(99.1)) # it will print 100
/// 
/// # 2
/// print(ceil(54.01)) # it will print 55
/// ```
pub fn ceil(number: Decimal) -> PyResult<i64> {
    Ok(number.ceil().to_i64().ok_or_else(|| PyValueError::new_err("Number is too large for i64"))?)
}

#[pyfunction]
/// Returns the absolute value of `number`.
/// ### Arguments
/// `number` - a number
/// ### Examples
/// ```python
/// # 1
/// print(absolute(-3)) # it will print 3
/// 
/// # 2
/// print(absolute(3)) # it will print 3
/// ```
pub fn absolute(number: Decimal) -> PyResult<Decimal> {
    Ok(number.abs())
}
