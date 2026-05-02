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
