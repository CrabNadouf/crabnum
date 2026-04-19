// code with pyclass

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::functions::*;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::exceptions::{PyZeroDivisionError, PyValueError};
use num_bigint::BigInt;
use num_traits::ToPrimitive;

#[pyclass]
struct Crabnum {
    number: f64,
}

#[pymethods]
impl Crabnum {
    #[new]
    pub fn new(number: f64) -> Self {
        Self { number }
    }

    #[pyo3(signature = (*args))]
    pub fn sum_of(&self, args: &Bound<'_, PyTuple>) -> PyResult<Self> {
        let result: f64 = sum_of(args)?;
        Ok(Self { number: self.number + result })
    }

    #[pyo3(signature = (*args))]
    pub fn dif_of(&self, args: &Bound<'_, PyTuple>) -> PyResult<Self> {
        let result: f64 = sum_of(args)?;
        Ok(Self { number: self.number - result})
    }

    #[pyo3(signature = (*args))]
    pub fn div_of(&self, args: &Bound<'_, PyTuple>) -> PyResult<Self> {
        let result: f64 = product(args)?;
        if result == 0.0 {
            return Err(PyZeroDivisionError::new_err("Can't divide by zero!"))
        }
        Ok(Self { number: self.number / result})
    }

    #[pyo3(signature = (*args))]
    pub fn int_div_of(&self, args: &Bound<'_, PyTuple>) -> PyResult<Self> {
        let result_1: f64 = product(args)?;
        let selfnumber = self.number as i64;
        let number = result_1 as i64;
        Ok(Self { number: (selfnumber / number) as f64})
    }

    #[pyo3(signature = (*args))]
    pub fn product(&self, args: &Bound<'_,  PyTuple>) -> PyResult<Self> {
        Ok(Self { number: self.number * product(args)?})
    }

    pub fn square(&self) -> PyResult<Self> {
        Ok(Self { number: square(self.number)?})
    }

    pub fn cube(&self) -> PyResult<Self> {
        Ok(Self { number: cube(self.number)?})
    }

    pub fn power(&self, exp: f64) -> PyResult<Self> {
        Ok(Self { number: power(self.number, exp)?})
    }

    pub fn square_root(&self) -> PyResult<Self> {
        Ok(Self { number: square_root(self.number)?})
    }

    pub fn cube_root(&self) -> PyResult<Self> {
        Ok(Self { number: cube_root(self.number)?})
    }

    pub fn root(&self, power: f64) -> PyResult<Self> {
        Ok(Self { number: root(self.number, power)?})
    }

    pub fn factorial(&self) -> PyResult<Self> {
        let n = self.number.round() as u64;
        let number = BigInt::from(n);
        let res_bigint = factorial(number)?;
        let result = res_bigint.to_f64().ok_or_else(|| {PyValueError::new_err("Result is too large to fit in f64")})?;
        Ok(Self { number: result })
    }
}