import pytest
import nadouf_math as nm
import math
import re
import sys

# ====================== БАЗОВЫЕ АРИФМЕТИЧЕСКИЕ ФУНКЦИИ ======================

class TestBasicArithmetic:
    """Тесты для базовых арифметических операций"""
    
    def test_sum_of(self):
        assert nm.sum_of([1.0, 2.0, 3.0]) == 6.0
        assert nm.sum_of([]) == 0.0
        assert nm.sum_of([-1.0, 1.0]) == 0.0
        assert nm.sum_of([1.5, 2.5]) == 4.0
    
    def test_dif_of(self):
        assert nm.dif_of([10.0, 2.0, 3.0]) == 5.0  # 10 - 2 - 3 = 5
        assert nm.dif_of([]) == 0.0
        assert nm.dif_of([5.0]) == 5.0
        assert nm.dif_of([10.0, 10.0]) == 0.0
        assert nm.dif_of([10.0, -5.0]) == 15.0  # 10 - (-5) = 15
    
    def test_mult_of(self):
        assert nm.mult_of([2.0, 3.0, 4.0]) == 24.0
        assert nm.mult_of([]) == 1.0
        assert nm.mult_of([5.0]) == 5.0
        assert nm.mult_of([2.0, 0.5]) == 1.0
    
    def test_div_of(self):
        assert nm.div_of([10.0, 2.0]) == 5.0
        assert nm.div_of([100.0, 2.0, 5.0]) == 10.0  # 100 / 2 / 5 = 10
        assert nm.div_of([]) == 0.0
        assert nm.div_of([5.0]) == 5.0
        
        with pytest.raises(ZeroDivisionError, match="Division by zero!"):
            nm.div_of([10.0, 0.0])
    
    def test_int_div_of(self):
        assert nm.int_div_of([10, 2]) == 5
        assert nm.int_div_of([100, 2, 5]) == 10
        assert nm.int_div_of([]) == 0
        assert nm.int_div_of([7, 2]) == 3  # Целочисленное деление
        
        with pytest.raises(ZeroDivisionError, match="Division by zero!"):
            nm.int_div_of([10, 0])


# ====================== СТЕПЕННЫЕ ФУНКЦИИ ======================

class TestPowerFunctions:
    """Тесты для функций возведения в степень и корней"""
    
    def test_square(self):
        assert nm.square(5.0) == 25.0
        assert nm.square(-3.0) == 9.0
        assert nm.square(0.0) == 0.0
    
    def test_cube(self):
        assert nm.cube(3.0) == 27.0
        assert nm.cube(-2.0) == -8.0
        assert nm.cube(0.0) == 0.0
    
    def test_power(self):
        assert nm.power(2.0, 3.0) == 8.0
        assert nm.power(4.0, 0.5) == 2.0
        assert nm.power(2.0, -1.0) == 0.5
        assert nm.power(0.0, 5.0) == 0.0
    
    def test_square_root(self):
        assert nm.square_root(9.0) == 3.0
        assert nm.square_root(2.0) == pytest.approx(1.41421356237)
        assert nm.square_root(0.0) == 0.0
        
        with pytest.raises(ValueError, match="Cannot calculate square root of a negative number"):
            nm.square_root(-4.0)
    
    def test_cube_root(self):
        assert nm.cube_root(27.0) == 3.0
        assert nm.cube_root(-8.0) == -2.0
        assert nm.cube_root(0.0) == 0.0


# ====================== ТЕОРИЯ ЧИСЕЛ ======================

