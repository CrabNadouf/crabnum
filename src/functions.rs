#![allow(unsafe_op_in_unsafe_fn)]

use num_bigint::{BigInt, BigUint};
use num_traits::{FromPrimitive, One, Signed, ToPrimitive, Zero};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::mem::replace;

// special functions

// checks if number is finite
pub(crate) fn check_is_finite(number: f64) -> PyResult<f64> {
    if number.is_finite() {
        Ok(number)
    } else {
        Err(PyValueError::new_err("Number must be finite"))
    }
}

pub(crate) fn empty<T>(args: &[T]) -> PyResult<&[T]> {
    if args.is_empty() {
        return Err(PyValueError::new_err(
            "Function takes at least one argument.",
        ));
    } else {
        Ok(args)
    }
}

// python functions

#[pyfunction]
/// Returns the sum of the numbers in the list.
/// ## Arguments
/// `args` - the list with float numbers
/// ## Examples
/// ```python
/// print(sum_of([5, 2)])
/// # It will print 7 (5 + 2 = 7)
/// ```
/// ```python
/// print(sum_of([1001 - 1, 5]))
/// # It will print 1005 (1000 - 1 + 5 = 1005)
/// ```
pub fn sum_of(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    check_is_finite(args.iter().sum())
}

#[pyfunction]
/// Returns the difference between the first number and the sum of the other numbers in the list.
/// ### Arguments
/// `args` - list with float numbers
/// ### Examples
/// ```python
/// print(dif_of([1, 2, 3])) # it will print -4.0 (1 - (2 + 3) = -4)
/// ```
/// ```python
/// print(dif_of([100, 24, 3])) # it will print 73.0 (100 - (24 + 3) = 73)
/// ```
pub fn dif_of(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    check_is_finite(args[0] - args[1..].iter().sum::<f64>())
}

#[pyfunction]
/// Returns the result of consecutive division of the numbers in the list.
/// ### Arguments
/// `args` - a list with float numbers
/// ### Examples
/// ```python
/// # 1
/// print(div_of([100, 25, 5])) # it will print 0.8
/// ```
/// ```python
/// # 2
/// print(div_of([993093, 33434, 4])) # it will print 7.425771669557935
/// ```
pub fn div_of(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    if args[1..].iter().any(|&x| x == 0.0) {
        return Err(PyValueError::new_err("Integer division by zero!"));
    }
    check_is_finite(args[1..].iter().fold(args[0], |acc, &x| acc / x))
}

#[pyfunction]
/// Returns the result of consecutive integer division of the numbers in the list.
/// ### Arguments
/// `args`- the list with integer numbers
/// ### Examples
/// ```python
/// # 1
/// print(int_div_of([993093, 33434, 4])) # it will print 7
/// ```
/// ```python
/// # 2
/// print(int_div_of([10, 3, 1])) # it will print 3
/// ```
pub fn int_div_of(args: Vec<i64>) -> PyResult<i64> {
    empty(&args)?;
    if args[1..].iter().any(|&x| x == 0) {
        return Err(PyValueError::new_err("Integer division by zero!"));
    }
    Ok(args[1..].iter().fold(args[0], |acc, &x| acc / x))
}

#[pyfunction]
/// Returns the remainder of division `a` / `b`.
/// ### Arguments
/// `a` - a float number
/// `b` - a float number
/// ### Examples
/// ```python
/// print(rem(10, 3)) # it will print 1.0
/// ```
/// ```python
/// print(rem(100, 6)) # it will print 4.0
/// ```
pub fn rem(a: f64, b: f64) -> PyResult<f64> {
    check_is_finite(a)?;
    check_is_finite(b)?;
    Ok(a % b)
}

#[pyfunction]
/// Returns the product between all numbers in a list.
/// ### Arguments
/// `args` - list with float numbers
/// ### Examples
/// ```python
/// print(product([10, 10])) # it will print 100.0
/// ```
/// ```python
/// print(product([3, 9, 17])) # it will print 459.0
/// ```
pub fn product(args: Vec<f64>) -> PyResult<f64> {
    empty(&args)?;
    check_is_finite(args.iter().product::<f64>())
}

#[pyfunction]
/// Returns the square of `number`.
/// # Arguments
/// `number` - a float number
pub fn square(number: f64) -> PyResult<f64> {
    check_is_finite(number.powf(2.0))
}

#[pyfunction]
/// Returns the cube of `number`.
/// # Arguments
/// `number` - a float number
pub fn cube(number: f64) -> PyResult<f64> {
    check_is_finite(number.powf(3.0))
}

#[pyfunction]
/// Returns a `number` raised to a `power`.
/// # Arguments
/// `number` - a float number<br>
/// `exp` - a float number
pub fn power(number: f64, exp: f64) -> PyResult<f64> {
    if exp == 0.0 {
        return Ok(1.0);
    }
    check_is_finite(number.powf(exp))
}

#[pyfunction]
/// Returns the square root of `number`.
/// # Arguments
/// `number` - a float number
pub fn square_root(number: f64) -> PyResult<f64> {
    if number < 0.0 {
        return Err(PyValueError::new_err("Number cant be negative."));
    }
    check_is_finite(number.powf(0.5))
}

#[pyfunction]
/// Returns the cube root  of `number`.
/// # Arguments
/// `number` - a float number
pub fn cube_root(number: f64) -> PyResult<f64> {
    check_is_finite(number.powf(1.0 / 3.0))
}

