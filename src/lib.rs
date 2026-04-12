// main library file 

// checks if number is not finite
fn check_isnt_finite(number: f64) -> PyResult<f64>{
	if !number.is_finite() { Err(PyValueError::new_err("Number must be finite")) } else { Ok(number) }
}


// checks integer overflow of result
fn check_is_integer_overflow(a: i64, b: i64) -> PyResult<()> {
	if a == i64::MIN && b == -1 { Err(PyOverflowError::new_err("Integer overrflow: MIN / -1")) } else { Ok(()) }
}


use pyo3::exceptions::{PyValueError, PyZeroDivisionError, PyOverflowError};
use pyo3::prelude::*;


#[pyfunction]
/// Returns the sum of the numbers in the list.
/// # Arguments
/// `args` - a list with float numbers
pub fn sum_of(args: Vec<f64>) -> PyResult<f64> { 
    let result: f64 = args.iter().sum();
    
   	check_isnt_finite(result)
}


#[pyfunction]
/// Returns the difference between the first number and the sum of the other numbers in the list.
/// # Arguments
/// `args` - a list  with float numbers
pub fn dif_of(args: Vec<f64>) -> PyResult<f64> {
    match args.as_slice() {
        [] => Ok(0.0),
        [first, rest @..] => {
            let result = *first - rest.iter().sum::<f64>();
            
            check_isnt_finite(result)
        }
    }
}


#[pyfunction]
/// Returns the result of consecutive division of the numbers in the list.
/// # Arguments
/// `args` - a list with float numbers
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
            
            check_isnt_finite(result)
        }
    }
}


#[pyfunction]
/// Returns the result of consecutive integer division of the numbers in the list.
/// # Arguments
/// `args` - a Vector with integer numbers
pub fn int_div_of(args: Vec<i64>) -> PyResult<i64> {
    match args.as_slice() {
        [] => Ok(0),
        [first, rest@..] => {
            let mut result = *first;

            for &i in rest {
                if i == 0 {
                    return Err(PyZeroDivisionError::new_err("Division by zero!"));
                }

                check_is_integer_overflow(result, i)?;
                result /= i;
            }

            Ok(result)
        }
    }
}
                

#[pyfunction]
/// Returns product of all numbers in the list.
/// # Arguments
/// `args` - a Vector with float numbers
pub fn mult_of(args: Vec<f64>) -> PyResult<f64> {
    match args.as_slice() {
        [] => Ok(0.0),
        [first, rest @..] => {
            let mut result = *first;
            
            for &i in rest {
                result *= i
            }
            
            check_isnt_finite(result)
        }
    }
}


#[pyfunction]
/// Returns the square of `number`.
/// # Arguments
/// `number` - a float number
pub fn square(number: f64) -> PyResult<f64> {
    let result = number * number;
    
    check_isnt_finite(result)
}


#[pyfunction]
/// Returns the cube of `number`.
/// # Arguments
/// `number` - a float number
pub fn cube(number: f64) -> PyResult<f64> {
    let result = number.powi(3);
    
    check_isnt_finite(result)
}


#[pyfunction]
/// Returns `number` to the power of `power`.
/// # Arguments
/// `number` - a float number
/// `exponent` - a float number
pub fn power(number: f64, exponent: f64) -> PyResult<f64> {
    let result = number.powf(exponent);
    
    check_isnt_finite(result)
}


#[pyfunction]
/// Returns the square root of `number`.
/// # Arguments
/// `number` - a float number
fn square_root(number: f64) -> PyResult<f64> {
    match number < 0.0 {
        true => Err(PyValueError::new_err("Cannot calculate square root of a negative number")),
        false => {
            let result = number.powf(0.5);
            
            check_isnt_finite(result)
        }
    }
}


#[pyfunction]
/// Returns the cube root of `number`.
/// # Arguments
/// `number` - a float number
fn cube_root(number: f64) -> PyResult<f64> {
    let result = number.powf(1.0/3.0);
    
    check_isnt_finite(result)
}


#[pyfunction]
/// Returns the `power`-th root of `number`.
/// # Arguments
/// `number` - a float number
/// `power` - a float number
fn root(number: f64, power: f64) -> PyResult<f64> {
	if power == 0.0 {
		return Err(PyValueError::new_err("Power cannot be zero."))
	 } else {
	 	if number < 0.0 && power % 2.0 == 0.0 {
	 		return Err(PyValueError::new_err("Cannot calculate even root of a negative number"))
	 	} else {
	 		let result = number.powf(1.0/power);
	 		
	 		check_isnt_finite(result)
		 }
	}
 }


