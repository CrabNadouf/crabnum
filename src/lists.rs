#![allow(unsafe_op_in_unsafe_fn)]

use crate::functions::*;
use pyo3::prelude::*;
use std::collections::HashSet;

#[pyfunction]
/// Returns the arithmetic mean of the `a`.
/// ### Arguments
/// `args` - a list with numbers
/// ### Examples
/// ```python
/// # 1
/// 
/// 
/// # 2
/// 
/// 
/// # 3
///
/// ```
pub fn mean(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    let len = args.len() as f64;
    let sum1: f64 = args.iter().sum();
    Ok(sum1 / len)
}

#[pyfunction]
/// Returns the maximum value in the list.
/// ### Arguments
/// `args` - a list with numbers
/// ### Examples
/// ```python
/// # 1
/// print(max_value([1, 5, 7, 1, 3])) # it will print 7
/// 
/// # 2
/// print(max_value([0, -2, 4, 1, -7])) # it will print 4
/// ```
pub fn max_value(args: Vec<f64>) -> PyResult<f64> {
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
/// ### Arguments
/// `args` - a list with numbers
/// ### Examples
/// ```python
/// # 1
/// print(max_index([1, 2, 3, 5, 4])) # it will print 3
/// 
/// # 2 
/// print(max_index([-1, -2, 0, -5, -7])) # it will print 2
/// ```
pub fn max_index(args: Vec<f64>) -> PyResult<usize> {
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
/// ### Arguments
/// `args` - a list with numbers
/// ### Examples
/// ```python
/// # 1
/// print(min_value([-1, -2, 0, -5, -7])) # it will print -7
/// 
/// # 2
/// print(min_value([123, 54, 3242, 2, 434, 0, -3, 322])) # it will print -3
/// ```
pub fn min_value(args: Vec<f64>) -> PyResult<f64> {
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
/// ### Arguments
/// `args` - a list with numbers
/// ### Examples
/// ```python
/// print(min_index([123, 54, -2, 2, 434, 0, -3])) # it will print 6
/// ```
/// ```python
/// print(min_index([0, 1, 2, 3])) # it will print 0
/// ```
pub fn min_index(args: Vec<f64>) -> PyResult<usize> {
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
pub fn sorted_list(mut args: Vec<f64>) -> PyResult<Vec<f64>> {
    empty(&args)?;
    args.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(args)
}

#[pyfunction]
pub fn reversed_list(mut args: Vec<f64>) -> PyResult<Vec<f64>> {
    empty(&args)?;
    args.reverse();
    Ok(args)
}

#[pyfunction]
pub fn count(args: Vec<f64>, x: f64) -> PyResult<f64> {
    empty(&args)?;
    let mut total = 0.0;
    for i in &args {
        if *i == x {
            total += 1.0;
        }
    }
    Ok(total)
}

#[pyfunction]
pub fn merge(mut list_1: Vec<f64>, args: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
    empty(&list_1)?;
    for i in args {
        list_1.extend(i)
    }
    Ok(list_1)
}

#[pyfunction]
pub fn median(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    let sorted = sorted_list(args)?;
    let mid = (sorted.len() as f64 / 2.0) as usize;
    if sorted.len() as f64 % 2.0 == 0.0 {
        Ok((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Ok(sorted[mid])
    }
}

#[pyfunction]
pub fn unique(args: Vec<f64>) -> PyResult<Vec<f64>> {
    empty(&args)?;
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for val in args {
        if seen.insert(val.to_bits()) {
            result.push(val);
        }
    }

    Ok(result)
}

#[pyfunction]
pub fn get_range(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    Ok(max_value(args.clone())? - min_value(args)?)
}
