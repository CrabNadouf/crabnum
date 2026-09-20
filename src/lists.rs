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
/// print(mean([1, 2, 3, 4, 5])) # it wil print 3.0
/// 
/// # 2
/// print(mean([1])) # it will print 1.0
/// 
/// # 3
/// print(mean([123, 564, 291, 57])) # it will print 258.75
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
/// # 1
/// print(min_index([123, 54, -2, 2, 434, 0, -3])) # it will print 6
/// 
/// # 2
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
/// Returns the input list of floats sorted in ascending order.
/// ### Arguments
/// `args` - a list with numbers
/// ### Examples
/// ```python
/// # 1
/// print(sorted_list([1, 3, 5, 7])) # it will print [1.0, 3.0, 5.0, 7.0]
/// 
/// # 2
/// print(sorted_list([])) 
/// 
/// # it will print: 
/// Traceback (most recent call last):
/// File "test.py", line 4, in <module>
///     print(sorted_list([]))
///           ~~~~~~~~~~~^^^^
/// ValueError: Function takes at least one argument.
/// ```
pub fn sorted_list(mut args: Vec<f64>) -> PyResult<Vec<f64>> {
    empty(&args)?;
    args.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(args)
}

#[pyfunction]
/// Returns a list reversed in order.
/// ### Arguments
/// `args` - a list with numbers
/// ### Examples
/// ```python
/// # 1
/// print(reversed_list([0])) # it will print [0.0]
/// 
/// # 2
/// print(reversed_list([1, 2, 3, 4, 5])) # it will print [5.0, 4.0, 3.0, 2.0, 1.0]
/// ```
pub fn reversed_list(mut args: Vec<f64>) -> PyResult<Vec<f64>> {
    args.reverse();
    Ok(args)
}

#[pyfunction]
/// Returns the number of `x` values ​​in the `args` list
/// ### Arguments
/// `args` - a list with numbers <br>
/// `x` - a number
/// ### Examples
/// ```python
/// # 1
/// print(count([1, 6, 5, 2, 1, 2, 5], 5)) # it will print 2
/// 
/// # 2
/// print(count([1, 6, 11, 0], 7)) # it will print 0
/// ```
pub fn count(args: Vec<f64>, x: f64) -> PyResult<i64> {
    empty(&args)?;
    let mut total = 0;
    for i in &args {
        if *i == x {
            total += 1;
        }
    }
    Ok(total)
}

#[pyfunction]
/// Returns the concatenation of all input lists, sorted in ascending order.
/// ### Arguments
/// `list` - a list with numbers <br>
/// `args` - a list with lists of numbers
/// ### Examples
/// ```python
/// # 1
/// print(merge([1], [])) # it will print [1.0]
/// 
/// # 2
/// print(merge([12, 1], [[3, 7, 0, 1, 4, 0], [1, 0]])) 
/// # it will print [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 3.0, 4.0, 7.0, 12.0]
/// ```
pub fn merge(mut list: Vec<f64>, args: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
    empty(&list)?;
    for i in args {
        list.extend(i)
    }
    sorted_list(list)
}

#[pyfunction]
/// Returns the median of the list
/// ### Arguments
/// `args` - a list with numbers
/// ### Examples
/// ```python
/// # 1
/// print(median([1, 2, 3, 4, 5])) # it will print 3.0
/// 
/// # 2
/// print(median([3, 6, 1, 0, 7, 2, 11])) # it will print 3.0
/// ```
pub fn median(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    let mid = (args.len() as f64 / 2.0) as usize;
    if args.len() as f64 % 2.0 == 0.0 {
        Ok((args[mid - 1] + args[mid]) / 2.0)
    } else {
        Ok(args[mid])
    }
}

#[pyfunction]
/// Returns a list without duplicate elements.
/// ### Arguments
/// `args` - a list with numbers
/// ### Examples
/// ```python
/// # 1
/// print(unique([1, 1, 2, 3, 4, 4, 5])) # it will print [1.0, 2.0, 3.0, 4.0, 5.0] 
/// 
/// # 2
/// print(unique([54, 2, 0, 11, 2])) # it will print [54.0, 2.0, 0.0, 11.0]
/// ```
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
/// Returns the range of the list (max value - min value)
/// ### Arguments
/// `args` - a list with numbers
/// ### Examples
/// ```python
/// # 1
/// print(get_range([1, 2, 3])) # it will print 2.0
/// 
/// # 2
/// print(get_range([x*2 for x in range(0, 5)])) # it will print 8.0
/// ```
pub fn get_range(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    Ok(max_value(args.clone())? - min_value(args)?)
}
