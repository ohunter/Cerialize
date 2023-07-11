use pyo3::prelude::*;
use half::f16;

#[pyclass(name="_i8", subclass, weakref)]
pub struct Int8 {
    _inner: i8
}

#[pymethods]
impl Int8 {
    #[new]
    fn new(value: i8) -> Self {
        Self{_inner: value}
    }
}

#[pyclass(name="_i16", subclass, weakref)]
pub struct Int16 {
    _inner: i16
}

#[pymethods]
impl Int16 {
    #[new]
    fn new(value: i16) -> Self {
        Self{_inner: value}
    }
}

#[pyclass(name="_i32", subclass, weakref)]
pub struct Int32 {
    _inner: i32
}

#[pymethods]
impl Int32 {
    #[new]
    fn new(value: i32) -> Self {
        Self{_inner: value}
    }
}

#[pyclass(name="_i64", subclass, weakref)]
pub struct Int64 {
    _inner: i64
}

#[pymethods]
impl Int64 {
    #[new]
    fn new(value: i64) -> Self {
        Self{_inner: value}
    }
}

#[pyclass(name="_u8", subclass, weakref)]
pub struct Uint8 {
    _inner: u8
}

#[pymethods]
impl Uint8 {
    #[new]
    fn new(value: u8) -> Self {
        Self{_inner: value}
    }
}

#[pyclass(name="_u16", subclass, weakref)]
pub struct Uint16 {
    _inner: u16
}

#[pymethods]
impl Uint16 {
    #[new]
    fn new(value: u16) -> Self {
        Self{_inner: value}
    }
}

#[pyclass(name="_u32", subclass, weakref)]
pub struct Uint32 {
    _inner: u32
}

#[pymethods]
impl Uint32 {
    #[new]
    fn new(value: u32) -> Self {
        Self{_inner: value}
    }
}

#[pyclass(name="_u64", subclass, weakref)]
pub struct Uint64 {
    _inner: u64
}

#[pymethods]
impl Uint64 {
    #[new]
    fn new(value: u64) -> Self {
        Self{_inner: value}
    }
}

#[pyclass(name="_f16", subclass, weakref)]
pub struct Float16 {
    _inner: f16
}

#[pymethods]
impl Float16 {
    #[new]
    fn new(value: f32) -> Self {
        Self{_inner: f16::from_f32(value)}
    }
}

#[pyclass(name="_f32", subclass, weakref)]
pub struct Float32 {
    _inner: f32
}

#[pymethods]
impl Float32 {
    #[new]
    fn new(value: f32) -> Self {
        Self{_inner: value}
    }
}

#[pyclass(name="_f64", subclass, weakref)]
pub struct Float64 {
    _inner: f64
}

#[pymethods]
impl Float64 {
    #[new]
    fn new(value: f64) -> Self {
        Self{_inner: value}
    }
}