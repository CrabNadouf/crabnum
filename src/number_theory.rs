#![allow(unsafe_op_in_unsafe_fn)]

use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::mem::replace;

use crate::functions::*;

#[pyfunction]
/// Returns the factorial of `number`.
/// ### Arguments
/// `number` - an integer number
/// ### Examples
/// ```python
/// print(factorial(6)) # it will print 720
/// ```
/// ```python
/// print(factorial(15)) # it will print 1307674368000
/// ```
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
/// ### Arguments
/// `args` - integer numbers
/// ### Examples
/// ```python
/// print(gcd([1, 2, 5])) # it will print 1
/// ```
/// ```python
/// print(gcd([100, 60, 20])) # it will print 20
/// ```
pub fn gcd(args: Vec<BigInt>) -> PyResult<BigInt> {
    empty(&args)?;
    Ok(args[1..].iter().fold(args[0].clone(), |acc, next_val| {
        gcd_rust(acc, next_val.clone())
    }))
}

#[pyfunction]
/// Returns the LCM of numbers in the list.
/// ### Arguments
/// `args` - the list wih integer numbers
/// ### Examples
/// ```python
/// print(lcm([100, 60, 20])) # it will print 300
/// ```
/// ```python
/// print(lcm([50, 33, 27])) # it will print 14850
/// ```
pub fn lcm(args: Vec<BigInt>) -> PyResult<BigInt> {
    empty(&args)?;
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
pub fn fibonacci(n: usize) -> PyResult<Vec<BigUint>> {
    let mut result = vec![];
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();

    for _ in 0..n {
        result.push(f0.clone());

        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }

    Ok(result)
}
