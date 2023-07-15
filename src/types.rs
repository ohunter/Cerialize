use half::f16;
use pyo3::exceptions::PyAttributeError;
use pyo3::types::PyBytes;
use pyo3::types::{PyDict, PyLong, PyString, PyType};
use pyo3::{prelude::*, pyclass::CompareOp};
use std::cell::RefCell;

/// Converts a slice to an array of the specified size
fn buffer_alias<const N: usize>(buffer: &[u8]) -> &[u8; N] {
    buffer.try_into().expect("slice with incorrect length")
}

#[pyclass(module = "primitives", frozen, weakref)]
#[derive(Clone, Copy)]
pub struct Native();

#[pymethods]
impl Native {
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

#[pyclass(module = "primitives", frozen, weakref)]
#[derive(Clone, Copy)]
pub struct Big();

#[pymethods]
impl Big {
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

#[pyclass(module = "primitives", frozen, weakref)]
#[derive(Clone, Copy)]
pub struct Little();

#[pymethods]
impl Little {
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
    Native(Native),
    #[pyo3(transparent)]
    Big(Big),
    #[pyo3(transparent)]
    Little(Little),
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

#[pyclass(module = "primitives", name = "_cstruct", subclass, weakref)]
pub struct CStruct {
    buffer: RefCell<Vec<u8>>,
    endianness: Endianness,
}

#[pymethods]
impl CStruct {
    #[new]
    fn new(buffer: &[u8], endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        Self {
            buffer: RefCell::new(Vec::from(buffer)),
            endianness,
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

    fn _type_and_offset_of(slf: &PyCell<Self>, attr: String) -> PyResult<(&PyType, usize)> {
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
            .downcast::<PyLong>()?
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

#[pyclass(module = "primitives", name = "_bool", subclass, weakref)]
pub struct Bool {
    buffer: RefCell<[u8; std::mem::size_of::<bool>()]>,
    endianness: Endianness,
}

impl Bool {
    // type DataType = bool;

    const PACKED_SIZE: usize = std::mem::size_of::<bool>();

    fn to_bytes(_endianness: &Endianness, value: bool) -> [u8; Self::PACKED_SIZE] {
        unsafe { std::mem::transmute_copy::<bool, [u8; Self::PACKED_SIZE]>(&value) }
    }

    fn from_bytes(_endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> bool {
        unsafe { std::mem::transmute_copy::<[u8; Self::PACKED_SIZE], bool>(&buffer.borrow()) }
    }

    fn value(&self) -> bool {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Bool {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<bool>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, bool::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, bool::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_i8", subclass, weakref)]
pub struct Int8 {
    buffer: RefCell<[u8; std::mem::size_of::<i8>()]>,
    endianness: Endianness,
}

impl Int8 {
    // type DataType = i8;

    const PACKED_SIZE: usize = std::mem::size_of::<i8>();

    fn to_bytes(endianness: &Endianness, value: i8) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> i8 {
        match endianness {
            Endianness::Native(_) => i8::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => i8::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => i8::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> i8 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Int8 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<i8>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<i8>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, i8::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, i8::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_i16", subclass, weakref)]
pub struct Int16 {
    buffer: RefCell<[u8; std::mem::size_of::<i16>()]>,
    endianness: Endianness,
}

impl Int16 {
    // type DataType = i16;

    const PACKED_SIZE: usize = std::mem::size_of::<i16>();

    fn to_bytes(endianness: &Endianness, value: i16) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> i16 {
        match endianness {
            Endianness::Native(_) => i16::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => i16::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => i16::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> i16 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Int16 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<i16>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<i16>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, i16::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, i16::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_i32", subclass, weakref)]
pub struct Int32 {
    buffer: RefCell<[u8; std::mem::size_of::<i32>()]>,
    endianness: Endianness,
}

impl Int32 {
    // type DataType = i32;

    const PACKED_SIZE: usize = std::mem::size_of::<i32>();

    fn to_bytes(endianness: &Endianness, value: i32) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> i32 {
        match endianness {
            Endianness::Native(_) => i32::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => i32::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => i32::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> i32 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Int32 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<i32>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<i32>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, i32::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, i32::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_i64", subclass, weakref)]
pub struct Int64 {
    buffer: RefCell<[u8; std::mem::size_of::<i64>()]>,
    endianness: Endianness,
}

impl Int64 {
    // type DataType = i64;

    const PACKED_SIZE: usize = std::mem::size_of::<i64>();

    fn to_bytes(endianness: &Endianness, value: i64) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> i64 {
        match endianness {
            Endianness::Native(_) => i64::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => i64::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => i64::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> i64 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Int64 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<i64>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<i64>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, 0)),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, 0)),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_i128", subclass, weakref)]
pub struct Int128 {
    buffer: RefCell<[u8; std::mem::size_of::<i128>()]>,
    endianness: Endianness,
}

impl Int128 {
    // type DataType = i128;

    const PACKED_SIZE: usize = std::mem::size_of::<i128>();

    fn to_bytes(endianness: &Endianness, value: i128) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> i128 {
        match endianness {
            Endianness::Native(_) => i128::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => i128::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => i128::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> i128 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Int128 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<i128>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<i128>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, i128::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, i128::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_u8", subclass, weakref)]
pub struct Uint8 {
    buffer: RefCell<[u8; std::mem::size_of::<u8>()]>,
    endianness: Endianness,
}

impl Uint8 {
    // type DataType = u8;

    const PACKED_SIZE: usize = std::mem::size_of::<u8>();

    fn to_bytes(endianness: &Endianness, value: u8) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> u8 {
        match endianness {
            Endianness::Native(_) => u8::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => u8::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => u8::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> u8 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Uint8 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<u8>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<u8>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, u8::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, u8::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_u16", subclass, weakref)]
pub struct Uint16 {
    buffer: RefCell<[u8; std::mem::size_of::<u16>()]>,
    endianness: Endianness,
}

impl Uint16 {
    // type DataType = u16;

    const PACKED_SIZE: usize = std::mem::size_of::<u16>();

    fn to_bytes(endianness: &Endianness, value: u16) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> u16 {
        match endianness {
            Endianness::Native(_) => u16::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => u16::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => u16::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> u16 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Uint16 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<u16>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<u16>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, u16::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, u16::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_u32", subclass, weakref)]
pub struct Uint32 {
    buffer: RefCell<[u8; std::mem::size_of::<u32>()]>,
    endianness: Endianness,
}

impl Uint32 {
    // type DataType = u32;

    const PACKED_SIZE: usize = std::mem::size_of::<u32>();

    fn to_bytes(endianness: &Endianness, value: u32) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> u32 {
        match endianness {
            Endianness::Native(_) => u32::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => u32::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => u32::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> u32 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Uint32 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<u32>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<u32>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, u32::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, u32::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_u64", subclass, weakref)]
pub struct Uint64 {
    buffer: RefCell<[u8; std::mem::size_of::<u64>()]>,
    endianness: Endianness,
}

impl Uint64 {
    // type DataType = u64;

    const PACKED_SIZE: usize = std::mem::size_of::<u64>();

    fn to_bytes(endianness: &Endianness, value: u64) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> u64 {
        match endianness {
            Endianness::Native(_) => u64::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => u64::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => u64::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> u64 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Uint64 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<u64>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<u64>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, u64::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, u64::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_u128", subclass, weakref)]
pub struct Uint128 {
    buffer: RefCell<[u8; std::mem::size_of::<u128>()]>,
    endianness: Endianness,
}

impl Uint128 {
    // type DataType = u128;

    const PACKED_SIZE: usize = std::mem::size_of::<u128>();

    fn to_bytes(endianness: &Endianness, value: u128) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> u128 {
        match endianness {
            Endianness::Native(_) => u128::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => u128::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => u128::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> u128 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Uint128 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<u128>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<u128>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, u128::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, u128::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_f16", subclass, weakref)]
pub struct Float16 {
    buffer: RefCell<[u8; std::mem::size_of::<f16>()]>,
    endianness: Endianness,
}

impl Float16 {
    // type DataType = f16;

    const PACKED_SIZE: usize = std::mem::size_of::<f16>();

    fn to_bytes(endianness: &Endianness, value: f16) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> f16 {
        match endianness {
            Endianness::Native(_) => f16::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => f16::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => f16::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> f16 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Float16 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = f16::from_f32(literal.extract::<f32>().unwrap());
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<f16>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, f16::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, f16::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_f32", subclass, weakref)]
pub struct Float32 {
    buffer: RefCell<[u8; std::mem::size_of::<f32>()]>,
    endianness: Endianness,
}

impl Float32 {
    // type DataType = f32;

    const PACKED_SIZE: usize = std::mem::size_of::<f32>();

    fn to_bytes(endianness: &Endianness, value: f32) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> f32 {
        match endianness {
            Endianness::Native(_) => f32::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => f32::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => f32::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> f32 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Float32 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<f32>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<f32>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, f32::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, f32::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "primitives", name = "_f64", subclass, weakref)]
pub struct Float64 {
    buffer: RefCell<[u8; std::mem::size_of::<f64>()]>,
    endianness: Endianness,
}

impl Float64 {
    // type DataType = f64;

    const PACKED_SIZE: usize = std::mem::size_of::<f64>();

    fn to_bytes(endianness: &Endianness, value: f64) -> [u8; Self::PACKED_SIZE] {
        match endianness {
            Endianness::Native(_) => value.to_ne_bytes(),
            Endianness::Big(_) => value.to_be_bytes(),
            Endianness::Little(_) => value.to_le_bytes(),
        }
    }

    fn from_bytes(endianness: &Endianness, buffer: &RefCell<[u8; Self::PACKED_SIZE]>) -> f64 {
        match endianness {
            Endianness::Native(_) => f64::from_ne_bytes(*buffer.borrow()),
            Endianness::Big(_) => f64::from_be_bytes(*buffer.borrow()),
            Endianness::Little(_) => f64::from_le_bytes(*buffer.borrow()),
        }
    }

    fn value(&self) -> f64 {
        Self::from_bytes(&self.endianness, &self.buffer)
    }
}

#[pymethods]
impl Float64 {
    #[new]
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> Self {
        let endianness = endianness.unwrap_or(Endianness::Native(Native()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<f64>().unwrap();
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                        endianness,
                    }
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    Self {
                        buffer: RefCell::new(*buffer_alias::<{ std::mem::size_of::<f64>() }>(
                            buffer,
                        )),
                        endianness,
                    }
                } else {
                    Self {
                        buffer: RefCell::new(Self::to_bytes(&endianness, f64::default())),
                        endianness,
                    }
                }
            }
            None => Self {
                buffer: RefCell::new(Self::to_bytes(&endianness, f64::default())),
                endianness,
            },
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(self.value() < other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() > other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}
