mod types;

use pyo3::prelude::*;

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// #[pyfunction]
// fn serialize()

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_cerialize")]
fn cerialize(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<types::Bool>()?;
    m.add_class::<types::Int8>()?;
    m.add_class::<types::Int16>()?;
    m.add_class::<types::Int32>()?;
    m.add_class::<types::Int64>()?;
    m.add_class::<types::Uint8>()?;
    m.add_class::<types::Uint16>()?;
    m.add_class::<types::Uint32>()?;
    m.add_class::<types::Uint64>()?;
    m.add_class::<types::Float16>()?;
    m.add_class::<types::Float32>()?;
    m.add_class::<types::Float64>()?;
    Ok(())
}