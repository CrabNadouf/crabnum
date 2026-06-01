#![allow(unsafe_op_in_unsafe_fn)]

use crate::functions::*;
use pyo3::prelude::*;
use std::collections::HashSet;

#[pyfunction]
/// Returns the arithmetic mean of the `a`.
/// ## Arguments
/// `a` - a list with float numbers
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
pub fn mean(a: Vec<f64>) -> PyResult<f64> {
    let len = a.len() as f64;
    empty(&a)?;
    Ok(sum_of(a)? / len)
}

#[pyfunction]
/// Returns the maximum value in the list.
/// ## Arguments
/// `a` - a list with float numbers
/// ## Examples
/// ```python
/// print(max_value([1, 5, 7, 1, 3])) # it will print 7.0
/// ```
/// ```python
/// print(max_value([0, -2, 4, 1, -7])) # it will print 4.0
/// ```
pub fn max_value(a: Vec<f64>) -> PyResult<f64> {
    empty(&a)?;
    let mut biggest = a[0];
    for &i in &a {
        if i > biggest {
            biggest = i;
        }
    }
    Ok(biggest)
}

#[pyfunction]
/// Returns the index of maximum value in the list.
/// ## Arguments
/// `a` - a list with float numbers
/// ## Examples
/// ```python
/// print(max_index([1, 2, 3, 5, 4])) # it will print 3
/// ```
/// ```python
/// print(max_index([-1, -2, 0, -5, -7])) # it will print 2
/// ```
pub fn max_index(a: Vec<f64>) -> PyResult<usize> {
    empty(&a)?;
    let mut biggest_index: usize = 0;
    for (i, &val) in a.iter().enumerate() {
        if val > a[biggest_index] {
            biggest_index = i;
        }
    }
    Ok(biggest_index)
}

#[pyfunction]
/// Returns the minimum value in the list.
/// ## Arguments
/// `a` - a list with float numbers
/// ## Examples
/// ```python
/// print(min_value([-1, -2, 0, -5, -7])) # it will print -7.0
/// ```
/// ```python
/// print(min_value([123, 54, 3242, 2, 434, 0, -3, 322])) # it will print -3.0
/// ```
pub fn min_value(a: Vec<f64>) -> PyResult<f64> {
    empty(&a)?;
    let mut smallest = a[0];
    for &i in &a {
        if i < smallest {
            smallest = i;
        }
    }
    Ok(smallest)
}

#[pyfunction]
/// Returns the index of minimum value in the list.
/// ## Arguments
/// `a` - a list with float numbers
/// ## Examples
/// ```python
/// print(min_index([123, 54, -2, 2, 434, 0, -3])) # it will print 6
/// ```
/// ```python
/// print(min_index([0, 1, 2, 3])) # it will print 0
/// ```
pub fn min_index(a: Vec<f64>) -> PyResult<usize> {
    empty(&a)?;
    let mut smallest: usize = 0;
    for (i, &val) in a.iter().enumerate() {
        if val < a[smallest] {
            smallest = i;
        }
    }
    Ok(smallest)
}

#[pyfunction]
pub fn sorted_list(mut a: Vec<f64>) -> PyResult<Vec<f64>> {
    empty(&a)?;
    a.sort_by(|a, b| a.total_cmp(b));
    Ok(a)
}

#[pyfunction]
pub fn reversed_list(mut a: Vec<f64>) -> PyResult<Vec<f64>> {
    empty(&a)?;
    a.sort_by(|a, b| b.total_cmp(a));
    Ok(a)
}

#[pyfunction]
pub fn count(a: Vec<f64>, x: f64) -> PyResult<i64> {
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
pub fn merge(mut list_1: Vec<f64>, args: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
    empty(&list_1)?;
    for i in args {
        list_1.extend(i)
    }
    Ok(list_1)
}

#[pyfunction]
pub fn median(a: Vec<f64>) -> PyResult<f64> {
    empty(&a)?;
    let sorted = sorted_list(a)?;
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Ok((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Ok(sorted[mid])
    }
}

#[pyfunction]
pub fn unique(a: Vec<f64>) -> PyResult<Vec<f64>> {
    empty(&a)?;
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for val in a {
        if seen.insert(val.to_bits()) {
            result.push(val);
        }
    }

    Ok(result)
}

#[pyfunction]
pub fn get_range(a: Vec<f64>) -> PyResult<f64> {
    empty(&a)?;
    Ok(max_value(a.clone())? - min_value(a)?)
}
