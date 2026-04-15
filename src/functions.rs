use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};
use pyo3::exceptions::{PyOverflowError, PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::types::PyTuple;

// special functions

// checks if number is finite
fn check_is_finite(number: f64) -> PyResult<f64> {
    if number.is_finite() {
        Ok(number)
    } else {
        Err(PyValueError::new_err("Number must be finite"))
    }
}

// checks integer overflow of result
fn check_is_integer_overflow(a: i64, b: i64) -> PyResult<()> {
    if a == i64::MIN && b == -1 {
        Err(PyOverflowError::new_err("Integer overrflow: MIN / -1"))
    } else {
        Ok(())
    }
}


fn empty<'py>(args: &'py Bound<'py, PyTuple>) -> PyResult<&'py Bound<'py, PyTuple>> {
    if args.is_empty() { return Err(PyValueError::new_err( "Function takes at least one argument.")) } else { Ok(args) }
}

// python functions

#[pyfunction]
#[pyo3(signature = (*args))]
/// Returns the sum of the numbers in the list.
/// # Arguments
/// `args` - a list with float numbers
pub fn sum_of(args: &Bound<'_, PyTuple>) -> PyResult<f64> {
    empty(args)?;
    let mut result: f64 = 0.0;

    for item in args.iter() {
        let val: f64 = item.extract()?;
        result += val;
    }

    check_is_finite(result)
}


#[pyfunction]
#[pyo3(signature = (*args))]
/// Returns the difference between the first number and the sum of the other numbers in the list.
/// # Arguments
/// `args` - a list with float numbers
pub fn dif_of(args: &Bound<'_, PyTuple>) -> PyResult<f64> {
    empty(args)?;
    let mut result: f64 = args.get_item(0)?.extract()?;

    for i in 1..args.len() {
        let val: f64 = args.get_item(i)?.extract()?;
        result -= val;
    }
    check_is_finite(result)
}


#[pyfunction]
#[pyo3(signature = (*args))]
/// Returns the result of consecutive division of the numbers in the list.
/// # Arguments
/// `args`  - a list with float numbers
pub fn div_of(args: &Bound<'_, PyTuple>) -> PyResult<f64> {
    empty(args)?;
    let mut result: f64 = args.get_item(0)?.extract()?;

    for i in 1..args.len() {
        let val: f64 = args.get_item(i)?.extract()?;
        result /= val;
    }

    check_is_finite(result)
}


#[pyfunction]
#[pyo3(signature = (*args))]
/// Returns the result of consecutive integer division of the numbers in the list.
/// # Arguments
/// `args`- a list with integer numbers
pub fn int_div_of(args: &Bound<'_, PyTuple>) -> PyResult<i64> {
    empty(args)?;
    let mut result: i64 = args.get_item(0)?.extract()?;

    for i in 1..args.len() {
        let val: i64 = args.get_item(i)?.extract()?;
        if val == 0 {
            return Err(PyZeroDivisionError::new_err("Division by zero."));
        }

        check_is_integer_overflow(result, val)?;

        result /= val;
    }
    Ok(result)
}


#[pyfunction]
#[pyo3(signature = (*args))]
/// Returns the product between all numbers in a list.
/// # Arguments
/// `args` - a list with float numbers
pub fn product(args: &Bound<'_, PyTuple>) -> PyResult<f64> {
    empty(args)?;
    let mut result: f64 = args.get_item(0)?.extract()?;

    for i in 1..args.len() {
        let val: f64 = args.get_item(i)?.extract()?;
        result *= val;
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
        return Err(PyValueError::new_err("Number cant be negative."));
    }

    let result = number.powf(0.5);

    check_is_finite(result)
}


#[pyfunction]
/// Returns the cube root  of `number`.
/// # Arguments
/// `number` - a float number
pub fn cube_root(number: f64) -> PyResult<f64> {
    let result = number.powf(1.0 / 3.0);

    check_is_finite(result)
}


#[pyfunction]
/// Returns the `power`-th root of `number`.
/// # Arguments
/// `number` - a float number
/// `power` - a float number
pub fn root(number: f64, power: f64) -> PyResult<f64> {
    if power < 0.0 {
        return Err(PyValueError::new_err("Power cant be negative."));
    }

    let total_power = 1.0 / power;
    let result = number.powf(total_power);

    check_is_finite(result)
}


