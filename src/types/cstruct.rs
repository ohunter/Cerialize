use super::{Endianness, NativeEndian, PyShaped};
use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyType};
use std::cell::RefCell;

#[pyclass(module = "_cerialize", name = "cstruct", subclass, weakref, extends=PyShaped)]
pub struct CStruct {
    buffer: RefCell<Vec<u8>>,
    endianness: Endianness,
}

#[pymethods]
impl CStruct {
    #[new]
    fn new(buffer: &[u8], endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        (
            Self {
                buffer: RefCell::new(Vec::from(buffer)),
                endianness,
            },
            PyShaped::new(),
        )
    }

    #[classmethod]
    fn __packed_size__(cls: &PyType) -> PyResult<usize> {
        let mut capacity = 0_usize;
        // This assumes that there are no padding bytes
        // That should probably be fixed at some point
        let fields = cls.getattr("_CFIELDS")?.downcast::<PyDict>()?;
        for (_, value) in fields {
            capacity += value.call_method0("__packed_size__")?.extract::<usize>()?;
        }
        Ok(capacity)
    }

    fn _type_and_offset_of(slf: &PyCell<Self>, attr: String) -> PyResult<(&PyType, usize)> {
        let fields = slf.getattr("_CFIELDS")?.downcast::<PyDict>()?;
        let mut attr_type = None;

        let offset = fields
            .iter()
            .map_while(|(key, value)| {
                let cur = key.extract::<&str>().unwrap();
                match cur == attr {
                    false => Some(
                        value
                            .call_method0("__packed_size__")
                            .unwrap()
                            .extract::<usize>()
                            .unwrap(),
                    ),
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

    fn _offset_of(slf: &PyCell<Self>, attr: String) -> PyResult<usize> {
        Ok(Self::_type_and_offset_of(slf, attr)?.1)
    }

    fn _type_of(slf: &PyCell<Self>, attr: String) -> PyResult<&PyType> {
        Ok(Self::_type_and_offset_of(slf, attr)?.0)
    }

    fn __getattr__(slf: &PyCell<Self>, attr: String) -> PyResult<&PyAny> {
        let (attr_type, buffer_offset) = Self::_type_and_offset_of(slf, attr)?;
        let type_size = attr_type
            .call_method0("__packed_size__")?
            .extract::<usize>()?;

        Ok(attr_type.call_method1(
            "__new__",
            (
                attr_type,
                &slf.borrow().buffer.borrow()[buffer_offset..buffer_offset + type_size],
                Some(slf.borrow().endianness),
            ),
        )?)
    }
}
