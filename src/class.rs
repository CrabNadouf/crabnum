// code with pyclass

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::advanced::*;
use crate::arithmetic::*;
use crate::checks::*;
use crate::functions::*;
use crate::number_theory::*;
use crate::powers::*;
use crate::rounding::*;
use crate::trigonometry::*;


use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use num_bigint::{BigInt, BigUint};
use num_traits::{FromPrimitive, ToPrimitive, Zero};
use pyo3::exceptions::{PyValueError, PyZeroDivisionError};
use pyo3::prelude::*;
use pyo3::types::PyTuple;

#[pyclass]
pub struct Crabnum {
    number: Decimal,
}

#[pymethods]
impl Crabnum {
    #[new]
    pub fn new(number: Decimal) -> Self {
        Self { number }
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.number.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Crabnum({})", self.number))
    }

    pub fn sum_of(&self, args: Vec<Decimal>) -> PyResult<Self> {
        Ok(Self {
            number: self.number + sum_of(args)?,
        })
    }

    pub fn dif_of(&self, args: Vec<Decimal>) -> PyResult<Self> {
        Ok(Self {
            number: self.number - sum_of(args)?,
        })
    }

    pub fn div_of(&self, args: Vec<Decimal>) -> PyResult<Self> {
        let result = product(args)?;
        if result == dec!(0.0) {
            return Err(PyValueError::new_err("Can't divide by zero!"));
        }
        Ok(Self {
            number: self.number / result,
        })
    }

    pub fn rem(&self, b: Decimal) -> PyResult<Self> {
        Ok(Self {
            number: rem(self.number, b)?,
        })
    }

