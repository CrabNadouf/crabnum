#![allow(unsafe_op_in_unsafe_fn)]

use pyo3::prelude::*;
use crate::functions::*;

#[pyfunction]
/// Returns `number` rounded down.
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// print(floor(5.7)) # it will print 5
/// ```
/// ```python
/// print(floor(99.99)) # it will print 99
/// ```
pub fn floor(mut number: f64) -> PyResult<i64> {
    number = number.floor();
    check_is_finite(number)?;
    Ok(number as i64)
}

#[pyfunction]
/// Returns `number` rounded up.
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// print(ceil(99.1)) # it will print 100
/// ```
/// ```python
/// print(ceil(54.33)) # it will print 55
/// ```
pub fn ceil(number: f64) -> PyResult<i64> {
    check_is_finite(number)?;
    Ok(number.ceil() as i64)
}

#[pyfunction]
/// Returns the absolute value of `number`.
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// print(absolute(-3)) # it will print 3
/// ```
/// ```python
/// print(absolute(3)) # it will print 3.0
/// ```
pub fn absolute(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    if number > 0.0 {
        Ok(number)
    } else {
        Ok(-number)
    }
}