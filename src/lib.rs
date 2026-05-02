use pyo3::prelude::*;

mod class;
mod functions;
mod lists;

#[pymodule]
fn crabnum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // functions
    m.add_function(wrap_pyfunction!(functions::sum_of, m)?)?;
    m.add_function(wrap_pyfunction!(functions::dif_of, m)?)?;
    m.add_function(wrap_pyfunction!(functions::div_of, m)?)?;
    m.add_function(wrap_pyfunction!(functions::int_div_of, m)?)?;
    m.add_function(wrap_pyfunction!(functions::rem, m)?)?;
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
    m.add_function(wrap_pyfunction!(functions::csc, m)?)?;
    m.add_function(wrap_pyfunction!(functions::sec, m)?)?;
    m.add_function(wrap_pyfunction!(functions::cos, m)?)?;
    m.add_function(wrap_pyfunction!(functions::tan, m)?)?;
    m.add_function(wrap_pyfunction!(functions::cot, m)?)?;
    m.add_function(wrap_pyfunction!(functions::fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(functions::absolute, m)?)?;
    m.add_function(wrap_pyfunction!(functions::log, m)?)?;
    m.add_function(wrap_pyfunction!(functions::tetration, m)?)?;

    // class
    m.add_class::<class::Crabnum>()?;

    // lists
    m.add_function(wrap_pyfunction!(lists::mean, m)?)?;
    m.add_function(wrap_pyfunction!(lists::biggest, m)?)?;
    m.add_function(wrap_pyfunction!(lists::biggest_index, m)?)?;
    m.add_function(wrap_pyfunction!(lists::smallest, m)?)?;
    m.add_function(wrap_pyfunction!(lists::smallest_index, m)?)?;
    m.add_function(wrap_pyfunction!(lists::clear, m)?)?;
    m.add_function(wrap_pyfunction!(lists::sorted_list, m)?)?;
    m.add_function(wrap_pyfunction!(lists::reversed_list, m)?)?;
    m.add_function(wrap_pyfunction!(lists::count, m)?)?;
    m.add_function(wrap_pyfunction!(lists::merge, m)?)?;
    m.add_function(wrap_pyfunction!(lists::median, m)?)?;
    Ok(())
}
