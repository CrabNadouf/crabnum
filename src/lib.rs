use pyo3::exceptions::{PyValueError, PyZeroDivisionError, PyOverflowError};
use pyo3::prelude::*;


#[pyfunction]
/// Returns the sum of the numbers in the list.
/// # Arguments
/// `args` - Vector with float numbers
pub fn sum_of(args: Vec<f64>) -> PyResult<f64> { 
    let result: f64 = args.iter().sum();
    match !result.is_finite() {
        true => Err(PyOverflowError::new_err("Result is infinite or NaN")),
        false => Ok(result)
    }
}


#[pyfunction]
/// Returns the difference between the first number and the sum of the other numbers in the list.
/// # Arguments
/// `args` - Vector with float numbers
pub fn dif_of(args: Vec<f64>) -> PyResult<f64> {
    match args.as_slice() {
        [] => Ok(0.0),
        [first, rest @..] => {
            let result = *first - rest.iter().sum::<f64>();
            match !result.is_finite() { 
                true => Err(PyOverflowError::new_err("Result is infinite or NaN")),
                false => Ok(result)
            }
        },
    }
}


#[pyfunction]
/// Returns the result of consecutive division of the numbers in the list.
/// # Arguments
/// `args` - Vector with float numbers
pub fn div_of(args: Vec<f64>) -> PyResult<f64> {
    match args.as_slice() {
        [] => Ok(0.0),
        [first, rest@..] => {
            let mut result = *first;
            for &i in rest {
                if i == 0.0 {
                    return Err(PyZeroDivisionError::new_err("Division by zero!"));
                }
            result /= i;
            }
            match !result.is_finite() {
                true => Err(PyOverflowError::new_err("Result is infinite or NaN")),
                false => Ok(result)
            }
        }
    }
}


#[pyfunction]
/// Returns the result of consecutive integer division of the numbers in the list.
/// # Arguments
/// `args` - Vector with integer numbers
pub fn int_div_of(args: Vec<i64>) -> PyResult<i64> {
    match args.as_slice() {
        [] => Ok(0),
        [first, rest@..] => {
            let mut result = *first;
            for &i in rest {
                if i == 0 {
                    return Err(PyZeroDivisionError::new_err("Division by zero!"))
                }
                match result == i64::MIN && i == -1 {
                    true => return Err(PyOverflowError::new_err("Integer overflow: MIN / -1")),
                    false => {
                        result /= i
                    }
                }          
            }  

            Ok(result)
        }
    }
}


#[pyfunction]
/// Returns product of all numbers in the list.
/// # Arguments
/// `args` - Vector with float numbers
pub fn mult_of(args: Vec<f64>) -> PyResult<f64> {
    match args.as_slice() {
        [] => Ok(0.0),
        [first, rest @..] => {
            let mut result = *first;
            for &i in rest {
                result *= i
            }
            match !result.is_finite() {
                true => Err(PyOverflowError::new_err("Result is infinite or NaN")),
                false => Ok(result)
            }
        }
    }
}


#[pyfunction]
/// Returns the square of `number`.
/// # Arguments
/// `number` - float number
pub fn square(number: f64) -> PyResult<f64> {
    let result = number * number;
    match !result.is_finite() {
        true => Err(PyOverflowError::new_err("Result is infinite or NaN")),
        false => Ok(result)
    }
}


#[pyfunction]
/// Returns the cube of `number`.
/// # Arguments
/// `number` - float number
pub fn cube(number: f64) -> PyResult<f64> {
    let result = number.powi(3);
    match !result.is_finite() {
        true => Err(PyOverflowError::new_err("Result is infinite or NaN")),
        false => Ok(result)
    }
}


#[pyfunction]
/// Returns `number` to the power of `power`.
/// # Arguments
/// `number` - float number
/// `exponent` - float number
pub fn power(number: f64, exponent: f64) -> PyResult<f64> {
    let result = number.powf(exponent);
    match !result.is_finite() {
        true => Err(PyOverflowError::new_err("Result is infinite or NaN")),
        false => Ok(result)
    }
}