#[pyfunction]
/// Returns the factorial of `number`.
/// # Arguments
/// `number` - integer number
fn factorial(number: i128) -> PyResult<i128> {
    match number < 0 {
        true => Err(PyValueError::new_err("The factorial of a negative number is not defined.")),
        false => {
            let mut result: i128 = 1;
            
            for i in 2..=number {
                result = match result.checked_mul(i) {
                    Some(val) => val,
                    None => return Err(PyOverflowError::new_err("Factorial overflow")),
                };
             }
            Ok(result)
         }
    }
}

 
#[pyfunction]
/// Returns the greatest common divisor of the integer numbers in the list.
/// # Arguments
/// `args` - a list of integers
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
/// Returns the greatest common divisor of the float numbers in the list.
/// # Arguments 
/// `args` - a list of floats 
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
///  Returns the least common multiple (LCM) of the numbers in the list.
/// # Arguments
/// `args` - a list of numbers
fn lcm_with(args: Vec<f64>) -> PyResult<f64> {
    if args.is_empty() {
        return Err(PyValueError::new_err("lcm_with() requires at least one argument."))
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

    check_isnt_finite(result)
}


#[pyfunction]
/// Rounds `number` down.
/// # Arguments
/// `number` - a float number
fn floor(number: f64) -> PyResult<i64> {
    if !number.is_finite() {
        return Err(PyValueError::new_err("Number must be finite."))
    }

    let rounded = number.floor();

    if rounded > i64::MAX as f64 || rounded < i64::MIN as f64 {
        return Err(PyOverflowError::new_err("Result exceeds i64 range"))
    }
    
    Ok(rounded as i64)
}


#[pyfunction]
/// Rounds `number` up.
/// # Arguments
/// `number` - a float number
fn ceil(number: f64) -> PyResult<i64> {
    if !number.is_finite() {
        return Err(PyValueError::new_err("Number must be finite."))
    }

    let rounded = number.ceil();

    if rounded > i64::MAX as f64 || rounded < i64::MIN as f64 {
        return Err(PyOverflowError::new_err("Result exceeds i64 range"))
    }
    
    Ok(rounded as i64)
}


#[pyfunction]
/// Returns `true` if `number` is a positive.
/// # Arguments
/// `number` - a float number
fn is_positive(number: f64) -> PyResult<bool>    {
    check_isnt_finite(number)?;
    Ok(number > 0.0)
}


#[pyfunction]
/// Returns `true` if `number` is a negative.
/// # Arguments
/// `number` - a float number
fn is_negative(number: f64) -> PyResult<bool> {
    if !number.is_finite() {
        return Err(PyValueError::new_err("Number must be finite."))
    }
    
    Ok(number < 0.0)
}


#[pyfunction]
/// Returns `true` if `number` is an integer.
/// # Arguments
/// `number` - a float number
fn is_integer(number: f64) -> PyResult<bool> {
    check_isnt_finite(number)?;
    Ok(number.fract() == 0.0)
}


#[pyfunction]
/// # Returns the sign of `number`:
/// `1`  if `number` > 0
/// `-1` if `number` < 0
/// `0` if number = 0
/// # Arguments
/// `number` - a float number
fn sign(number: f64) -> PyResult<i16> {
    check_isnt_finite(number)?;
    
    if number > 0.0 {
        Ok(1)
    } else if number < 0.0 {
        Ok(-1)
    } else {
        Ok(0)
    }
}


#[pyfunction]
/// Returns `true` if `number` is even.
/// # Arguments
/// `number` - a integer number
fn is_even(number: i64) ->PyResult<bool> {
   Ok(number % 2 == 0)
}


#[pyfunction]
/// Returns `true` if `number` is odd.
/// # Arguments
/// `number`- a integer number
fn is_odd(number: i64) -> PyResult<bool> {
   Ok(number % 2 != 0)
}


#[pyfunction]
/// Returns the sine of `number` (in radians)
/// # Arguments
/// `number` - a float number
fn sin(number: f64) -> PyResult<f64> {
   if !number.is_finite() {
	return Err(PyValueError::new_err("Number must be finite."));
   }

   Ok(number.sin())
}


#[pyfunction]
/// Returns the cosine of `number` (in radians)
/// # Arguments
/// `number` - a float number
fn cos(number: f64) -> PyResult<f64> {
   check_isnt_finite(number)?;
   Ok(number.cos())
}

	
#[pyfunction]
/// Returns the tangent of `number` (in radians)
/// # Arguments
/// `number` - a float number
fn tan(number: f64) -> PyResult<f64> {
   check_isnt_finite(number)?;
   Ok(number.tan())
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
            check_isnt_finite(result)
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
    m.add_function(wrap_pyfunction!(root, m)?)?;
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
 