    pub fn product(&self, args: Vec<Decimal>) -> PyResult<Self> {
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

    pub fn power(&self, exp: Decimal) -> PyResult<Self> {
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

    pub fn root(&self, power: Decimal) -> PyResult<Self> {
        Ok(Self {
            number: root(self.number, power)?,
        })
    }

    pub fn factorial(&self) -> PyResult<BigInt> {
        let n = self
            .number
            .round()
            .to_u64()
            .ok_or_else(|| PyValueError::new_err("Number must be a non-negative integer"))?;
        
        factorial(BigInt::from(n))
    }

    pub fn gcd(&self, args: Vec<Decimal>) -> PyResult<Self> {
        if self.number.fract() != dec!(0) {
            return Err(PyValueError::new_err("Number must be an integer for GCD"));
        }
        
        let first = self
            .number
            .to_i64()
            .ok_or_else(|| PyValueError::new_err("Number is too large for GCD"))?;
        
        let mut arguments = vec![BigInt::from(first)];
        
        for arg in args {
            if arg.fract() != Decimal::ZERO {
                return Err(PyValueError::new_err("All arguments must be integers for GCD"));
            }
            let int_arg = arg
                .to_i64()
                .ok_or_else(|| PyValueError::new_err("Argument is too large for GCD"))?;
            arguments.push(BigInt::from(int_arg));
        }
        
        
        let decimal_result = gcd(arguments)?
            .to_i64()
            .map(Decimal::from)
            .ok_or_else(|| PyValueError::new_err("GCD result is too large for Decimal"))?;
        
        Ok(Self { number: decimal_result })
    }

    pub fn lcm(&self, args: Vec<Decimal>) -> PyResult<Self> {
        if self.number.fract() != Decimal::ZERO {
            return Err(PyValueError::new_err("Number must be an integer for LCM"));
        }
        
        let first = self
            .number
            .to_i64()
            .ok_or_else(|| PyValueError::new_err("Number is too large for LCM"))?;
        
        let mut arguments = vec![BigInt::from(first)];
        
        for arg in args {
            if arg.fract() != Decimal::ZERO {
                return Err(PyValueError::new_err("All arguments must be integers for LCM"));
            }
            let int_arg = arg
                .to_i64()
                .ok_or_else(|| PyValueError::new_err("Argument is too large for LCM"))?;
            arguments.push(BigInt::from(int_arg));
        }
        
        let decimal_result = lcm(arguments)?
            .to_i64()
            .map(Decimal::from)
            .ok_or_else(|| PyValueError::new_err("LCM result is too large for Decimal"))?;
        
        Ok(Self { number: decimal_result })
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
        if self.number.fract() != Decimal::ZERO {
            return Err(PyValueError::new_err("Number must be an integer"));
        }
        
        let n = self
            .number
            .to_i64()
            .ok_or_else(|| PyValueError::new_err("Number is too large for even check"))?;
        
        Ok(n % 2 == 0)  
    }

    pub fn is_odd(&self) -> PyResult<bool> {
        if self.number.fract() != Decimal::ZERO {
            return Err(PyValueError::new_err("Number must be an integer"));
        }
        
        let n = self
            .number
            .to_i64()
            .ok_or_else(|| PyValueError::new_err("Number is too large for odd check"))?;
        
        Ok(n % 2 != 0)
    }

    pub fn sin(&self) -> PyResult<Self> {
        let num = self
            .number
            .to_f64()
            .ok_or_else(|| PyValueError::new_err("Number is too large for trigonometric function"))?;
        
        Ok(Self { number: Decimal::from_f64(num.sin())
            .ok_or_else(|| PyValueError::new_err("Result is out of Decimal range"))? })
    }

    pub fn csc(&self) -> PyResult<Self> {
        let num = self
            .number
            .to_f64()
            .ok_or_else(|| PyValueError::new_err("Number is too large for trigonometric function"))?;
        
        Ok(Self { number: Decimal::from_f64(csc(num)?)
            .ok_or_else(|| PyValueError::new_err("Result is out of Decimal range"))? })
    }

    pub fn cos(&self) -> PyResult<Self> {
        let num = self
            .number
            .to_f64()
            .ok_or_else(|| PyValueError::new_err("Number is too large for trigonometric function"))?;
        
        Ok(Self { number: Decimal::from_f64(num.cos())
            .ok_or_else(|| PyValueError::new_err("Result is out of Decimal range"))? })
    }

    pub fn sec(&self) -> PyResult<Self> {
        let num = self
            .number
            .to_f64()
            .ok_or_else(|| PyValueError::new_err("Number is too large for trigonometric function"))?;
        
        Ok(Self { number: Decimal::from_f64(sec(num)?)
            .ok_or_else(|| PyValueError::new_err("Result is out of Decimal range"))? })
    }

    pub fn tan(&self) -> PyResult<Self> {
        let num = self
            .number
            .to_f64()
            .ok_or_else(|| PyValueError::new_err("Number is too large for trigonometric function"))?;
        
        Ok(Self { number: Decimal::from_f64(num.tan())
            .ok_or_else(|| PyValueError::new_err("Result is out of Decimal range"))? })
    }

    pub fn cot(&self) -> PyResult<Self> {
        let num = self
            .number
            .to_f64()
            .ok_or_else(|| PyValueError::new_err("Number is too large for trigonometric function"))?;
        
        Ok(Self { number: Decimal::from_f64(cot(num)?)
            .ok_or_else(|| PyValueError::new_err("Result is out of Decimal range"))? })
    }

    pub fn fibonacci(&self) -> PyResult<Vec<BigUint>> {
        if self.number.fract() != Decimal::ZERO {
            return Err(PyValueError::new_err("Number must be an integer"));
        }
    
        if self.number < Decimal::ZERO {
            return Err(PyValueError::new_err("Number cannot be negative"));
        }

        let n = self.number.to_usize().ok_or_else(|| PyValueError::new_err("Number is too large for fibonacci"))?;
    
        Ok(fibonacci(n)?)
    }

    pub fn absolute(&self) -> PyResult<Self> {
        Ok(Self {
            number: absolute(self.number)?,
        })
    }

    pub fn log(&self, base: Decimal) -> PyResult<Self> {
        Ok(Self {
            number: log(self.number, base)?,
        })
    }
}