#[pyfunction]
/// Returns the square root of `number`.
/// # Arguments
/// `number` - float number
fn square_root(number: f64) -> PyResult<f64> {
    match number < 0.0 {
        true => Err(PyValueError::new_err("Cannot calculate square root of a negative number")),
        false => {
            let result = number.powf(0.5);
            match !result.is_finite() {
                true => Err(PyOverflowError::new_err("Result is infinite of Nan")),
                false => Ok(result)
            }
        }
    }
}


#[pyfunction]
/// Returns the cube root of `number`.
/// # Arguments
/// `number` - float number
fn cube_root(number: f64) -> PyResult<f64> {
    let result = number.powf(1.0/3.0);
    match !result.is_finite() {
        true => Err(PyOverflowError::new_err("Result is infinite of Nan")),
        false => Ok(result)
    }
}


#[pyfunction]
/// Returns the factorial of `number`.
/// # Arguments
/// `number` - integer number
fn factorial(number: i64) -> PyResult<i64> {
    match number < 0 {
        true => Err(PyValueError::new_err("The factorial of a negative number is not defined.")),
        false => {
            let mut result: i64 = 1;
            for i in 2..=number {
                result = match result.checked_mul(i) {
                    Some(val) => val,
                    None => return Err(PyValueError::new_err("Factorial overflow")),
                };
             }
            Ok(result)
         }
    }
}

 
#[pyfunction]
fn gcd_with_int(args: Vec<i64>) -> PyResult<i64> {
    if args.is_empty() {
        return Err(PyValueError::new_err(
            "gcd_with_int() requires at least one argument",
        ));
    }

    let mut result = args[0].abs();

    for &num in &args[1..] {
        let mut a = result;
        let mut b = num.abs();

        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        result = a;
    }

    Ok(result)
}

#[pyfunction]
fn gcd_with_float(args: Vec<f64>) -> PyResult<i64> {
    if args.is_empty() {
        return Err(PyValueError::new_err(
            "gcd_with_float() requires at least one argument",
        ));
    }

    let ints: Vec<i64> = args.iter().map(|&x| x.round() as i64).collect();
    gcd_with_int(ints)
}

#[pyfunction]
fn lcm_with(args: Vec<f64>) -> PyResult<f64> {
    if args.is_empty() {
        return Ok(0.0);
    }

    let mut result = args[0].abs();

    for &num in &args[1..] {
        let num_abs = num.abs();

        let gcd = match gcd_with_float(vec![result, num_abs]) {
            Ok(val) => val,
            Err(_) => return Err(PyValueError::new_err("Error calculating GCD")),
        };

        if gcd == 0 {
            return Ok(0.0);
        }

        result = (result * num_abs) / gcd as f64;
    }

    Ok(result)
}

#[pyfunction]
fn floor(number: f64) -> i64 {
    number.floor() as i64
}

#[pyfunction]
fn ceil(number: f64) -> i64 {
    number.ceil() as i64
}

#[pyfunction]
fn is_positive(number: f64) -> bool {
    if number > 0.0 { true } else { false }
}

#[pyfunction]
fn is_negative(number: f64) -> bool {
    if number < 0.0 { true } else { false }
}

#[pyfunction]
fn is_integer(number: f64) -> bool {
    if number.fract() == 0.0 { true } else { false }
}

#[pyfunction]
fn sign(number: f64) -> i32 {
    if number > 0.0 {
        1
    } else if number < 0.0 {
        -1
    } else {
        0
    }
}

#[pyfunction]
fn is_even(number: i64) -> bool {
    if number % 2 == 0 { true } else { false }
}

#[pyfunction]
fn is_odd(number: i64) -> bool {
    if number % 2 != 0 { true } else { false }
}