class TestNumberTheory:
    """Тесты для GCD, LCM и факториала"""
    
    def test_factorial(self):
        assert nm.factorial(0) == 1
        assert nm.factorial(1) == 1
        assert nm.factorial(5) == 120
        assert nm.factorial(10) == 3628800
        
        with pytest.raises(ValueError, match="negative number"):
            nm.factorial(-5)
        
        with pytest.raises(ValueError, match="overflow"):
            nm.factorial(100)  # Должен переполниться для i64
    
    def test_gcd_with_int(self):
        assert nm.gcd_with_int([12, 18]) == 6
        assert nm.gcd_with_int([100, 75, 50]) == 25
        assert nm.gcd_with_int([7, 13]) == 1
        assert nm.gcd_with_int([-12, 18]) == 6
        assert nm.gcd_with_int([0, 5]) == 5
        assert nm.gcd_with_int([48, 18, 30]) == 6
        
        with pytest.raises(ValueError, match="at least one argument"):
            nm.gcd_with_int([])
    
    def test_gcd_with_float(self):
        # Тест для целых чисел через float
        assert nm.gcd_with_float([12.0, 18.0]) == 6
        assert nm.gcd_with_float([100.0, 75.0]) == 25
        
        # Для дробных - проверяем что результат соответствует GCD округленных значений
        result1 = nm.gcd_with_float([12.3, 18.7])
        expected1 = math.gcd(round(12.3), round(18.7))  # round(12.3)=12, round(18.7)=19 -> GCD=1
        assert result1 == expected1
        
        result2 = nm.gcd_with_float([12.6, 18.2])
        expected2 = math.gcd(round(12.6), round(18.2))  # round(12.6)=13, round(18.2)=18 -> GCD=1
        assert result2 == expected2
        
        result3 = nm.gcd_with_float([48.3, 18.7, 30.2])
        expected3 = math.gcd(math.gcd(round(48.3), round(18.7)), round(30.2))
        assert result3 == expected3
        
        with pytest.raises(ValueError, match="at least one argument"):
            nm.gcd_with_float([])
    
    def test_lcm_with(self):
        assert nm.lcm_with([12.0, 18.0]) == 36.0
        assert nm.lcm_with([4.0, 6.0, 8.0]) == 24.0
        assert nm.lcm_with([7.0, 13.0]) == 91.0
        assert nm.lcm_with([0.0, 5.0]) == 0.0
        assert nm.lcm_with([]) == 0.0


# ====================== ПРОВЕРКА ЧИСЕЛ ======================

class TestNumberChecks:
    """Тесты для функций проверки свойств чисел"""
    
    def test_floor(self):
        assert nm.floor(5.7) == 5
        assert nm.floor(-2.3) == -3
        assert nm.floor(0.0) == 0
    
    def test_ceil(self):
        assert nm.ceil(5.3) == 6
        assert nm.ceil(-2.7) == -2
        assert nm.ceil(0.0) == 0
    
    def test_is_positive(self):
        assert nm.is_positive(5.0) is True
        assert nm.is_positive(-5.0) is False
        assert nm.is_positive(0.0) is False
    
    def test_is_negative(self):
        assert nm.is_negative(-5.0) is True
        assert nm.is_negative(5.0) is False
        assert nm.is_negative(0.0) is False
    
    def test_is_integer(self):
        assert nm.is_integer(5.0) is True
        assert nm.is_integer(5.7) is False
        assert nm.is_integer(-3.0) is True
        assert nm.is_integer(-3.5) is False
    
    def test_sign(self):
        assert nm.sign(10.0) == 1
        assert nm.sign(-5.0) == -1
        assert nm.sign(0.0) == 0
    
    def test_is_even(self):
        assert nm.is_even(4) is True
        assert nm.is_even(7) is False
        assert nm.is_even(0) is True
        assert nm.is_even(-2) is True
        assert nm.is_even(-3) is False
    
    def test_is_odd(self):
        assert nm.is_odd(3) is True
        assert nm.is_odd(8) is False
        assert nm.is_odd(0) is False
        assert nm.is_odd(-5) is True
        assert nm.is_odd(-4) is False


# ====================== ТРИГОНОМЕТРИЯ ======================

