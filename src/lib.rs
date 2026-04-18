use pyo3::prelude::*;

mod functions;

#[pymodule]
fn crabnum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(functions::sum_of, m)?)?;
    m.add_function(wrap_pyfunction!(functions::dif_of, m)?)?;
    m.add_function(wrap_pyfunction!(functions::div_of, m)?)?;
    m.add_function(wrap_pyfunction!(functions::int_div_of, m)?)?;
    m.add_function(wrap_pyfunction!(functions::product, m)?)?;
    m.add_function(wrap_pyfunction!(functions::square, m)?)?;
    m.add_function(wrap_pyfunction!(functions::cube, m)?)?;
    m.add_function(wrap_pyfunction!(functions::power, m)?)?;
    m.add_function(wrap_pyfunction!(functions::square_root, m)?)?;
    m.add_function(wrap_pyfunction!(functions::cube_root, m)?)?;
    m.add_function(wrap_pyfunction!(functions::root, m)?)?;
    m.add_function(wrap_pyfunction!(functions::factorial, m)?)?;
    m.add_function(wrap_pyfunction!(functions::gcd, m)?)?;
    m.add_function(wrap_pyfunction!(functions::lcm, m)?)?;
    m.add_function(wrap_pyfunction!(functions::floor, m)?)?;
    m.add_function(wrap_pyfunction!(functions::ceil, m)?)?;
    m.add_function(wrap_pyfunction!(functions::is_positive, m)?)?;
    m.add_function(wrap_pyfunction!(functions::is_negative, m)?)?;
    m.add_function(wrap_pyfunction!(functions::sign, m)?)?;
    m.add_function(wrap_pyfunction!(functions::is_integer, m)?)?;
    m.add_function(wrap_pyfunction!(functions::is_even, m)?)?;
    m.add_function(wrap_pyfunction!(functions::is_odd, m)?)?;
    m.add_function(wrap_pyfunction!(functions::sin, m)?)?;
    m.add_function(wrap_pyfunction!(functions::cos, m)?)?;
    m.add_function(wrap_pyfunction!(functions::tan, m)?)?;
    m.add_function(wrap_pyfunction!(functions::tetration, m)?)?;
    Ok(())
}