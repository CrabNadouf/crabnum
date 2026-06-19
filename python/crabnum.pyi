from typing import List, Union, Optional
from numbers import Number

class Crabnum:
    """Class for mathematical operations on numbers."""

    def __init__(self, number: float) -> None:
        """Initialize Crabnum with a number."""
        ...

    def __str__(self) -> str:
        """Return string representation."""
        ...

    def __repr__(self) -> str:
        """Return representation string."""
        ...

    def sum_of(self, *args: List[float]) -> Crabnum:
        """Add numbers to the current value.

        Args:
            *args: a list with numbers

        Returns:
            Crabnum: New instance with result
        """
        ...

    def dif_of(self, *args: List[float]) -> Crabnum:
        """Subtract numbers from the current value.

        Args:
            *args: a list with numbers

        Returns:
            Crabnum: New instance with result
        """
        ...

    def div_of(self, *args: List[float]) -> Crabnum:
        """Division current value by product of numbers.

        Args:
            *args: a list with numbers

        Returns:
            Crabnum: New instance with result
        """
        ...

    def int_div_of(self, *args: List[float]) -> Crabnum:
        """Integer division current value by product of numbers.

        Args:
            *args: a list with numbers

        Returns:
            Crabnum: New instance with result
        """
        ...

    def rem(self, b: float) -> Crabnum:
        """Get remainder of current value divided by b.

        Args:
            b: a float number

        Returns:
            Crabnum: New instance with result
        """
        ...

    def product(self, *args: List[float]) -> Crabnum:
        """Multiply current value by product of numbers.

        Args:
            *args: a list with numbers

        Returns:
            Crabnum: New instance with result
        """
        ...

    def square(self) -> Crabnum:
        """Square the current value.

        Returns: 
            Crabnum: New instance with result
        """
        ...

    def cube(self) -> Crabnum:
        """Cube the current value.

        Returns:
            Crabnum: New instance with result
        """
        ...

    def power(self, exp: float) -> Crabnum:
        """Raise current value to a power.

        Args:
            exp: Exponent (a float number)

        Returns:
            Crabnum: New instance with result
        """
        ...

    def square_root(self) -> Crabnum:
        """Get square root of current value.

        Returns:
            Crabnum: New instance with result

        Raises:
            ValueError: If self.number is negative
        """
        ...

    def cube_root(self) -> Crabnum:
        """Get cube root of current value.

        Returns:
            Crabnum: New instance with result
        """
        ...

    def root(self, power: float) -> Crabnum:
        """Get power-th root of current value.
        
        Args:
            power: Root degree

        Returns:
            Crabnum: New instance with result

        Raises:
            ValueError: If power is negative
        """
        ...

    def factorial(self) -> Crabnum:
        """Calculate factorial of current value.

        Returns:
            Crabnum: New instance with result

        Raises:
            ValueError: If result is too large or self.number is negative
        """
        ...

    def gcd(self, *args: List[int]) -> Crabnum:
        """Calculate GCD of current value and arguments.

        Args:
            *args: Numbers to compute GCD with
        
        Returns:
            Crabnum: New instance with result

        Raises:
            ValueError: If self.number is not integer
        """
        ...

    def lcm(self, *args: List[int]) -> Crabnum:
        """Calculate LCM of current value and arguments.

        Args:
            *args: Numbers to compute LCM with

        Returns:
            Crabnum: New instance with result

        Raises:
            ValueError: If number is not integer
        """
        ...

    def floor(self) -> Crabnum:
        """Floor the current value.

        Returns:
            Crabnum: New instance with result
        """
        ...

    def ceil(self) -> Crabnum:
        """Ceil the current value.

        Returns:
            Crabnum: New instance with result
        """
        ...

    def is_positive(self) -> bool:
        """Check if the current value is strictly greater than zero.

        Returns:
            bool: True if value > 0, False otherwise.
        """
        ...

    def is_negative(self) -> bool:
        """Check if the current value is strictly less than zero.

        Returns:
            bool: True if value < 0, False otherwise.
        """
        ...

    def sign(self) -> int:
        """Check the sign of the current value.
        
        Returns:
            int: -1 if the current value < 0, 0 if the current value = 0, 1 if the current value > 0
        """
        ...

    def is_integer(self) -> bool:
        """Check if the current value is integer.
        
        Returns:
            bool: True if the current value is integer, False otherwise.
        """
        ...

    def is_even(self) -> bool:
        """Check if the current value is even.

        Returns:
            bool: True if the current value is even, False otherwise.
        """
        ...

    def is_odd(self) -> bool:
        """Check if the current value is odd.

        Returns:
            bool: True if the current value is odd, False otherwise.
        """
        ...

    def sin(self) -> Crabnum:
        """Get the sine of the current value.

        Returns:
            Crabnum: New instance with result 
        """
        ...

    def csc(self) -> Crabnum:
        """Get the cosecant of the current value.

        Returns:
            Crabnum: New instance with result
        """
        ...
    
    def cos(self) -> Crabnum:
        """Get the cosine of the current value.

        Returns:
            Crabnum: New instance with result
        """
        ...

    def sec(self) -> Crabnum:
        """Get the secant of the current value.

        Returns:
            Crabnum: New instance with result
        """
        ...

    def tan(self) -> Crabnum:
        """Get the tangent of the current value.

        Returns:
            Crabnum: New instance with result
        """
        ...

    def cot(self) -> Crabnum:
        """Get the cotangent of the current value.

        Returns:
            Crabnum: New instance with result
        """
        ...

    def fibonacci(self) -> List[int]:
        """Generate the Fibonacci sequence up to the current value.

        Returns:
            List[int]: A list of Fibonacci numbers up to the current value.
        """
        ...

    def absolute(self) -> Crabnum:
        """Get the absolute value of the current value.

        Returns:
            Crabnum: New instance with result
        """
        ...

    def log(selff, base: float) -> Crabnum:
        """Compute the logarithm of the current value using the given base.

        Args:
            base (float): The base of the logarithm. Must be > 0 and != 1.
        
        Returns:
            Crabnum: The logarithm of the current value with the specified base.
        """
        ...