class TestTrigonometry:
    """Тесты для тригонометрических функций"""
    
    def test_sin(self):
        # Базовые значения
        assert nm.sin(0.0) == pytest.approx(0.0)
        assert nm.sin(math.pi/2) == pytest.approx(1.0, rel=1e-10)
        assert nm.sin(math.pi) == pytest.approx(0.0, abs=1e-10)
        assert nm.sin(3*math.pi/2) == pytest.approx(-1.0, rel=1e-10)
        
        # Периодичность
        assert nm.sin(2*math.pi) == pytest.approx(0.0, abs=1e-10)
        assert nm.sin(5*math.pi/2) == pytest.approx(1.0, rel=1e-10)
        
        # Малое количество членов ряда
        assert nm.sin(1.0, terms=5) == pytest.approx(0.841470, rel=1e-5)
        assert nm.sin(1.0, terms=10) == pytest.approx(math.sin(1.0), rel=1e-10)
    
    def test_cos(self):
        # Базовые значения
        assert nm.cos(0.0) == pytest.approx(1.0)
        assert nm.cos(math.pi/2) == pytest.approx(0.0, abs=1e-10)
        assert nm.cos(math.pi) == pytest.approx(-1.0, rel=1e-10)
        assert nm.cos(2*math.pi) == pytest.approx(1.0, rel=1e-10)
        
        # Малое количество членов ряда
        assert nm.cos(1.0, terms=5) == pytest.approx(0.540302, rel=1e-5)
        assert nm.cos(1.0, terms=10) == pytest.approx(math.cos(1.0), rel=1e-10)
    
    def test_tan(self):
        # Базовые значения
        assert nm.tan(0.0) == pytest.approx(0.0)
        assert nm.tan(math.pi/4) == pytest.approx(1.0, rel=1e-10)
        assert nm.tan(math.pi) == pytest.approx(0.0, abs=1e-10)
        
        # Периодичность
        assert nm.tan(5*math.pi/4) == pytest.approx(1.0, rel=1e-10)
        
        # Ошибка при cos ≈ 0
        with pytest.raises(ZeroDivisionError, match="undefined|zero"):
            nm.tan(math.pi/2)


# ====================== СПЕЦИАЛЬНЫЕ ФУНКЦИИ ======================

class TestSpecialFunctions:
    """Тесты для специальных функций"""
    
    def test_tetration(self):
        # Тетрация: база ^^ высота
        assert nm.tetration(2.0, 0) == 1.0  # 2^^0 = 1
        assert nm.tetration(2.0, 1) == 2.0  # 2^^1 = 2
        assert nm.tetration(2.0, 2) == 2.0**2.0 == 4.0  # 2^^2 = 2^2 = 4
        assert nm.tetration(2.0, 3) == 2.0**4.0 == 16.0  # 2^^3 = 2^(2^2) = 16
        assert nm.tetration(2.0, 4) == 2.0**16.0 == 65536.0  # 2^^4 = 2^(2^(2^2)) = 65536
        
        assert nm.tetration(3.0, 2) == 27.0  # 3^^2 = 3^3 = 27
        assert nm.tetration(3.0, 3) == 3.0**27.0  # 3^^3 = 3^(3^3) = 3^27
        
        with pytest.raises(ValueError, match="negative"):
            nm.tetration(2.0, -1)
        
        with pytest.raises(ValueError, match="too big|infinity"):
            nm.tetration(10.0, 100)  # должно переполниться


# ====================== КЛАСС NADOUFMATH ======================

