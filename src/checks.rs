#![allow(unsafe_op_in_unsafe_fn)]

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use pyo3::prelude::*;

#[pyfunction]
/// Returns `true` if `number` is positive.
/// ### Arguments
/// `number` - a number
/// ### Examples
/// ```python
/// # 1
/// print(is_positive(-6)) # it will print False
/// 
/// # 2
/// print(is_positive(0)) # it will print False
/// 
/// # 3
/// print(is_positive(11)) # it will print True
/// ```
pub fn is_positive(number: Decimal) -> PyResult<bool> {
    Ok(number > dec!(0))
}

#[pyfunction]
/// Returns `true` if `number` is negative.
/// ### Arguments
/// `number` - a number
/// ### Examples
/// ```python
/// # 1
/// print(is_negative(99)) # it will print False
/// 
/// # 2
/// print(is_negative(-1798)) # it will print True
/// 
/// # 3
/// print(is_negative(0)) # it will print False
/// ```
pub fn is_negative(number: Decimal) -> PyResult<bool> {
    Ok(number < dec!(0))
}

#[pyfunction]
/// ### Returns
/// `-1` if `number` is negative,
/// 
/// `0` if `number` is zero, 
/// 
/// `1` if `number` is positive.
/// ### Arguments
/// `number` - a number
/// ### Examples
/// ```python
/// # 1
/// print(sign(0)) # it will print 0
/// 
/// # 2
/// print(sign(543)) # it will print 1
/// 
/// # 3
/// print(sign(-14)) # it will print -1
/// ```
pub fn sign(number: Decimal) -> PyResult<i8> {
    if number > dec!(0) {
        Ok(1)
    } else if number < dec!(0) {
        Ok(-1)
    } else {
        Ok(0)
    }
}

#[pyfunction]
/// Returns `true` if `number` is integer.
/// ### Arguments
/// `number` - a number
/// ### Examples
/// ```python
/// # 1
/// print(is_integer(8)) # it will print True
/// 
/// # 2
/// print(is_integer(6.5)) # it will print False
/// ```
pub fn is_integer(number: Decimal) -> PyResult<bool> {
    Ok(number.fract() == dec!(0.0))
}

#[pyfunction]
/// ### Returns
/// `true` if `number` is even, <br>
/// `false` if number isn't even.
/// ### Arguments
/// `number` - an integer number
/// ### Examples
/// ```python
/// # 1
/// print(is_even(22)) # it will print True
/// 
/// # 2
/// print(is_even(21)) # it will print False
/// ```
pub fn is_even(number: i64) -> PyResult<bool> {
    Ok(number % 2 == 0)
}

#[pyfunction]
/// Returns `true` if number is odd.
/// ### Arguments
/// `number` - an integer number
/// ### Examples
/// ```python
/// # 1
/// print(is_odd(7892)) # it will print False
/// 
/// # 2
/// print(is_odd(-73)) # it will print True
/// ```
pub fn is_odd(number: i64) -> PyResult<bool> {
    Ok(number % 2 != 0)
}
