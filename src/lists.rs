#![allow(unsafe_op_in_unsafe_fn)]

use crate::functions::*;
use pyo3::prelude::*;

#[pyfunction]
pub fn mean(args: Vec<f64>) -> PyResult<f64> {
    let len = args.len() as f64;
    empty(&args)?;
    Ok( sum_of(args)? / len  )
}

#[pyfunction]
pub fn biggest(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    let mut biggest = args[0];
    for i in &args {
        if args[*i as usize] > biggest {
            biggest = args[*i as usize];
        }
    }
    Ok( biggest )
}


#[pyfunction]
pub fn biggest_index(args: Vec<f64>) -> PyResult<usize> {
    empty(&args)?;
    let mut biggest_index: usize = 0;
    for i in &args {
        if args[*i as usize] > args[biggest_index] {
            biggest_index = *i as usize;
        }
    }
    Ok( biggest_index )
}

#[pyfunction]
pub fn smallest(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    let mut smallest = args[0];
    for i in &args {
        if args[*i as usize] < smallest {
            smallest = args[*i as usize];
        }
    }
    Ok( smallest )
}

#[pyfunction]
pub fn smallest_index(args: Vec<f64>) -> PyResult<usize> {
    empty(&args)?;
    let mut smallest: usize = 0;
    for i in &args {
        if args[*i as usize] < args[smallest] {
            smallest = *i as usize;
        }
    }
    Ok( smallest )
}

#[pyfunction]
pub fn clear(mut args: Vec<f64>) -> PyResult<Vec<f64>> {
    args.clear();
    Ok( args )
}

#[pyfunction]
pub fn sorted_list(mut args: Vec<f64>) -> PyResult<Vec<f64>> {
    empty(&args)?;
    args.sort_by(|a, b| a.total_cmp(b));
    Ok( args )
}

#[pyfunction]
pub fn reversed_list(mut args:  Vec<f64>) -> PyResult<Vec<f64>> {
    empty(&args)?;
    args.sort_by(|a, b| b.total_cmp(a));
    Ok( args )
}

#[pyfunction]
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
pub fn merge(mut list_1: Vec<f64>, args: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
    empty(&list_1)?;
    empty(&args)?;
    for i in args {
        list_1.extend(i)
    }
    Ok(list_1)
}