#[pyfunction]
/// Returns the factorial of `number`.
/// # Arguments
/// `number` - an integer number
pub fn factorial(number: BigInt) -> PyResult<BigInt> {
    // i created the bigint type so that there would be no limitations in calculating the factorial
    if number < BigInt::zero() {
        return Err(PyValueError::new_err("Number cant be negattive."));
    }

    let mut result = BigInt::one();
    let mut current = BigInt::one();

    while current < number {
        current += 1;
        result *= &current;
    }

    Ok(result)
}


// auxiliary function
pub fn gcd_rust(mut a: BigInt, mut b: BigInt) -> BigInt {
    let mut a = a.abs();
    let mut b = b.abs();
    while !b.is_zero() {
        let r = &a % &b;
        a = std::mem::replace(&mut b, r);
    }
    a
}


#[pyfunction]
/// Returns the GCD of `args` in the list.
/// # Arguments
/// `args` - integer numbers
#[pyo3(signature = (*args))]
pub fn gcd(args: &Bound<'_, PyTuple>) -> PyResult<BigInt> {
    empty(args)?;

    let mut res: BigInt = args.get_item(0)?.extract()?;
    for i in 1..args.len() {
        let next_val: BigInt = args.get_item(i)?.extract()?;
        res = gcd_rust(res, next_val);
    }
    Ok(res)
}

#[pyfunction]
/// Returns the LCM of `args`.
/// # Arguments
/// `args` - integer numbers
#[pyo3(signature = (*args))]
pub fn lcm(args: &Bound<'_, PyTuple>) -> PyResult<BigInt> {
    empty(args)?;
    let mut res: BigInt = args.get_item(0)?.extract()?;

    if res.is_zero() {
        return Ok(BigInt::zero());
    }

    for i in 1..args.len() {
        let next_val: BigInt = args.get_item(i)?.extract()?;

        if next_val.is_zero() {
            return Ok(BigInt::zero());
        }

        let g = gcd_rust(res.clone(), next_val.clone());
        res = (res.abs() / g) * next_val.abs();
    }

    Ok(res)
}

#[pyfunction]
/// Returns `number` rounded down.
/// # Arguments
/// `number` - a float number
pub fn floor(mut number: f64) -> PyResult<i64> {
    number = number.floor();
    check_is_finite(number)?;
    Ok(number as i64)
}

#[pyfunction]
/// Returns `number` rounded up.
/// # Arguments
/// `number` - a float number
pub fn ceil(mut number: f64) -> PyResult<i64> {
    check_is_finite(number)?;
    number = number.ceil();
    Ok(number as i64)
}

#[pyfunction]
/// Returns `true` if `number` is positive.
/// # Arguments
/// `number` - a float number
pub fn is_positive(number: f64) -> PyResult<bool> {
    check_is_finite(number)?;
    Ok(number > 0.0)
}

#[pyfunction]
/// Returns `true` if `number` is negative.
/// # Arguments
/// `number` - a float number
pub fn is_negative(number: f64) -> PyResult<bool> {
    check_is_finite(number)?;
    Ok(number < 0.0)
}

#[pyfunction]
/// Returns `-1` if `number` is negative, `0` if `number` is zero and `1` if `number` is positive.
/// # Arguments
/// `number` - a float number
pub fn sign(number: f64) -> PyResult<i8> {
    check_is_finite(number)?;
    if number > 0.0 {
        Ok(1)
    } else if number < 0.0 {
        Ok(-1)
    } else {
        Ok(0)
    }
}

#[pyfunction]
/// Returns `true` if `number` is integer.
/// # Arguments
/// `number` - a floatt number
pub fn is_integer(number: f64) -> PyResult<bool> {
    check_is_finite(number)?;
    Ok(number.fract() == 0)
}


#[pyfunction]
/// Returns `true` if `number` is even.
/// # Arguments
/// `number` - an integer number
pub fn is_even(number: i64) -> PyResult<bool> {
    Ok(number % 2 == 0)
}


#[pyfunction]
/// Returns `true` if number is odd.
/// # Arguments
/// `number` - a float number
pub fn is_odd(number: i64) -> PyResult<bool> {
    Ok(number % 2 != 0)
}


#[pyfunction]
/// Returns the sine of `number` (in radians).
/// # Arguments
/// `number` - a float number
pub fn sin(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    Ok(number.sin())
}


#[pyfunction]
/// Returns cosine of `number` (in radians).
/// # Arguments
/// `number` - a float number
pub fn cos(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    Ok(number.cos())
}


#[pyfunction]
/// Returns the tangent of `number` (in radians).
/// # Arguments
/// `number` - a float number
pub fn tan(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    Ok(number.tan())
}
