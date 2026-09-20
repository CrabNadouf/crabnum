#![allow(unsafe_op_in_unsafe_fn)]

use crate::functions::*;
use pyo3::prelude::*;

#[pyfunction]
/// Returns the sine of `number` (in radians).
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// # 1
/// print(sin(0)) # it will print 0.0
///
/// # 2
/// print(sin(1.5707963267948966)) # it will print 1.0
/// ```
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
/// # 1
/// print(csc(0)) # it will print inf
/// 
/// # 2
/// print(csc(PI/4)) # it will print 1.4142135623730951
/// ```
pub fn csc(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    Ok(1.0 / number.sin())
}

#[pyfunction]
/// Returns cosine of `number` (in radians).
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// # 1
/// print(cos(0)) # it will print 1.0
/// 
/// # 2
/// print(cos(2.0943951023931953)) # it will print -0.4999999999999998
/// ```
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
/// # 1
/// print(sec(0)) # it will print 1.0
/// 
/// # 2
/// print(sec(5-4)) # it will print 1.8508157176809255
/// 
/// # 3
/// print(sec(0.14*0.66)) # it will print 1.0042841190233904
/// ```
pub fn sec(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    Ok(1.0 / number.cos())
}

#[pyfunction]
/// Returns the tangent of `number` (in radians).
/// ### Arguments
/// `number` - a float number
/// ### Examples
/// ```python
/// # 1
/// print(tan(0)) # it will print 0.0
/// 
/// # 2
/// print(tan(0.7853981633974483)) # it will print 0.9999999999999999
/// ```
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
/// # 1
/// print(cot(0)) # it will print inf
///
/// # 2
/// print(cot(PI/4)) # it will print 1.0000000000000002
/// ```
pub fn cot(number: f64) -> PyResult<f64> {
    check_is_finite(number)?;
    Ok(1.0 / number.tan())
}
