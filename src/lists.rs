#![allow(unsafe_op_in_unsafe_fn)]

use crate::functions::*;
use pyo3::prelude::*;

#[pyfunction]
pub fn mean(a: Vec<f64>) -> PyResult<f64> {
    let len = a.len() as f64;
    empty(&a)?;
    Ok( sum_of(a)? / len  )
}

#[pyfunction]
pub fn biggest(a: Vec<f64>) -> PyResult<f64> {
    empty(&a)?;
    let mut biggest = a[0];
    for i in &a {
        if a[*i as usize] > biggest {
            biggest = a[*i as usize];
        }
    }
    Ok( biggest )
}


#[pyfunction]
pub fn biggest_index(a: Vec<f64>) -> PyResult<usize> {
    empty(&a)?;
    let mut biggest_index: usize = 0;
    for i in &a {
        if a[*i as usize] > a[biggest_index] {
            biggest_index = *i as usize;
        }
    }
    Ok( biggest_index )
}

#[pyfunction]
pub fn smallest(a: Vec<f64>) -> PyResult<f64> {
    empty(&a)?;
    let mut smallest = a[0];
    for i in &a {
        if a[*i as usize] < smallest {
            smallest = a[*i as usize];
        }
    }
    Ok( smallest )
}

#[pyfunction]
pub fn smallest_index(a: Vec<f64>) -> PyResult<usize> {
    empty(&a)?;
    let mut smallest: usize = 0;
    for i in &a {
        if a[*i as usize] < a[smallest] {
            smallest = *i as usize;
        }
    }
    Ok( smallest )
}

#[pyfunction]
pub fn clear(mut a: Vec<f64>) -> PyResult<Vec<f64>> {
    a.clear();
    Ok( a )
}

#[pyfunction]
pub fn sorted_list(mut a: Vec<f64>) -> PyResult<Vec<f64>> {
    empty(&a)?;
    a.sort_by(|a, b| a.total_cmp(b));
    Ok( a )
}

#[pyfunction]
pub fn reversed_list(mut a:  Vec<f64>) -> PyResult<Vec<f64>> {
    empty(&a)?;
    a.sort_by(|a, b| b.total_cmp(a));
    Ok( a )
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
    empty(&args)?;
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
    if mid % 2 == 0 {
        Ok((sorted[mid-1] + sorted[mid]) / 2.0)
    } else {
        Ok(sorted[mid])
    }
}