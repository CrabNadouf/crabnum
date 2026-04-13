// file with functions 


use pyo3::exceptions::{PyValueError, PyZeroDivisionError, PyOverflowError};	
use pyo3::prelude::*; 
use num_bigint::BigInt; 
use num_traits::{One, Zero};


// special functions

// checks if number is finite
fn check_is_finite(number: f64) -> PyResult<f64>{
    if number.is_finite() { Ok(number) } else { Err(PyValueError::new_err("Number must be finite")) }
}


// checks integer overflow of result
fn check_is_integer_overflow(a: i64, b: i64) -> PyResult<()> {
    if a == i64::MIN && b == -1 { Err(PyOverflowError::new_err("Integer overrflow: MIN / -1")) } else { Ok(()) }
}


// python functions


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
		let mut result = args[0];

		for &i in &args[1..] {
			result -= i
		}

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
				return Err(PyZeroDivisionError::new_err("Divivsion by zero."))
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
			return Err(PyZeroDivisionError::new_err("Division by zero."))
		}
		
		check_is_integer_overflow(result, i)?;

		result /= i;
	}
	Ok(result)
}


#[pyfunction]
/// Returns the product between all numbers in a list.
/// # Arguments
/// `args` - a list with float numbers
pub fn product(args: Vec<f64>) -> PyResult<f64> {
	if args.is_empty() {
		return Err(PyValueError::new_err("mult_of() takes at least 1 value."))
	}

	let mut result = args[0];
	
	for &i in &args[1..] {
		result *= i
	}

	check_is_finite(result)
}


#[pyfunction]
/// Returns the square of `number`.
/// # Arguments
/// `number` - a float number
pub fn square(number: f64) -> PyResult<f64> {
	let result = number.powf(2.0);
	
	check_is_finite(result)
}


#[pyfunction]
/// Returns the cube of `number`.
/// # Arguments
/// `number` - a float number
pub fn cube(number: f64) -> PyResult<f64> {
	let result = number.powf(3.0);
	
	check_is_finite(result)
}


#[pyfunction]
/// Returns a `number` raised to a `power`.
/// # Arguments
/// `number` - a float number
/// `power` - a float number
pub fn power(number: f64, power: f64) -> PyResult<f64> {
	if power == 0.0 {
		return Ok(1.0);
	}

	let result = number.powf(power);
	
	check_is_finite(result)
}


#[pyfunction]
/// Returns the square root of `number`.
/// # Arguments
/// `number` - a float number
pub fn square_root(number: f64) -> PyResult<f64> {
	if number < 0.0 {
		return Err(PyValueError::new_err("Number cant be negative."))
	}
	
	let result = number.powf(0.5);
	
	check_is_finite(result)
} 


#[pyfunction]
/// Returns the cube root  of `number`.
/// # Arguments
/// `number` - a float number
pub fn cube_root(number: f64) -> PyResult<f64> {
	let result = number.powf(1.0/3.0);

	check_is_finite(result)
}


#[pyfunction]
/// Returns the `power`-th root of `number`.
/// # Arguments
/// `number` - a float number
/// `power` - a float number
pub fn root(number: f64, power: f64) -> PyResult<f64> {
	if power < 0.0 {
		return Err(PyValueError::new_err("Power cant be negative."))
	}

	let total_power = 1.0/power;
	let result = number.powf(total_power);

	check_is_finite(result)
}


#[pyfunction]
/// Returns the factorial of `number`.
/// # Arguments
/// `number` - an integer number
pub fn factorial(number: BigInt) -> PyResult<BigInt> {  // i created the bigint type so that there would be no limitations in calculating the factorial
	if number < BigInt::zero() {
		return Err(PyValueError::new_err("Number cant be negattive."))
	}
	
	let mut result = BigInt::one();
	let mut current = BigInt::one();

	while current < number {
		current += 1;
		result *= &current;
	}

	Ok(result)
}











































































