fn taylor_series(x: f64, terms: usize, is_sin: bool) -> f64 {
    if terms == 0 {
        return 0.0;
    }

    let mut result = if is_sin { x } else { 1.0 };
    let mut term = result;
    let x_sq = x * x;

    for i in 1..terms {
        let n = if is_sin { 2 * i + 1 } else { 2 * i };
        term *= -x_sq / (n * (n - 1)) as f64;
        result += term;

        if term.abs() < 1e-16 {
            break;
        }
    }
    result
}

fn normalize_angle(x: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let tau = 2.0 * pi;
    x - tau * ((x + pi) / tau).floor()
}

#[pyfunction]
#[pyo3(signature = (number, terms=20))]
fn sin(number: f64, terms: usize) -> PyResult<f64> {
    let x = normalize_angle(number);
    Ok(taylor_series(x, terms, true))
}

#[pyfunction]
#[pyo3(signature = (number, terms=20))]
fn cos(number: f64, terms: usize) -> PyResult<f64> {
    let x = normalize_angle(number);
    Ok(taylor_series(x, terms, false))
}

#[pyfunction]
#[pyo3(signature = (number, terms=20))]
fn tan(number: f64, terms: usize) -> PyResult<f64> {
    let x = normalize_angle(number);
    let s = taylor_series(x, terms, true);
    let c = taylor_series(x, terms, false);

    if c.abs() < 1e-12 {
        return Err(PyZeroDivisionError::new_err(
            "Tangent is undefined: cosine is zero",
        ));
    }

    Ok(s / c)
}

#[pyfunction]
fn tetration(base: f64, height: i64) -> PyResult<f64> {
    match height {
        0 => Ok(1.0),
        1 => Ok(base),
        n if n < 0 => Err(PyValueError::new_err("Height can't be negative")),
        n => {
            let mut result = base;
            for _ in 2..=n {
                result = base.powf(result);

                if result.is_infinite() {
                    return Err(PyValueError::new_err("The result is too big (infinity)"));
                }
            }
            Ok(result)
        }
    }
}
// classes

#[pyclass]
#[derive(Clone)]
struct Nadoufmath {
    number: f64,
}

#[pymethods]
impl Nadoufmath {
    #[new]
    fn new(number: f64) -> Self {
        Nadoufmath { number }
    }

    fn get_number(&self) -> f64 {
        self.number
    }

    fn sum_of(&mut self, args: Vec<f64>) -> PyResult<Self> {
        let sum: f64 = args.iter().sum();
        self.number += sum;
        Ok(self.clone())
    }

    fn dif_of(&mut self, args: Vec<f64>) -> PyResult<Self> {
        if args.is_empty() {
            Ok(self.clone())
        } else {
            let mut result = args[0];
            for num in &args[1..] {
                result -= num;
            }
            self.number -= result;
            Ok(self.clone())
        }
    }

    fn div_of(&mut self, args: Vec<f64>) -> PyResult<Self> {
        if args.is_empty() {
            Ok(self.clone())
        } else {
            let mut result = self.number;
            for &num in &args {
                if num == 0.0 {
                    return Err(PyZeroDivisionError::new_err("Can't divide by zero!"));
                } else {
                    result /= num;
                }
            }
            self.number = result;

            Ok(self.clone())
        }
    }

    fn int_div_of(&mut self, args: Vec<i64>) -> PyResult<Self> {
        if args.is_empty() {
            Ok(self.clone())
        } else {
            let mut result = self.number as i64;
            for &num in &args {
                if num == 0 {
                    return Err(PyZeroDivisionError::new_err("Can't divide by zero!"));
                } else {
                    result /= num;
                }
            }
            self.number = result as f64;
            Ok(self.clone())
        }
    }

    fn mult_of(&mut self, args: Vec<f64>) -> PyResult<Self> {
        if args.is_empty() {
            Ok(self.clone())
        } else {
            let mut result = self.number;
            for &num in &args {
                result *= num;
            }
            self.number = result;
            Ok(self.clone())
        }
    }

    fn square(&mut self) -> PyResult<Self> {
        self.number *= self.number;
        Ok(self.clone())
    }

    fn cube(&mut self) -> PyResult<Self> {
        self.number = self.number.powi(3);
        Ok(self.clone())
    }

