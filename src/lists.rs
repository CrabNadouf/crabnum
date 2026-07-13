#![allow(unsafe_op_in_unsafe_fn)]

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::arithmetic::*;
use crate::functions::*;
use pyo3::prelude::*;
use std::collections::HashSet;

#[pyfunction]
/// Returns the arithmetic mean of the `a`.
/// ## Arguments
/// `args` - a list with numbers
/// ## Examples
/// ```python
/// print(mean([1, 2, 3, 4, 5])) # it will print 3.0
/// ```
/// ```python
/// print(mean([3, 6, 9, 12, 15, 55])) # it will print 16.666666666666668
/// ```
/// ```python
/// print(mean([1])) # it will print 1.0
/// ```
pub fn mean(args: Vec<Decimal>) -> PyResult<Decimal> {
    empty(&args)?;
    let len = Decimal::from(args.len());
    let sum = sum_of(args)?;
    Ok(sum / len)
}

#[pyfunction]
/// Returns the maximum value in the list.
/// ## Arguments
/// `args` - a list with numbers
/// ## Examples
/// ```python
/// print(max_value([1, 5, 7, 1, 3])) # it will print 7.0
/// ```
/// ```python
/// print(max_value([0, -2, 4, 1, -7])) # it will print 4.0
/// ```
pub fn max_value(args: Vec<Decimal>) -> PyResult<Decimal> {
    empty(&args)?;
    let mut biggest = args[0];
    for &i in &args {
        if i > biggest {
            biggest = i;
        }
    }
    Ok(biggest)
}

#[pyfunction]
/// Returns the index of maximum value in the list.
/// ## Arguments
/// `args` - a list with numbers
/// ## Examples
/// ```python
/// print(max_index([1, 2, 3, 5, 4])) # it will print 3
/// ```
/// ```python
/// print(max_index([-1, -2, 0, -5, -7])) # it will print 2
/// ```
pub fn max_index(args: Vec<Decimal>) -> PyResult<usize> {
    empty(&args)?;
    let mut biggest_index: usize = 0;
    for (i, &val) in args.iter().enumerate() {
        if val > args[biggest_index] {
            biggest_index = i;
        }
    }
    Ok(biggest_index)
}

#[pyfunction]
/// Returns the minimum value in the list.
/// ## Arguments
/// `args` - a list with numbers
/// ## Examples
/// ```python
/// print(min_value([-1, -2, 0, -5, -7])) # it will print -7.0
/// ```
/// ```python
/// print(min_value([123, 54, 3242, 2, 434, 0, -3, 322])) # it will print -3.0
/// ```
pub fn min_value(args: Vec<Decimal>) -> PyResult<Decimal> {
    empty(&args)?;
    let mut smallest = args[0];
    for &i in &args {
        if i < smallest {
            smallest = i;
        }
    }
    Ok(smallest)
}

#[pyfunction]
/// Returns the index of minimum value in the list.
/// ## Arguments
/// `args` - a list with numbers
/// ## Examples
/// ```python
/// print(min_index([123, 54, -2, 2, 434, 0, -3])) # it will print 6
/// ```
/// ```python
/// print(min_index([0, 1, 2, 3])) # it will print 0
/// ```
pub fn min_index(args: Vec<Decimal>) -> PyResult<usize> {
    empty(&args)?;
    let mut smallest: usize = 0;
    for (i, &val) in args.iter().enumerate() {
        if val < args[smallest] {
            smallest = i;
        }
    }
    Ok(smallest)
}

#[pyfunction]
pub fn sorted_list(mut args: Vec<Decimal>) -> PyResult<Vec<Decimal>> {
    empty(&args)?;
    args.sort();
    Ok(args)
}

#[pyfunction]
pub fn reversed_list(mut args: Vec<Decimal>) -> PyResult<Vec<Decimal>> {
    empty(&args)?;
    args.reverse();
    Ok(args)
}

#[pyfunction]
pub fn count(a: Vec<Decimal>, x: Decimal) -> PyResult<u32> {
    empty(&a)?;
    let mut total = 0;
    for i in &a {
        if *i == x {
            total += 1;
        }
    }
    Ok(total)
}

#[pyfunction]
pub fn merge(mut list_1: Vec<Decimal>, args: Vec<Vec<Decimal>>) -> PyResult<Vec<Decimal>> {
    empty(&list_1)?;
    for i in args {
        list_1.extend(i)
    }
    Ok(list_1)
}

#[pyfunction]
pub fn median(a: Vec<Decimal>) -> PyResult<Decimal> {
    empty(&a)?;
    let sorted = sorted_list(a)?;
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Ok((sorted[mid - 1] + sorted[mid]) / dec!(2))
    } else {
        Ok(sorted[mid])
    }
}

#[pyfunction]
pub fn unique(args: Vec<Decimal>) -> PyResult<Vec<Decimal>> {
    empty(&args)?;
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for val in args {
        if seen.insert(val) {
            result.push(val);
        }
    }

    Ok(result)
}

#[pyfunction]
pub fn get_range(args: Vec<Decimal>) -> PyResult<Decimal> {
    empty(&args)?;
    Ok(max_value(args.clone())? - min_value(args)?)
}
