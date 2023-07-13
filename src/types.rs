use half::f16;
use pyo3::types::PyType;
use pyo3::{prelude::*, pyclass::CompareOp};

#[pyclass(name = "_bool", subclass, weakref)]
pub struct Bool(bool);

#[pymethods]
impl Bool {
    #[new]
    fn new(value: bool) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}

#[pyclass(name = "_i8", subclass, weakref)]
pub struct Int8(i8);

#[pymethods]
impl Int8 {
    #[new]
    fn new(value: i8) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}

#[pyclass(name = "_i16", subclass, weakref)]
pub struct Int16(i16);

#[pymethods]
impl Int16 {
    #[new]
    fn new(value: i16) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}

#[pyclass(name = "_i32", subclass, weakref)]
pub struct Int32(i32);

#[pymethods]
impl Int32 {
    #[new]
    fn new(value: i32) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}

#[pyclass(name = "_i64", subclass, weakref)]
pub struct Int64(i64);

#[pymethods]
impl Int64 {
    #[new]
    fn new(value: i64) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}

#[pyclass(name = "_u8", subclass, weakref)]
pub struct Uint8(u8);

#[pymethods]
impl Uint8 {
    #[new]
    fn new(value: u8) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}

#[pyclass(name = "_u16", subclass, weakref)]
pub struct Uint16(u16);

#[pymethods]
impl Uint16 {
    #[new]
    fn new(value: u16) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}

#[pyclass(name = "_u32", subclass, weakref)]
pub struct Uint32(u32);

#[pymethods]
impl Uint32 {
    #[new]
    fn new(value: u32) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}

#[pyclass(name = "_u64", subclass, weakref)]
pub struct Uint64(u64);

#[pymethods]
impl Uint64 {
    #[new]
    fn new(value: u64) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}

#[pyclass(name = "_f16", subclass, weakref)]
pub struct Float16(f16);

#[pymethods]
impl Float16 {
    #[new]
    fn new(value: f32) -> Self {
        Self(f16::from_f32(value))
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}

#[pyclass(name = "_f32", subclass, weakref)]
pub struct Float32(f32);

#[pymethods]
impl Float32 {
    #[new]
    fn new(value: f32) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}

#[pyclass(name = "_f64", subclass, weakref)]
pub struct Float64(f64);

#[pymethods]
impl Float64 {
    #[new]
    fn new(value: f64) -> Self {
        Self(value)
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        Ok(format!("{class_name}({})", slf.borrow().0))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.0 < other.0),
            CompareOp::Le => Ok(self.0 <= other.0),
            CompareOp::Eq => Ok(self.0 == other.0),
            CompareOp::Ne => Ok(self.0 != other.0),
            CompareOp::Gt => Ok(self.0 > other.0),
            CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(std::mem::size_of::<Self>())
    }
}
