#![allow(unsafe_op_in_unsafe_fn)]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use crate::functions::*;

#[pyfunction]
/// Returns the square of `number`.
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// print(square(3)) # it will print 9.0
/// ```
/// ```python
/// print(square(10)) # it will print 100.0
/// ```
pub fn square(number: f64) -> PyResult<f64> {
    check_is_finite(number.powf(2.0))
}

#[pyfunction]
/// Returns the cube of `number`.
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// print(cube(2)) # it will print 8.0
/// ```
/// ```python
/// print(cube(4)) # it will print 64.0
/// ```
pub fn cube(number: f64) -> PyResult<f64> {
    check_is_finite(number.powf(3.0))
}

#[pyfunction]
/// Returns a `number` raised to a `power`.
/// ### Arguments
/// `number` - a float number<br>
/// `exp` - a float number
/// ### Examples
/// ```python
/// print(power(2, 5)) # it will print 32.0
/// ```
/// ```python
/// print(power(7, 3)) # it will print 343.0
/// ```
pub fn power(number: f64, exp: f64) -> PyResult<f64> {
    if exp == 0.0 {
        return Ok(1.0);
    }
    check_is_finite(number.powf(exp))
}

#[pyfunction]
/// Returns the square root of `number`.
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// print(square_root(25)) # it will print 5.0
/// ```
/// ```python
/// print(square_root(64)) # it will print 8.0
/// ```
pub fn square_root(number: f64) -> PyResult<f64> {
    if number < 0.0 {
        return Err(PyValueError::new_err("Number cant be negative."));
    }
    check_is_finite(number.sqrt())
}

#[pyfunction]
/// Returns the cube root  of `number`.
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// print(cube_root(8)) # it will print 2.0
/// ```
/// ```python
/// print(cube_root(1331)) # it will print 11.0
/// ```
/// ```python
/// print(cube_root(343)) # it will print 7.0
/// ```
pub fn cube_root(number: f64) -> PyResult<f64> {
    check_is_finite(number.cbrt())
}

#[pyfunction]
/// Returns the `power`-th root of `number`.
/// ### Arguments
/// `number` - a float number<br>
/// `power` - a float number
/// ### Examples
/// ```python
/// print(root(1331, 3)) # it will print 11.0
/// ```
/// ```python
/// print(root(6561, 4)) # it will print 9.0
/// ```
pub fn root(number: f64, power: f64) -> PyResult<f64> {
    if power <= 0.0 {
        return Err(PyValueError::new_err("Power cant be negative."));
    }
    let total_power = 1.0 / power;
    check_is_finite((number.powf(total_power) * 1e12).round() / 1e12)
}
