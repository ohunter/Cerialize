mod types;

use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyLong, PyString, PyType};
use std::cell::RefCell;

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

// #[pyfunction]
// fn serialize()

#[pyclass(name = "_cstruct", subclass, weakref)]
struct CStruct {
    buffer: RefCell<Vec<u8>>,
}

impl CStruct {
    fn type_and_offset_of(slf: &PyCell<Self>, attr: String) -> PyResult<(&PyType, usize)> {
        let fields = slf.getattr("_CFIELDS")?.downcast::<PyDict>()?;
        let mut attr_type = None;

        let offset = fields
            .iter()
            .map_while(|(key, value)| {
                let cur = key.downcast::<PyString>().unwrap();
                match cur.to_string() == attr {
                    false => {
                        let field_type = value.downcast::<PyType>().unwrap();
                        Some(
                            field_type
                                .call_method0("__packed_size__")
                                .unwrap()
                                .downcast::<PyLong>()
                                .unwrap()
                                .extract::<usize>()
                                .unwrap(),
                        )
                    }
                    true => {
                        attr_type = Some(value.downcast::<PyType>().unwrap());
                        None
                    }
                }
            })
            .fold(0_usize, |acc, val| acc + val);

        match attr_type {
            Some(type_) => Ok((type_, offset)),
            None => Err(PyAttributeError::new_err(format!(
                "Unable to locate attribute {attr}"
            ))),
        }
    }
}

#[pymethods]
impl CStruct {
    #[new]
    fn new(capacity: usize) -> Self {
        Self {
            buffer: RefCell::new(vec![0_u8; capacity]),
        }
    }

    #[classmethod]
    fn from_buffer(cls: &PyType, buffer: &[u8]) -> PyResult<Self> {
        let minimum_size = cls
            .call_method0("__packed_size__")?
            .downcast::<PyLong>()?
            .extract::<usize>()?;
        Ok(Self::from(&buffer[..minimum_size]))
    }

    #[classmethod]
    fn __packed_size__(cls: &PyType) -> PyResult<usize> {
        let mut capacity = 0_usize;
        // This assumes that there are no padding bytes
        // That should probably be fixed at some point
        let fields = cls.getattr("_CFIELDS")?.downcast::<PyDict>()?;
        for (_key, value) in fields {
            let field_type = value.downcast::<PyType>()?;
            capacity += field_type
                .call_method0("__packed_size__")?
                .downcast::<PyLong>()?
                .extract::<usize>()?;
        }
        Ok(capacity)
    }

    fn _offset_of(slf: &PyCell<Self>, attr: String) -> PyResult<usize> {
        Ok(Self::type_and_offset_of(slf, attr)?.1)
    }

    fn _type_of(slf: &PyCell<Self>, attr: String) -> PyResult<&PyType> {
        Ok(Self::type_and_offset_of(slf, attr)?.0)
    }

    fn __getattr__(slf: &PyCell<Self>, attr: String) -> PyResult<&PyAny> {
        let (attr_type, buffer_offset) = Self::type_and_offset_of(slf, attr)?;
        let type_size = attr_type
            .call_method0("__packed_size__")?
            .downcast::<PyLong>()?
            .extract::<usize>()?;

        Ok(attr_type.call_method1(
            "from_buffer",
            (&slf.borrow().buffer.borrow()[buffer_offset..buffer_offset + type_size],),
        )?)
    }
}

impl From<&[u8]> for CStruct {
    fn from(value: &[u8]) -> Self {
        Self {
            buffer: RefCell::new(Vec::from(value)),
        }
    }
}

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
    m.add_class::<CStruct>()?;
    Ok(())
}
