# √Nadouf's math library 

## Navigation 🗺

 - [➊ Description 🗒](#description-)
 - [➋ Features ⚡](#features-)
 - [❸ Installing 🗁](#installing-)
 - [➍ Quick start 🖈](#quick-start-)
 - [❺ Documentation 🗎](#documentation-)


## Description 🗒

> ⚡ **Blazing fast** • 🛡️ **Memory safe** • 🎯 **Precise computation**

The Nadouf math library is written entirely in **Rust** and wrapped for Python, combining:
- **Rust's performance** — compiled to native code, no interpreter overhead
- **Python's simplicity** — intuitive API, easy integration
- **Type safety** — catch errors at compile time, not runtime

Perfect for scientific computing, data analysis, and educational purposes! ✨

## Features ⚡

✅ **Basic arithmetic** — sum, difference, product, division  
✅ **Powers & roots** — square, cube, power, square root, cube root  
✅ **Number theory** — factorial (up to 33!), GCD, LCM  
✅ **Trigonometry** — sin, cos, tan (via Taylor series)  
✅ **Advanced math** — tetration, sign functions  
✅ **Object-oriented API** — stateful calculations with method chaining  
✅ **Zero-cost abstractions** — Rust's efficiency without compromise  



## Installing 🗁

### From PyPi (recommend) 🚀

```bash
pip install nadouf-math
```

### From source (for contributors) 🛠️

```bash
git clone https://github.com/CrabNadouf/Nadouf_Math_Library.git
cd Nadouf_Math_Library
pip install maturin
maturin develop
```

## Quick start 🖈

```python
from nadouf_math import Nadoufmath, sum_of, dif_of, factorial

# STYLE 1

calc = Nadoufmath(5)
print(calc.sum_of([5]).dif_of([6]).factorial()) # 24

# or

print(Nadoufmath(5).sum_of([5]).dif_of([6]).factorial()) # 24

# STYLE 2

calc = Nadoufmath(5)
calc = calc.sum_of([5]) # 10
calc = calc.dif_of([6]) # 4
calc = calc.factorial() # 24
print(calc)

# OR STYLE 3. JUST USE FUNCTUIONS!

result = sum_of([5, 5])
print(result)
result = dif_of([result, 6])
print(result)
result = factorial(int(result))
print(result)
```

## Documentation 🗎

### Navigation

 - [Usage Overview 🎗](#usage-overview-)
 - [Functional API 🎗](#-functional-api-functions)
    - [Basic Arithmetic](#basic-arithmetic)
    - [Powers & Roots](#powers--roots)
    - [Number Theory](#number-theory)
    - [Trigonometry (in radians)](#trigonometry-in-radians)
    - [Comparison & Properties](#comparison--properties)
    - [Advanced Functions](#advanced-functions)

 - [Object-Oriented API 🎗](#-object-oriented-api-class)
    - [Constructor & Basic](#constructor--basic)
    - [Arithmetic Operations](#arithmetic-operations)
    - [Powers & Roots Methods](#powers--roots-methods)
    - [Number Theory Methods](#number-theory-methods)
    - [Trigonometric Methods (in radians)](#trigonometric-methods-in-radians)
    - [Rounding & Properties](#rounding--properties)
    - [Boolean Checks](#boolean-checks)
    - [Advanced Methods](#advanced-methods)
    - [Utility Methods](#utility-methods)
 - [CONSTANTS](#constants)
 - [Error Handling](#error-handling)


### Usage Overview 🎗

## 📥 Import what you need

Start by importing the functions or classes you want:

```python
# Import everything (all functions and classes)
from nadouf_math import *

# Import specific functions
from nadouf_math import sum_of, dif_of, factorial

# Import the class for OOP style
from nadouf_math import Nadoufmath
```

## 🎖 Functional API (functions)

#### Basic Arithmetic 
| Function | Description | Example | Result |
|----------|-------------|---------|--------|
| `sum_of(list)` | Sum of all numbers | `sum_of([1, 2, 3, 4])` | `10.0` |
| `dif_of(list)` | First - rest | `dif_of([10, 3, 2])` | `5.0` |
| `mult_of(list)` | Product of all | `mult_of([2, 3, 4])` | `24.0` |
| `div_of(list)` | First / rest | `div_of([100, 2, 5])` | `10.0` |
| `int_div_of(list)` | Integer division | `int_div_of([10, 3])` | `3` |


#### Powers & Roots 
| Function | Description | Example | Result |
|----------|-------------|---------|--------|
| `square(x)` | x² | `square(5)` | `25.0` |
| `cube(x)` | x³ | `cube(3)` | `27.0` |
| `power(x, y)` | x^y | `power(2, 10)` | `1024.0` |
| `square_root(x)` | √x | `square_root(25)` | `5.0` |
| `cube_root(x)` | ∛x | `cube_root(-8)` | `-2.0` |


#### Number Theory 
| Function | Description | Example | Result |
|----------|-------------|---------|--------|
| `factorial(n)` | n! (up to 33!) | `factorial(5)` | `120` |
| `gcd_with_int(list)` | GCD of integers | `gcd_with_int([48, 18])` | `6` |
| `gcd_with_float(list)` | GCD of floats (rounded) | `gcd_with_float([48.5, 18.3])` | `6` |
| `lcm_with(list)` | LCM of numbers | `lcm_with([12, 18])` | `36.0` |

#### Trigonometry (in radians) 
| Function | Description | Example | Result |
|----------|-------------|---------|--------|
| `sin(x, terms=20)` | Sine (Taylor series) | `sin(3.14159/2)` | `≈1.0` |
| `cos(x, terms=20)` | Cosine (Taylor series) | `cos(3.14159)` | `≈-1.0` |
| `tan(x, terms=20)` | Tangent | `tan(3.14159/4)` | `≈1.0` |

#### Comparison & Properties 
| Function | Description | Example | Result |
|----------|-------------|---------|--------|
| `floor(x)` | Round down | `floor(5.7)` | `5` |
| `ceil(x)` | Round up | `ceil(5.2)` | `6` |
| `is_positive(x)` | x > 0? | `is_positive(-5)` | `False` |
| `is_negative(x)` | x < 0? | `is_negative(-5)` | `True` |
| `is_integer(x)` | x is integer? | `is_integer(5.0)` | `True` |
| `sign(x)` | Sign of x (-1, 0, 1) | `sign(-42)` | `-1` |
| `is_even(x)` | x is even? | `is_even(4)` | `True` |
| `is_odd(x)` | x is odd? | `is_odd(7)` | `True` |

#### Advanced Functions 
| Function | Description | Example | Result |
|----------|-------------|---------|--------|
| `tetration(base, height)` | base^^height | `tetration(2, 3)` | `16.0` |

## 🎯 Object-Oriented API (class)
Create a calculator object and chain methods - choose any style you prefer!

```python
from nadouf_math import Nadoufmath

# Style 1: One-liner (the fastest)
print(Nadoufmath(7).sum_of([3]).dif_of([5]))  # 5

# Style 2: Step by step 
calc = Nadoufmath(7)
calc = calc.sum_of([3])
calc = calc.dif_of([5])
print(calc)  # 5

# Style 3: Method chaining with variable 
calc = Nadoufmath(7)
result = calc.sum_of([3]).dif_of([5])
print(result)  # 5

# Style 4: Multi-line chaining (most readable) 
result = (Nadoufmath(7)
          .sum_of([3])    # 7 + 3 = 10
          .dif_of([5]))   # 10 - 5 = 5
print(result)  # 5

# Style 5: Reusing the same object 
calc = Nadoufmath(7)
calc.sum_of([3])     # now 10
calc.dif_of([5])     # now 5
print(calc)  # 5
```

#### Constructor & Basic 
| Method | Description | Example |
|--------|-------------|---------|
| `Nadoufmath(value)` | Create instance | `calc = Nadoufmath(10)` |
| `__str__` / `__repr__` | Pretty printing | `print(calc)` → `10` |

#### Arithmetic Operations 
| Method | Description | Example | Result |
|--------|-------------|---------|--------|
| `.sum_of(list)` | Add sum to current | `calc.sum_of([1, 2])` | `self + 3` |
| `.dif_of(list)` | Subtract (first - rest) | `calc.dif_of([3, 2])` | `self - (3-2)` |
| `.mult_of(list)` | Multiply by product | `calc.mult_of([2, 3])` | `self × 6` |
| `.div_of(list)` | Divide by each | `calc.div_of([2, 5])` | `self ÷ 2 ÷ 5` |
| `.int_div_of(list)` | Integer division | `calc.int_div_of([3, 2])` | `self // 3 // 2` |

#### Powers & Roots Methods 
| Method | Description | Example | Result |
|--------|-------------|---------|--------|
| `.square()` | Square the value | `calc.square()` | `self²` |
| `.cube()` | Cube the value | `calc.cube()` | `self³` |
| `.power(exp)` | Raise to power | `calc.power(3)` | `self³` |
| `.power_of_2()` | 2^self | `calc.power_of_2()` | `2^self` |
| `.square_root()` | √self | `calc.square_root()` | `√self` |
| `.cube_root()` | ∛self | `calc.cube_root()` | `∛self` |

#### Number Theory Methods 
| Method | Description | Example | Result |
|--------|-------------|---------|--------|
| `.factorial()` | self! | `calc.factorial()` | `self!` |
| `.gcd_with_int(list)` | GCD with list | `calc.gcd_with_int([48, 18])` | `gcd(self, ...)` |
| `.gcd_with_float(list)` | GCD with floats | `calc.gcd_with_float([48.5, 18.3])` | `gcd(self, ...)` |
| `.lcm_with(list)` | LCM with list | `calc.lcm_with([12, 18])` | `lcm(self, ...)` |


#### Trigonometric Methods (in radians)
| Method | Description | Example | Result |
|--------|-------------|---------|--------|
| `.sin(terms=20)` | Sine | `calc.sin()` | `sin(self)` |
| `.cos(terms=20)` | Cosine | `calc.cos()` | `cos(self)` |
| `.tan(terms=20)` | Tangent | `calc.tan()` | `tan(self)` |

#### Rounding & Properties 
| Method | Description | Example | Result |
|--------|-------------|---------|--------|
| `.floor()` | Round down | `calc.floor()` | `⌊self⌋` |
| `.ceil()` | Round up | `calc.ceil()` | `⌈self⌉` |
| `.sign()` | Set to sign | `calc.sign()` | `-1, 0, or 1` |

#### Boolean Checks 
| Method | Description | Example | Result |
|--------|-------------|---------|--------|
| `.is_positive()` | self > 0? | `calc.is_positive()` | `bool` |
| `.is_negative()` | self < 0? | `calc.is_negative()` | `bool` |
| `.is_even()` | self is even? | `calc.is_even()` | `bool` |
| `.is_odd()` | self is odd? | `calc.is_odd()` | `bool` |

#### Advanced Methods 
| Method | Description | Example | Result |
|--------|-------------|---------|--------|
| `.tetration(height)` | self^^height | `calc.tetration(3)` | `self^^3` |


#### Utility Methods 
| Method | Description | Example |
|--------|-------------|---------|
| `.get_number()` | Get current value | `calc.get_number()` → `float` |


## CONSTANTS

| Constant | Value | Description |
|----------|-------|-------------|
| `number_pi` | `3.141592653589793` | π (pi) |
| `number_e` | `2.718281828459045` | e (Euler's number) |
| `infinity` | `inf` | Positive infinity |

## Error Handling 

| Error Type | When it occurs | Example |
|------------|----------------|---------|
| `ZeroDivisionError` | Division by zero | `div_of([10, 0])` |
| `ValueError` | Negative factorial | `factorial(-5)` |
| `ValueError` | Factorial overflow (n > 33) | `factorial(34)` |
| `ValueError` | Empty list for GCD | `gcd_with_int([])` |
| `ZeroDivisionError` | Tangent of π/2 | `tan(3.14159/2)` |
| `ValueError` | Negative tetration height | `tetration(2, -1)` |


<p align="center">
  <sub>Made with ❤️ and 🦀 by CrabNadouf</sub>
  <br>
  <sub>⚡ Fast • 🛡️ Safe • 🎯 Precise ⚡</sub>
</p>