class TestNadoufmathClass:
    """Тесты для класса Nadoufmath"""
    
    def test_creation_and_repr(self):
        obj = nm.Nadoufmath(5.0)
        assert obj.get_number() == 5.0
        # Принимаем оба формата: "Nadoufmath(5)" или "Nadoufmath(5.0)"
        repr_str = repr(obj)
        assert re.match(r"Nadoufmath\(5(\.0)?\)", repr_str)
        
        obj2 = nm.Nadoufmath(5.5)
        assert repr(obj2) == "Nadoufmath(5.5)"
        
        obj3 = nm.Nadoufmath(0.0)
        assert re.match(r"Nadoufmath\(0(\.0)?\)", repr(obj3))
    
    def test_sum_of_method(self):
        obj = nm.Nadoufmath(5.0)
        obj.sum_of([1.0, 2.0, 3.0])
        assert obj.get_number() == 11.0  # 5 + 6 = 11
        
        obj2 = nm.Nadoufmath(10.0)
        obj2.sum_of([])
        assert obj2.get_number() == 10.0
    
    def test_dif_of_method(self):
        obj = nm.Nadoufmath(10.0)
        obj.dif_of([2.0, 3.0])
        # 10 - (2 - 3) = 10 - (-1) = 11
        assert obj.get_number() == 11.0
        
        obj2 = nm.Nadoufmath(20.0)
        obj2.dif_of([5.0, 2.0, 1.0])  # 5 - 2 - 1 = 2
        assert obj2.get_number() == 18.0  # 20 - 2 = 18
        
        obj3 = nm.Nadoufmath(15.0)
        obj3.dif_of([])
        assert obj3.get_number() == 15.0
    
    def test_div_of_method(self):
        obj = nm.Nadoufmath(100.0)
        obj.div_of([2.0, 5.0])
        assert obj.get_number() == 10.0
        
        obj2 = nm.Nadoufmath(50.0)
        obj2.div_of([2.0])
        assert obj2.get_number() == 25.0
        
        obj3 = nm.Nadoufmath(10.0)
        obj3.div_of([])
        assert obj3.get_number() == 10.0
        
        with pytest.raises(ZeroDivisionError):
            obj = nm.Nadoufmath(10.0)
            obj.div_of([0.0])
    
    def test_int_div_of_method(self):
        obj = nm.Nadoufmath(100.0)
        obj.int_div_of([2, 5])
        assert obj.get_number() == 10.0
        
        obj2 = nm.Nadoufmath(7.0)
        obj2.int_div_of([2])
        assert obj2.get_number() == 3.0  # Целочисленное деление 7//2 = 3
        
        with pytest.raises(ZeroDivisionError):
            obj = nm.Nadoufmath(10.0)
            obj.int_div_of([0])
    
    def test_mult_of_method(self):
        obj = nm.Nadoufmath(5.0)
        obj.mult_of([2.0, 3.0])
        assert obj.get_number() == 30.0
        
        obj2 = nm.Nadoufmath(10.0)
        obj2.mult_of([])
        assert obj2.get_number() == 10.0
    
    def test_power_methods(self):
        obj = nm.Nadoufmath(4.0)
        obj.square()
        assert obj.get_number() == 16.0
        
        obj.cube()
        assert obj.get_number() == 4096.0  # 16^3 = 4096
        
        obj = nm.Nadoufmath(16.0)
        obj.square_root()
        assert obj.get_number() == 4.0
        
        obj.cube_root()
        assert obj.get_number() == pytest.approx(1.5874, rel=1e-4)
        
        obj = nm.Nadoufmath(2.0)
        obj.power(3.0)
        assert obj.get_number() == 8.0
    
    def test_factorial_method(self):
        obj = nm.Nadoufmath(5.0)
        obj.factorial()
        assert obj.get_number() == 120.0
        
        obj2 = nm.Nadoufmath(0.0)
        obj2.factorial()
        assert obj2.get_number() == 1.0
        
        with pytest.raises(ValueError):
            obj = nm.Nadoufmath(-5.0)
            obj.factorial()
    
    def test_gcd_methods(self):
        obj = nm.Nadoufmath(0.0)
        obj.gcd_with_int([12, 18])
        assert obj.get_number() == 6.0

        obj2 = nm.Nadoufmath(0.0)
        obj2.gcd_with_int([100, 75, 50])
        assert obj2.get_number() == 25.0

        obj3 = nm.Nadoufmath(0.0)
        # Используем числа, которые после округления дают GCD 6
        obj3.gcd_with_float([12.0, 18.0])
        assert obj3.get_number() == 6.0
        
        # Тестируем что метод возвращает GCD от округленных значений
        obj4 = nm.Nadoufmath(0.0)
        obj4.gcd_with_float([12.3, 18.7])
        expected = float(math.gcd(round(12.3), round(18.7)))
        assert obj4.get_number() == expected
        
        with pytest.raises(ValueError):
            obj = nm.Nadoufmath(0.0)
            obj.gcd_with_int([])
    
    def test_lcm_method(self):
        # Тест 1: Обычный случай
        obj = nm.Nadoufmath(0.0)
        obj.lcm_with([12.0, 18.0])
        assert obj.get_number() == 36.0
        
        # Тест 2: Три числа
        obj2 = nm.Nadoufmath(0.0)
        obj2.lcm_with([4.0, 6.0, 8.0])
        assert obj2.get_number() == 24.0
        
        # Тест 3: Пустой список - должно стать 0.0 (согласно коду)
        obj3 = nm.Nadoufmath(5.0)
        obj3.lcm_with([])
        assert obj3.get_number() == 0.0  # Исправлено: 5.0 -> 0.0
        
        # Тест 4: Проверяем что начальное число не влияет на результат
        obj4 = nm.Nadoufmath(100.0)
        obj4.lcm_with([12.0, 18.0])
        assert obj4.get_number() == 36.0  # Должно перезаписаться на LCM
        
        # Тест 5: С отрицательными числами
        obj5 = nm.Nadoufmath(0.0)
        obj5.lcm_with([-4.0, 6.0])
        assert obj5.get_number() == 12.0  # LCM от абсолютных значений
    
    def test_floor_ceil_methods(self):
        obj = nm.Nadoufmath(5.7)
        obj.floor()
        assert obj.get_number() == 5.0
        
        obj = nm.Nadoufmath(5.3)
        obj.ceil()
        assert obj.get_number() == 6.0
        
        obj = nm.Nadoufmath(-2.3)
        obj.floor()
        assert obj.get_number() == -3.0
    
    def test_sign_method(self):
        obj = nm.Nadoufmath(10.0)
        obj.sign()
        assert obj.get_number() == 1.0
        
        obj = nm.Nadoufmath(-5.0)
        obj.sign()
        assert obj.get_number() == -1.0
        
        obj = nm.Nadoufmath(0.0)
        obj.sign()
        assert obj.get_number() == 0.0
    
    def test_boolean_methods(self):
        obj = nm.Nadoufmath(5.0)
        assert obj.is_positive() is True
        assert obj.is_negative() is False
        
        obj = nm.Nadoufmath(-3.0)
        assert obj.is_positive() is False
        assert obj.is_negative() is True
        
        obj = nm.Nadoufmath(4.0)
        assert obj.is_even() is True
        assert obj.is_odd() is False
        
        obj = nm.Nadoufmath(7.0)
        assert obj.is_even() is False
        assert obj.is_odd() is True
    
    def test_trig_methods(self):
        obj = nm.Nadoufmath(0.0)
        obj.cos()
        assert obj.get_number() == pytest.approx(1.0)
        
        obj = nm.Nadoufmath(math.pi/2)
        obj.sin()
        assert obj.get_number() == pytest.approx(1.0, rel=1e-10)
        
        obj = nm.Nadoufmath(math.pi/4)
        obj.tan()
        assert obj.get_number() == pytest.approx(1.0, rel=1e-10)
        
        # Тест с параметром terms
        obj = nm.Nadoufmath(1.0)
        obj.cos(terms=5)
        assert obj.get_number() == pytest.approx(0.540302, rel=1e-5)
    
    def test_tetration_method(self):
        obj = nm.Nadoufmath(2.0)
        obj.tetration(3)
        assert obj.get_number() == 16.0
        
        obj2 = nm.Nadoufmath(3.0)
        obj2.tetration(2)
        assert obj2.get_number() == 27.0
        
        with pytest.raises(ValueError):
            obj = nm.Nadoufmath(2.0)
            obj.tetration(-1)
    
    def test_power_of_2(self):
        obj = nm.Nadoufmath(3.0)
        obj.power_of_2()
        assert obj.get_number() == 8.0  # 2^3 = 8
        
        obj2 = nm.Nadoufmath(0.0)
        obj2.power_of_2()
        assert obj2.get_number() == 1.0  # 2^0 = 1
        
        obj3 = nm.Nadoufmath(-1.0)
        obj3.power_of_2()
        assert obj3.get_number() == 0.5  # 2^-1 = 0.5


# ====================== КОНСТАНТЫ ======================

class TestConstants:
    """Тесты для констант модуля"""
    
    def test_pi(self):
        assert nm.number_pi == pytest.approx(math.pi)
    
    def test_e(self):
        assert nm.number_e == pytest.approx(math.e)
    
    def test_infinity(self):
        assert nm.infinity == float('inf')
        assert math.isinf(nm.infinity)