use half::f16;
use pyo3::exceptions::PyAttributeError;
use pyo3::types::{PyDict, PyLong, PyString, PyType};
use pyo3::types::PyBytes;
use pyo3::{prelude::*, pyclass::CompareOp};
use std::cell::RefCell;

/// Converts a slice to an array of the specified size
fn buffer_alias<const N: usize>(buffer: &[u8]) -> &[u8; N] {
    buffer.try_into().expect("slice with incorrect length")
}

#[pyclass(weakref)]
#[derive(Clone)]
pub struct Native();

#[pyclass(weakref)]
#[derive(Clone)]
pub struct Big();

#[pyclass(weakref)]
#[derive(Clone)]
pub struct Little();

#[derive(FromPyObject)]
pub enum Endianness{
    #[pyo3(transparent)]
    Native(Native),
    #[pyo3(transparent)]
    Big(Big),
    #[pyo3(transparent)]
    Little(Little),
}

#[pyclass(name = "_cstruct", subclass, weakref)]
pub struct CStruct {
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
    fn new(buffer: &[u8]) -> Self {
        Self {
            buffer: RefCell::new(Vec::from(buffer)),
        }
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
            "__new__",
            (
                attr_type,
                &slf.borrow().buffer.borrow()[buffer_offset..buffer_offset + type_size],
            ),
        )?)
    }
}

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
pub struct Int64{
    buffer: RefCell<[u8; std::mem::size_of::<i64>()]>,
    endianness: Endianness,
}

#[pymethods]
impl Int64 {
    const PACKED_SIZE: usize = std::mem::size_of::<i64>();
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<i64>().unwrap();
                    Self { buffer: RefCell::new([0_u8; std::mem::size_of::<i64>()]), endianness}
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self{buffer: RefCell::new(*buffer_alias::<{std::mem::size_of::<i64>()}>(buffer)), endianness}
                }
                else {
                    Self { buffer: RefCell::new([0_u8; std::mem::size_of::<i64>()]), endianness}
                }
            }
            None => Self { buffer: RefCell::new([0_u8; std::mem::size_of::<i64>()]), endianness}
        }
    }

    // fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
    //     let class_name: &str = slf.get_type().name()?;
    //     Ok(format!("{class_name}({})", slf.borrow().0))
    // }

    // fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
    //     match op {
    //         CompareOp::Lt => Ok(self.0 < other.0),
    //         CompareOp::Le => Ok(self.0 <= other.0),
    //         CompareOp::Eq => Ok(self.0 == other.0),
    //         CompareOp::Ne => Ok(self.0 != other.0),
    //         CompareOp::Gt => Ok(self.0 > other.0),
    //         CompareOp::Ge => Ok(self.0 >= other.0),
    //     }
    // }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
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
