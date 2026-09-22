use pyo3::prelude::*;

mod advanced;
mod arithmetic;
mod checks;
mod class;
mod functions;
mod lists;
mod number_theory;
mod powers;
mod rounding;
mod trigonometry;
mod consts;

#[pymodule]
fn crabnum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // arithmetic.rs
    m.add_function(wrap_pyfunction!(arithmetic::sum_of, m)?)?;
    m.add_function(wrap_pyfunction!(arithmetic::dif_of, m)?)?;
    m.add_function(wrap_pyfunction!(arithmetic::div_of, m)?)?;
    m.add_function(wrap_pyfunction!(arithmetic::int_div_of, m)?)?;
    m.add_function(wrap_pyfunction!(arithmetic::rem, m)?)?;
    m.add_function(wrap_pyfunction!(arithmetic::product, m)?)?;

    // powers.rs
    m.add_function(wrap_pyfunction!(powers::square, m)?)?;
    m.add_function(wrap_pyfunction!(powers::cube, m)?)?;
    m.add_function(wrap_pyfunction!(powers::power, m)?)?;
    m.add_function(wrap_pyfunction!(powers::square_root, m)?)?;
    m.add_function(wrap_pyfunction!(powers::cube_root, m)?)?;
    m.add_function(wrap_pyfunction!(powers::root, m)?)?;

    // number_theory.rs
    m.add_function(wrap_pyfunction!(number_theory::factorial, m)?)?;
    m.add_function(wrap_pyfunction!(number_theory::gcd, m)?)?;
    m.add_function(wrap_pyfunction!(number_theory::lcm, m)?)?;
    m.add_function(wrap_pyfunction!(number_theory::fibonacci, m)?)?;

    // rounding.rs
    m.add_function(wrap_pyfunction!(rounding::floor, m)?)?;
    m.add_function(wrap_pyfunction!(rounding::ceil, m)?)?;
    m.add_function(wrap_pyfunction!(rounding::absolute, m)?)?;

    // checks.rs
    m.add_function(wrap_pyfunction!(checks::is_positive, m)?)?;
    m.add_function(wrap_pyfunction!(checks::is_negative, m)?)?;
    m.add_function(wrap_pyfunction!(checks::sign, m)?)?;
    m.add_function(wrap_pyfunction!(checks::is_integer, m)?)?;
    m.add_function(wrap_pyfunction!(checks::is_even, m)?)?;
    m.add_function(wrap_pyfunction!(checks::is_odd, m)?)?;

    // trigonometry.rs
    m.add_function(wrap_pyfunction!(trigonometry::sin, m)?)?;
    m.add_function(wrap_pyfunction!(trigonometry::csc, m)?)?;
    m.add_function(wrap_pyfunction!(trigonometry::sec, m)?)?;
    m.add_function(wrap_pyfunction!(trigonometry::cos, m)?)?;
    m.add_function(wrap_pyfunction!(trigonometry::tan, m)?)?;
    m.add_function(wrap_pyfunction!(trigonometry::cot, m)?)?;

    // advanced.rs
    m.add_function(wrap_pyfunction!(advanced::log, m)?)?;
    m.add_function(wrap_pyfunction!(advanced::tetration, m)?)?;

    // class
    m.add_class::<class::Crabnum>()?;

    // lists.rs
    m.add_function(wrap_pyfunction!(lists::mean, m)?)?;
    m.add_function(wrap_pyfunction!(lists::max_value, m)?)?;
    m.add_function(wrap_pyfunction!(lists::max_index, m)?)?;
    m.add_function(wrap_pyfunction!(lists::min_value, m)?)?;
    m.add_function(wrap_pyfunction!(lists::min_index, m)?)?;
    m.add_function(wrap_pyfunction!(lists::sorted_list, m)?)?;
    m.add_function(wrap_pyfunction!(lists::reversed_list, m)?)?;
    m.add_function(wrap_pyfunction!(lists::count, m)?)?;
    m.add_function(wrap_pyfunction!(lists::merge, m)?)?;
    m.add_function(wrap_pyfunction!(lists::median, m)?)?;
    m.add_function(wrap_pyfunction!(lists::unique, m)?)?;
    m.add_function(wrap_pyfunction!(lists::get_range, m)?)?;

    // consts.rs

    // math.rs
    m.add("PI", consts::math::PI)?;
    m.add("E", consts::math::E)?;
    m.add("GOLDEN_RATIO", consts::math::GOLDEN_RATIO)?;
    m.add("SILVER_RATIO", consts::math::SILVER_RATIO)?;
    m.add("GAMMA", consts::math::GAMMA)?;
    m.add("SQRT_2", consts::math::SQRT_2)?;
    m.add("LN_2", consts::math::LN_2)?;
    m.add("CATALAN", consts::math::CATALAN)?;
    m.add("TAU", consts::math::TAU)?;
    m.add("FEIGENBAUM_DELTA", consts::math::FEIGENBAUM_DELTA)?;
    m.add("APERY", consts::math::APERY)?;
    m.add("KHINCHIN", consts::math::KHINCHIN)?;

    // physics.rs
    m.add("SPEED_OF_LIGHT", consts::physics::SPEED_OF_LIGHT)?;
    m.add("GRAVITY", consts::physics::GRAVITY)?;
    m.add("AVOGADRO", consts::physics::AVOGADRO)?;

    // Quantum & Atomic Physics
    m.add("ELEMENTARY_CHARGE", consts::physics::ELEMENTARY_CHARGE)?;
    m.add("ELECTRON_MASS", consts::physics::ELECTRON_MASS)?;
    m.add("PROTON_MASS", consts::physics::PROTON_MASS)?;
    m.add("NEUTRON_MASS", consts::physics::NEUTRON_MASS)?;
    m.add("PLANCK", consts::physics::PLANCK)?;
    m.add("REDUCED_PLANCK", consts::physics::REDUCED_PLANCK)?;
    m.add("VACUUM_PERMITTIVITY", consts::physics::VACUUM_PERMITTIVITY)?;
    m.add("VACUUM_PERMEABILITY", consts::physics::VACUUM_PERMEABILITY)?;

    // Atomic Spectroscopy & Scales 
    m.add("FINE_STRUCTURE", consts::physics::FINE_STRUCTURE)?;
    m.add("BOHR_RADIUS", consts::physics::BOHR_RADIUS)?;
    m.add("RYDBERG", consts::physics::RYDBERG)?;
    m.add("HARTREE_ENERGY", consts::physics::HARTREE_ENERGY)?;
    m.add("BOHR_MAGNETON", consts::physics::BOHR_MAGNETON)?;
    m.add("ATOMIC_MASS_UNIT", consts::physics::ATOMIC_MASS_UNIT)?;

    Ok(())
}
