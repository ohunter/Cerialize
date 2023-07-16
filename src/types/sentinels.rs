use pyo3::basic::CompareOp;
use pyo3::prelude::*;

#[pyclass(module = "_cerialize", frozen, weakref)]
#[derive(Clone, Copy)]
pub struct NativeEndian();

#[pymethods]
impl NativeEndian {
    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}"))
    }

    fn __richcmp__(&self, _other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(false),
            CompareOp::Le => Ok(true),
            CompareOp::Eq => Ok(true),
            CompareOp::Ne => Ok(false),
            CompareOp::Gt => Ok(false),
            CompareOp::Ge => Ok(true),
        }
    }
}

#[pyclass(module = "_cerialize", frozen, weakref)]
#[derive(Clone, Copy)]
pub struct BigEndian();

#[pymethods]
impl BigEndian {
    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}"))
    }

    fn __richcmp__(&self, _other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(false),
            CompareOp::Le => Ok(true),
            CompareOp::Eq => Ok(true),
            CompareOp::Ne => Ok(false),
            CompareOp::Gt => Ok(false),
            CompareOp::Ge => Ok(true),
        }
    }
}

#[pyclass(module = "_cerialize", frozen, weakref)]
#[derive(Clone, Copy)]
pub struct LittleEndian();

#[pymethods]
impl LittleEndian {
    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}"))
    }

    fn __richcmp__(&self, _other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(false),
            CompareOp::Le => Ok(true),
            CompareOp::Eq => Ok(true),
            CompareOp::Ne => Ok(false),
            CompareOp::Gt => Ok(false),
            CompareOp::Ge => Ok(true),
        }
    }
}

#[derive(FromPyObject, Copy, Clone)]
pub enum Endianness {
    #[pyo3(transparent)]
    Native(NativeEndian),
    #[pyo3(transparent)]
    Big(BigEndian),
    #[pyo3(transparent)]
    Little(LittleEndian),
}

impl IntoPy<Py<pyo3::PyAny>> for Endianness {
    fn into_py(self, py: Python<'_>) -> Py<pyo3::PyAny> {
        match self {
            Endianness::Native(value) => Py::new(py, value).unwrap().as_ref(py).into(),
            Endianness::Big(value) => Py::new(py, value).unwrap().as_ref(py).into(),
            Endianness::Little(value) => Py::new(py, value).unwrap().as_ref(py).into(),
        }
    }
}