    fn power(&mut self, exponent: f64) -> PyResult<Self> {
        self.number = self.number.powf(exponent);
        Ok(self.clone())
    }

    fn power_of_2(&mut self) -> PyResult<Self> {
        self.number = 2.0_f64.powf(self.number);
        Ok(self.clone())
    }

    fn square_root(&mut self) -> PyResult<Self> {
        if self.number < 0.0 {
            return Err(PyValueError::new_err(
                "Cannot calculate square root of a negative number",
            ));
        }
        self.number = self.number.sqrt();
        Ok(self.clone())
    }

    fn cube_root(&mut self) -> PyResult<Self> {
        self.number = self.number.cbrt();
        Ok(self.clone())
    }

    fn factorial(&mut self) -> PyResult<Self> {
        let n = self.number as i128;
        if n < 0 {
            return Err(PyValueError::new_err(
                "The factorial of a negative number is not defined.",
            ));
        }

        let result: i128 = if n == 0 {
            1
        } else {
            let mut fact: i128 = 1;
            for i in 2..=n {
                fact = match fact.checked_mul(i) {
                    Some(val) => val,
                    None => return Err(PyValueError::new_err("Factorial overflow.")),
                };
            }
            fact
        };
        self.number = result as f64;
        Ok(self.clone())
    }

    fn gcd_with_int(&mut self, args: Vec<i64>) -> PyResult<Self> {
        if args.is_empty() {
            return Err(PyValueError::new_err(
                "gcd_with_int() requires at least one argument",
            ));
        }
        let mut result = args[0].abs();

        for &num in &args[1..] {
            let mut a = result;
            let mut b = num.abs();

            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            result = a;
        }

        self.number = result as f64;
        Ok(self.clone())
    }

    fn gcd_with_float(&mut self, args: Vec<f64>) -> PyResult<i64> {
        if args.is_empty() {
            return Err(PyValueError::new_err(
                "gcd_with_float() requires at least one argument",
            ));
        }

        let ints: Vec<i64> = args.iter().map(|&x| x.round() as i64).collect();
        self.gcd_with_int(ints)?;

        Ok(self.number as i64)
    }

    fn lcm_with(&mut self, args: Vec<f64>) -> PyResult<Self> {
        if args.is_empty() {
            self.number = 0.0;
            return Ok(self.clone());
        }

        let mut result = args[0].abs();

        for &num in &args[1..] {
            let num_abs = num.abs();
            let mut temp = self.clone();
            let gcd = temp.gcd_with_float(vec![result, num_abs])?;

            let gcd = gcd as f64;

            if gcd == 0.0 {
                self.number = 0.0;
                return Ok(self.clone());
            }

            result = (result * num_abs) / gcd;
        }
        self.number = result;
        Ok(self.clone())
    }

    fn floor(&mut self) -> PyResult<Self> {
        self.number = self.number.floor();
        Ok(self.clone())
    }

    fn ceil(&mut self) -> PyResult<Self> {
        self.number = self.number.ceil();
        Ok(self.clone())
    }

    fn sign(&mut self) -> PyResult<Self> {
        if self.number > 0.0 {
            self.number = 1.0;
            Ok(self.clone())
        } else if self.number < 0.0 {
            self.number = -1.0;
            Ok(self.clone())
        } else {
            self.number = 0.0;
            Ok(self.clone())
        }
    }

