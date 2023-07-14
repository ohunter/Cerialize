mod types;

use pyo3::prelude::*;

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
    m.add_class::<types::CStruct>()?;
    m.add_class::<types::Native>()?;
    m.add_class::<types::Big>()?;
    m.add_class::<types::Little>()?;
    Ok(())
}
