use super::{Endianness, NativeEndian, PyShaped};
use half::f16;
use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyLong, PyType};
use std::cell::RefCell;

/// Converts a slice to an array of the specified size
fn buffer_alias<const N: usize>(buffer: &[u8]) -> &[u8; N] {
    buffer.try_into().expect("slice with incorrect length")
}

#[pyclass(module = "_cerialize", name = "boolean", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<bool>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, bool::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, bool::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
        }
    }

    fn __repr__(slf: &PyCell<Self>) -> PyResult<String> {
        let class_name: &str = slf.get_type().name()?;
        let value = slf.borrow().value();
        Ok(format!("{class_name}({})", value))
    }

    fn __richcmp__(&self, other: &Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Lt => Ok(!self.value() & other.value()),
            CompareOp::Le => Ok(self.value() <= other.value()),
            CompareOp::Eq => Ok(self.value() == other.value()),
            CompareOp::Ne => Ok(self.value() != other.value()),
            CompareOp::Gt => Ok(self.value() & !other.value()),
            CompareOp::Ge => Ok(self.value() >= other.value()),
        }
    }

    #[classmethod]
    fn __packed_size__(_cls: &PyType) -> PyResult<usize> {
        Ok(Self::PACKED_SIZE)
    }
}

#[pyclass(module = "_cerialize", name = "i8", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<i8>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, i8::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, i8::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "i16", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<i16>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, i16::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, i16::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "i32", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<i32>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, i32::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, i32::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "i64", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<i64>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, 0)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, 0)),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "i128", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<i128>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, i128::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, i128::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "u8", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<u8>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, u8::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, u8::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "u16", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<u16>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, u16::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, u16::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "u32", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<u32>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, u32::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, u32::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "u64", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<u64>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, u64::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, u64::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "u128", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<u128>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, u128::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, u128::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "f16", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = f16::from_f32(literal.extract::<f32>().unwrap());
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, f16::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, f16::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "f32", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<f32>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, f32::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, f32::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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

#[pyclass(module = "_cerialize", name = "f64", subclass, weakref, extends=PyShaped)]
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
    fn new(value: Option<&PyAny>, endianness: Option<Endianness>) -> (Self, PyShaped) {
        let endianness = endianness.unwrap_or(Endianness::Native(NativeEndian()));
        match value {
            Some(value) => {
                if let Ok(literal) = value.downcast::<PyLong>() {
                    let literal = literal.extract::<f64>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, literal)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else if let Ok(buffer) = value.downcast::<PyBytes>() {
                    let buffer = buffer.extract::<&[u8]>().unwrap();
                    (
                        Self {
                            buffer: RefCell::new(*buffer_alias::<{ Self::PACKED_SIZE }>(buffer)),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                } else {
                    (
                        Self {
                            buffer: RefCell::new(Self::to_bytes(&endianness, f64::default())),
                            endianness,
                        },
                        PyShaped::new(),
                    )
                }
            }
            None => (
                Self {
                    buffer: RefCell::new(Self::to_bytes(&endianness, f64::default())),
                    endianness,
                },
                PyShaped::new(),
            ),
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
