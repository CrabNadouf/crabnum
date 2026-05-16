// code with pyclass

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::functions::*;
use num_bigint::{BigInt, BigUint};
use num_traits::{FromPrimitive, ToPrimitive, Zero};
use pyo3::exceptions::{PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::types::PyTuple;

#[pyclass]
pub struct Crabnum {
    number: f64,
}

#[pymethods]
impl Crabnum {
    #[new]
    pub fn new(number: f64) -> Self {
        Self { number }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.number.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Crabnum({})", self.number))
    }

    #[pyo3(signature = (*args))]
    pub fn sum_of(&self, args: Vec<f64>) -> PyResult<Self> {
        Ok(Self {
            number: self.number + sum_of(args)?,
        })
    }

    #[pyo3(signature = (*args))]
    pub fn dif_of(&self, args: Vec<f64>) -> PyResult<Self> {
        Ok(Self {
            number: self.number - sum_of(args)?,
        })
    }

    pub fn div_of(&self, args: Vec<f64>) -> PyResult<Self> {
        let result: f64 = product(args)?;
        if result == 0.0 {
            return Err(PyValueError::new_err("Can't divide by zero!"));
        }
        Ok(Self {
            number: self.number / result,
        })
    }

    #[pyo3(signature = (*args))]
    pub fn int_div_of(&self, args: Vec<f64>) -> PyResult<Self> {
        let res = product(args)?;
        if res == 0.0 {
            return Err(PyValueError::new_err("Can't divide by zero!"));
        }
        Ok(Self {
            number: (self.number as i64 / res as i64) as f64,
        })
    }

    pub fn rem(&self, a: f64) -> PyResult<Self> {
        Ok(Self {
            number: rem(self.number, a)?,
        })
    }

    pub fn product(&self, args: Vec<f64>) -> PyResult<Self> {
        Ok(Self {
            number: self.number * product(args)?,
        })
    }

    pub fn square(&self) -> PyResult<Self> {
        Ok(Self {
            number: square(self.number)?,
        })
    }

    pub fn cube(&self) -> PyResult<Self> {
        Ok(Self {
            number: cube(self.number)?,
        })
    }

    pub fn power(&self, exp: f64) -> PyResult<Self> {
        Ok(Self {
            number: power(self.number, exp)?,
        })
    }

    pub fn square_root(&self) -> PyResult<Self> {
        Ok(Self {
            number: square_root(self.number)?,
        })
    }

    pub fn cube_root(&self) -> PyResult<Self> {
        Ok(Self {
            number: cube_root(self.number)?,
        })
    }

    pub fn root(&self, power: f64) -> PyResult<Self> {
        Ok(Self {
            number: root(self.number, power)?,
        })
    }

    pub fn factorial(&self) -> PyResult<Self> {
        Ok(Self {
            number: factorial(BigInt::from(self.number.round() as u64))?
                .to_f64()
                .ok_or_else(|| PyValueError::new_err("Result is too large to fit in f64"))?,
        })
    }

    pub fn gcd(&self, args: Vec<BigInt>) -> PyResult<Self> {
        Ok(Self {
            number: gcd_rust(BigInt::from(self.number.round() as i64), gcd(args)?)
                .to_f64()
                .ok_or_else(|| PyValueError::new_err("Result is too large to fit in f64"))?,
        })
    }

    pub fn lcm(&self, args: Vec<f64>) -> PyResult<Self> {
        let first = BigInt::from(self.number as i64);
        let mut arguments = vec![first];
        for i in args {
            let val = BigInt::from(i as i64);
            arguments.push(val);
        }
        Ok(Self {
            number: lcm(arguments)?.to_f64().unwrap_or(f64::NAN),
        })
    }

    pub fn floor(&self) -> PyResult<Self> {
        Ok(Self {
            number: self.number.floor(),
        })
    }

    pub fn ceil(&self) -> PyResult<Self> {
        Ok(Self {
            number: self.number.ceil(),
        })
    }

    pub fn is_positive(&self) -> PyResult<bool> {
        Ok(is_positive(self.number)?)
    }

    pub fn is_negative(&self) -> PyResult<bool> {
        Ok(is_negative(self.number)?)
    }

    pub fn sign(&self) -> PyResult<i8> {
        Ok(sign(self.number)?)
    }

    pub fn is_integer(&self) -> PyResult<bool> {
        Ok(is_integer(self.number)?)
    }

    pub fn is_even(&self) -> PyResult<bool> {
        match is_integer(self.number)? {
            true => Ok(is_even(self.number as i64)?),
            false => Err(PyValueError::new_err("Number must be integer.")),
        }
    }

    pub fn is_odd(&self) -> PyResult<bool> {
        match is_integer(self.number)? {
            true => Ok(is_odd(self.number as i64)?),
            false => Err(PyValueError::new_err("Number must be integer.")),
        }
    }

    pub fn sin(&self) -> PyResult<Self> {
        Ok(Self {
            number: sin(self.number)?,
        })
    }

    pub fn csc(&self) -> PyResult<Self> {
        Ok(Self {
            number: csc(self.number)?,
        })
    }

    pub fn cos(&self) -> PyResult<Self> {
        Ok(Self {
            number: cos(self.number)?,
        })
    }

    pub fn sec(&self) -> PyResult<Self> {
        Ok(Self {
            number: sec(self.number)?,
        })
    }

    pub fn tan(&self) -> PyResult<Self> {
        Ok(Self {
            number: tan(self.number)?,
        })
    }

    pub fn cot(&self) -> PyResult<Self> {
        Ok(Self {
            number: cot(self.number)?,
        })
    }

    pub fn fibonacci(&self) -> PyResult<BigUint> {
        Ok(fibonacci(self.number as usize)?)
    }

    pub fn absolute(&self) -> PyResult<Self> {
        Ok(Self {
            number: absolute(self.number)?,
        })
    }

    pub fn log(&self, base: f64) -> PyResult<Self> {
        Ok(Self {
            number: log(base, self.number)?,
        })
    }
}
