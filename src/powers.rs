#![allow(unsafe_op_in_unsafe_fn)]

use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
/// Returns the square of `number`.
/// ### Arguments
/// `number` - a number
/// ### Examples
/// ```python
/// # 1
/// print(square(3)) # it will print 9
/// 
/// # 2
/// print(square(10)) # it will print 100
/// ```
pub fn square(number: Decimal) -> PyResult<Decimal> {
    Ok(number * number)
}

#[pyfunction]
/// Returns the cube of `number`.
/// ### Arguments
/// `number` - a number
/// ### Examples
/// ```python
/// # 1
/// print(cube(2)) # it will print 8
///
/// # 2
/// print(cube(4)) # it will print 64
/// ```
pub fn cube(number: Decimal) -> PyResult<Decimal> {
    Ok(number * number * number)
}

#[pyfunction]
/// Returns a `number` raised to a `power`.
/// ### Arguments
/// `number` - a number<br>
/// `exp` - a number
/// ### Examples
/// ```python
/// # 1
/// print(power(2, 5)) # it will print 32
/// 
/// # 2
/// print(power(7, 3)) # it will print 343
/// ```
pub fn power(number: Decimal, exp: Decimal) -> PyResult<Decimal> {
    if exp == dec!(0) {
        return Ok(dec!(1));
    }
    Ok(number.powd(exp))
}

#[pyfunction]
/// Returns the square root of `number`.
/// ### Arguments
/// `number` - a number
/// ### Examples
/// ```python
/// # 1
/// print(square_root(25)) # it will print 5
///
/// # 2
/// print(square_root(64)) # it will print 8
/// ```
pub fn square_root(number: Decimal) -> PyResult<Decimal> {
    if number < dec!(0.0) {
        return Err(PyValueError::new_err("Number cant be negative."));
    }
    Ok(number.sqrt().unwrap().round_dp(12).normalize())
}

#[pyfunction]
/// Returns the cube root  of `number`.
/// ### Arguments
/// `number` - a number
/// ### Examples
/// ```python
/// print(cube_root(8)) # it will print 2
/// 
/// # 2
/// print(cube_root(1331)) # it will print 11
/// 
/// # 3
/// print(cube_root(343)) # it will print 7
/// ```
pub fn cube_root(number: Decimal) -> PyResult<Decimal> {
    Ok(number.powd(dec!(1)/dec!(3)).round_dp(12).normalize())
}

#[pyfunction]
/// Returns the `power`-th root of `number`.
/// ### Arguments
/// `number` - a number <br>
/// `power` - a number
/// ### Examples
/// ```python
/// # 1
/// print(root(1331, 3)) # it will print 11
/// 
/// # 2
/// print(root(6561, 4)) # it will print 9
/// ```
pub fn root(number: Decimal, power: Decimal) -> PyResult<Decimal> {
    if power <= dec!(0.0) {
        return Err(PyValueError::new_err("Power cant be negative."));
    }
    let total_power = dec!(1.0) / power;
    Ok(number.powd(total_power).round_dp(12).normalize())
}