    fn is_positive(&self) -> PyResult<bool> {
        if self.number > 0.0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn is_negative(&self) -> PyResult<bool> {
        if self.number < 0.0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn is_even(&self) -> PyResult<bool> {
        if self.number % 2.0 == 0.0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn is_odd(&self) -> PyResult<bool> {
        if self.number % 2.0 != 0.0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[pyo3(signature = (terms=20))]
    fn cos(&mut self, terms: usize) -> PyResult<f64> {
        let pi = std::f64::consts::PI;
        let x = self.number - 2.0 * pi * ((self.number + pi) / (2.0 * pi)).floor();

        let mut result = 1.0;
        let mut term = 1.0;
        let x_sq = x * x;

        for i in 1..terms {
            let n = 2 * i;
            term *= -x_sq / (n * (n - 1)) as f64;
            result += term;
            if term.abs() < 1e-16 {
                break;
            }
        }

        self.number = result;
        Ok(result)
    }

    #[pyo3(signature = (terms=20))]
    fn sin(&mut self, terms: usize) -> PyResult<f64> {
        let pi = std::f64::consts::PI;
        let x = self.number - 2.0 * pi * ((self.number + pi) / (2.0 * pi)).floor();

        let mut result = x;
        let mut term = x;
        let x_sq = x * x;

        for i in 1..terms {
            let n = 2 * i + 1;
            term *= -x_sq / (n * (n - 1)) as f64;
            result += term;
            if term.abs() < 1e-16 {
                break;
            }
        }

        self.number = result;
        Ok(result)
    }

    #[pyo3(signature = (terms=20))]
    fn tan(&mut self, terms: usize) -> PyResult<f64> {
        let original_number = self.number;

        let s = self.sin(terms)?;
        self.number = original_number;
        let c = self.cos(terms)?;

        if c.abs() < 1e-12 {
            return Err(PyZeroDivisionError::new_err(
                "Tangent undefined (cos close to 0)",
            ));
        }

        let result = s / c;
        self.number = result;
        Ok(result)
    }

    fn tetration(&mut self, height: i64) -> PyResult<Self> {
        self.number = tetration(self.number, height)?;
        Ok(self.clone())
    }

    fn __repr__(&self) -> String {
        format!("Nadoufmath({})", self.number)
    }

    fn __str__(&self) -> String {
        format!("{}", self.number)
    }
}

#[pymodule]
fn nadouf_math(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //class
    m.add_class::<Nadoufmath>()?;

    // functions
    m.add_function(wrap_pyfunction!(sum_of, m)?)?;
    m.add_function(wrap_pyfunction!(dif_of, m)?)?;
    m.add_function(wrap_pyfunction!(div_of, m)?)?;
    m.add_function(wrap_pyfunction!(int_div_of, m)?)?;
    m.add_function(wrap_pyfunction!(mult_of, m)?)?;
    m.add_function(wrap_pyfunction!(square, m)?)?;
    m.add_function(wrap_pyfunction!(cube, m)?)?;
    m.add_function(wrap_pyfunction!(power, m)?)?;
    m.add_function(wrap_pyfunction!(square_root, m)?)?;
    m.add_function(wrap_pyfunction!(cube_root, m)?)?;
    m.add_function(wrap_pyfunction!(factorial, m)?)?;
    m.add_function(wrap_pyfunction!(gcd_with_int, m)?)?;
    m.add_function(wrap_pyfunction!(gcd_with_float, m)?)?;
    m.add_function(wrap_pyfunction!(lcm_with, m)?)?;
    m.add_function(wrap_pyfunction!(floor, m)?)?;
    m.add_function(wrap_pyfunction!(ceil, m)?)?;
    m.add_function(wrap_pyfunction!(is_positive, m)?)?;
    m.add_function(wrap_pyfunction!(is_negative, m)?)?;
    m.add_function(wrap_pyfunction!(is_integer, m)?)?;
    m.add_function(wrap_pyfunction!(sign, m)?)?;
    m.add_function(wrap_pyfunction!(is_even, m)?)?;
    m.add_function(wrap_pyfunction!(is_odd, m)?)?;
    m.add_function(wrap_pyfunction!(cos, m)?)?;
    m.add_function(wrap_pyfunction!(sin, m)?)?;
    m.add_function(wrap_pyfunction!(tan, m)?)?;
    m.add_function(wrap_pyfunction!(tetration, m)?)?;

    // constants
    m.add("number_pi", std::f64::consts::PI)?;
    m.add("number_e", std::f64::consts::E)?;
    m.add("infinity", f64::INFINITY)?;

    Ok(())
}
