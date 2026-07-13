#![allow(unsafe_op_in_unsafe_fn)]

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use crate::functions::*;

#[pyfunction]
/// Returns the sum of the numbers in the list.
/// ## Arguments
/// `args` - the list with float numbers
/// ## Examples
/// ```python
/// print(sum_of([5, 2)])
/// # It will print 7 (5 + 2 = 7)
/// ```
/// ```python
/// print(sum_of([1001 - 1, 5]))
/// # It will print 1005 (1000 - 1 + 5 = 1005)
/// ```
pub fn sum_of(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    check_is_finite(args.iter().sum())
}

#[pyfunction]
/// Returns the difference between the first number and the sum of the other numbers in the list.
/// ### Arguments
/// `args` - list with float numbers
/// ### Examples
/// ```python
/// print(dif_of([1, 2, 3])) # it will print -4.0 (1 - (2 + 3) = -4)
/// ```
/// ```python
/// print(dif_of([100, 24, 3])) # it will print 73.0 (100 - (24 + 3) = 73)
/// ```
pub fn dif_of(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    check_is_finite(args[0] - args[1..].iter().sum::<f64>())
}

#[pyfunction]
/// Returns the result of consecutive division of the numbers in the list.
/// ### Arguments
/// `args` - a list with float numbers
/// ### Examples
/// ```python
/// # 1
/// print(div_of([100, 25, 5])) # it will print 0.8
/// ```
/// ```python
/// # 2
/// print(div_of([993093, 33434, 4])) # it will print 7.425771669557935
/// ```
pub fn div_of(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    if args[1..].iter().any(|&x| x == 0.0) {
        return Err(PyValueError::new_err("Division by zero!"));
    }
    check_is_finite(args[1..].iter().fold(args[0], |acc, &x| acc / x))
}

#[pyfunction]
/// Returns the result of consecutive integer division of the numbers in the list.
/// ### Arguments
/// `args`- the list with integer numbers
/// ### Examples
/// ```python
/// # 1
/// print(int_div_of([993093, 33434, 4])) # it will print 7
/// ```
/// ```python
/// # 2
/// print(int_div_of([10, 3, 1])) # it will print 3
/// ```
pub fn int_div_of(args: Vec<i64>) -> PyResult<i64> {
    empty(&args)?;
    if args[1..].iter().any(|&x| x == 0) {
        return Err(PyValueError::new_err("Integer division by zero!"));
    }
    Ok(args[1..].iter().fold(args[0], |acc, &x| acc / x))
}

#[pyfunction]
/// Returns the remainder of division `a` / `b`.
/// ### Arguments
/// `a` - a float number
/// `b` - a float number
/// ### Examples
/// ```python
/// print(rem(10, 3)) # it will print 1.0
/// ```
/// ```python
/// print(rem(100, 6)) # it will print 4.0
/// ```
pub fn rem(a: f64, b: f64) -> PyResult<f64> {
    check_is_finite(a)?;
    check_is_finite(b)?;
    Ok(a % b)
}

#[pyfunction]
/// Returns the product between all numbers in a list.
/// ### Arguments
/// `args` - list with float numbers
/// ### Examples
/// ```python
/// print(product([10, 10])) # it will print 100.0
/// ```
/// ```python
/// print(product([3, 9, 17])) # it will print 459.0
/// ```
pub fn product(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    check_is_finite(args.iter().product::<f64>())
}