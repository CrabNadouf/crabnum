// file with functions 

// special functions

// checks if number is finite
fn check_is_finite(number: f64) -> PyResult<f64>{
    if number.is_finite() { Ok(number) } else { Err(PyValueError::new_err("Number must be finite")) }
}


// checks integer overflow of result
fn check_is_integer_overflow(a: i64, b: i64) -> PyResult<()> {
    if a == i64::MIN && b == -1 { Err(PyOverflowError::new_err("Integer overrflow: MIN / -1")) } else { Ok(()) }
}


use pyo3::exceptions::{PyValueError, PyZeroDivisionError, PyOverflowError};	// import errors
use pyo3::prelude::*; // import crate



#[pyfunction]
/// Returns the sum of the numbers in the list.
/// # Arguments
/// `args` - a list with float numbers
pub fn sum_of(args: Vec<f64>) -> PyResult<f64> {
    if args.is_empty() {
		return Err(PyValueError::new_err("sum_of() takes at least 1 value."))
	} else {
	let result: f64 = args.iter().sum();

    check_is_finite(result)
	}
}


#[pyfunction]
/// Returns the difference between the first number and the sum of the other numbers in the list.
/// # Arguments
/// `args` - a list with float numbers
pub fn dif_of(args: Vec<f64>) -> PyResult<f64> {
	if args.is_empty() {
		return Err(PyValueError::new_err("dif_of() takes at least 1 value."))
	} else {
		let mut first = args[0];
		let sum_of_rest = args[1..].iter().sum();
		let result = first - sum_of_rest;

		check_is_finite(result)
	}
}


#[pyfunction]
/// Returns the result of consecutive division of the numbers in the list.
/// # Arguments
/// `args`  - a list with float numbers
pub fn div_of(args: Vec<f64>) -> PyResult<f64> {
	if args.is_empty() {
		return Err(PyValueError::new_err("div_of() takes at least 1 value"))
	} else {
		let mut result = args[0];
		for &i in &args[1..]{
			if i == 0.0 {
				return Err(PyZeroDivisionError::new_err("Divivsion by zero.")
			} 
			
			result /=i;
		}
	check_is_finite(result)
	}
}


#[pyfunction]
/// Returns the result of consecutive integer division of the numbers in the list.
/// # Arguments
/// `args`- a list with integer numbers
pub fn int_div_of(args: Vec<i64>) -> PyResult<i64>{
	if args.is_empty() {
		return Err(PyValueError::new_err("int_div_of() takes at least 1 value"))
	}
	
	let mut result = args[0];
	for &i in &args[1..] {
		if i == 0 {
			return Err(PyZeroDivisionError::new_err("Division by zero.")
		}
		
		result /= i;
	}
	
	check_is_finite(result)
}





































