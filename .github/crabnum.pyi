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