#[pyfunction]
/// Returns the `power`-th root of `number`.
/// # Arguments
/// `number` - a float number<br>
/// `power` - a float number
pub fn root(number: f64, power: f64) -> PyResult<f64> {
    if power < 0.0 {
        return Err(PyValueError::new_err("Power cant be negative."));
    }
    let total_power = 1.0 / power;
    check_is_finite(number.powf(total_power))
}

#[pyfunction]
/// Returns the factorial of `number`.
/// # Arguments
/// `number` - an integer number
pub fn factorial(number: BigInt) -> PyResult<BigInt> {
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
pub fn gcd_rust(a: BigInt, b: BigInt) -> BigInt {
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
pub fn gcd(args: Vec<BigInt>) -> PyResult<BigInt> {
    empty(&args)?;
    Ok(args[1..].iter().fold(args[0].clone(), |acc, next_val| {
        gcd_rust(acc, next_val.clone())
    }))
}

#[pyfunction]
/// Returns the LCM of numbers in the list.
/// # Arguments
/// `args` - the list wih integer numbers
pub fn lcm(args: Vec<BigInt>) -> PyResult<BigInt> {
    // empty(args)?;
    let mut res = args[0].clone();

    if res.is_zero() {
        return Ok(BigInt::zero());
    }

    for i in 1..args.len() {
        let next_val = &args[i];

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
pub fn ceil(number: f64) -> PyResult<i64> {
    check_is_finite(number)?;
    Ok(number.ceil() as i64)
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
/// # Returns <br>
/// `-1` if `number` is negative,<br>
/// `0` if `number` is zero, <br>
/// `1` if `number` is positive.
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
/// `number` - a float number
pub fn is_integer(number: f64) -> PyResult<bool> {
    check_is_finite(number)?;
    Ok(number.fract() == 0.0)
}

#[pyfunction]
/// # Returns
/// `true` if `number` is even, <br>
/// `false` if number isn't even.
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
/// Returs the cosecant of `number` (in radians).
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// print(csc(0)) # it will print inf
/// ```
/// ```python
/// print(csc(number_pi/4)) # it will print 1.4142135623730951
/// ```
pub fn csc(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    Ok(1.0 / number.sin())
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
/// Returns the secant of `number` (in radians).
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// print(sec(0)) # it will print 1.0
/// ```
/// ```python
/// print(sec(5-4)) # it will print 1.8508157176809255
/// ```
/// ```python
/// print(sec(0.14*0.66)) # it will print 1.0042841190233904
/// ```
pub fn sec(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    Ok(1.0 / number.cos())
}

#[pyfunction]
/// Returns the tangent of `number` (in radians).
/// # Arguments
/// `number` - a float number
pub fn tan(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    Ok(number.tan())
}

#[pyfunction]
/// Returns the cotangent of `number` (in radians).
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// print(cot(0)) # it will print inf
/// ```
/// ```python
/// print(cot(number_pi/4)) # it will print 1.0000000000000002
/// ```
pub fn cot(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    Ok(1.0 / number.tan())
}

pub(crate) fn pow_bigint(base: &BigInt, exponent: &BigInt) -> BigInt {
    if exponent.is_zero() {
        return BigInt::one();
    }
    if exponent.is_one() {
        return base.clone();
    }

    if let Some(exp_u32) = exponent.to_u32() {
        return base.pow(exp_u32);
    }

    let mut result = BigInt::one();
    let mut base_power = base.clone();
    let mut exp = exponent.clone();

    while !exp.is_zero() {
        if (&exp & &BigInt::one()).is_one() {
            result *= &base_power;
        }
        base_power = &base_power * &base_power;
        exp >>= 1;
    }
    result
}

#[pyfunction]
/// Returns the list with Fibonacci sequence for a given `range`.
/// ### Arguments
/// `range` - an integer i64 number
/// ### Examples
/// ```python
/// print(fibonacci(10)) # it will print [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
/// ```
/// ```python
/// print(fibonacci(4)) # it will print [0, 1, 1, 2]
/// ```
/// ### Warning!
/// The specified range must not exceed 91!
pub fn fibonacci(n: usize) -> PyResult<BigUint> {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();

    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }

    Ok(f0)
}

#[pyfunction]
/// Returns the absolute value of `number`.
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// print(absolute(-3)) # it will print 3
/// ```
/// ```python
/// print(absolute(3)) # it will print 3.0
/// ```
pub fn absolute(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    if number > 0.0 {
        Ok(number)
    } else {
        Ok(-number)
    }
}

#[pyfunction]
/// Returns the logarithm of`number` to `base`.
/// ### Arguments
/// `base` - a float number<br>
/// `number` - a float number
/// ### Examples
/// ```python
/// print(log(10, 100)) # it will print 2.0
/// ```
/// ```python
/// print(log(2, 8)) # it will print 3.0
/// ```
pub fn log(base: f64, number: f64) -> PyResult<f64> {
    check_is_finite(base)?;
    check_is_finite(number)?;
    Ok(number.log(base))
}

#[pyfunction]
/// Returns the result of tetration of `base` to height `n`.
/// ### Arguments
/// `a` - a positive integer (base)
/// `n` - a non-negative integer (the height)
pub fn tetration(a: i64, n: u32) -> PyResult<BigInt> {
    if a <= 0 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "The base must be positive integer.",
        ));
    }
    let base = BigInt::from_i64(a).unwrap();

    match n {
        0 => Ok(BigInt::one()),
        1 => Ok(base),
        _ => {
            let mut result = base.clone();
            for _ in 1..n {
                result = pow_bigint(&base, &result);
            }
            Ok(result)
        }
    }
}
