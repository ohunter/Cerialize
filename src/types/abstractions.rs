use pyo3::prelude::*;
use pyo3::types::{PyDict, PyLong, PyTuple, PyType};

use itertools::Itertools;

use once_cell::sync::Lazy;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::RwLock;

static TYPE_CACHE: Lazy<RwLock<HashMap<TypeCacheKey, Py<PyType>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Clone)]
struct TypeCacheKey(Py<PyType>, Py<PyTuple>);

impl PartialEq for TypeCacheKey {
    fn eq(&self, other: &Self) -> bool {
        let attributes = &["__qualname__"];

        Python::with_gil(|py| {
            let same_type = attributes.iter().all(|&attr| {
                self.0
                    .getattr(py, attr)
                    .unwrap()
                    .extract::<&str>(py)
                    .unwrap()
                    == other
                        .0
                        .getattr(py, attr)
                        .unwrap()
                        .extract::<&str>(py)
                        .unwrap()
            });

            let same_tuple = self
                .1
                .as_ref(py)
                .iter()
                .zip_longest(other.1.as_ref(py).iter())
                .all(|val| match val {
                    itertools::EitherOrBoth::Both(left, right) => {
                        left.extract::<usize>().unwrap() == right.extract::<usize>().unwrap()
                    }
                    _ => false,
                });
            same_type && same_tuple
        })
    }
}

impl Eq for TypeCacheKey {}

impl Hash for TypeCacheKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Python::with_gil(|py| {
            self.0
                .getattr(py, "__qualname__")
                .unwrap()
                .extract::<String>(py)
                .unwrap()
                .hash(state);
            self.1
                .as_ref(py)
                .iter()
                .map(|val| val.extract::<usize>())
                .into_iter()
                .collect::<PyResult<Vec<usize>>>()
                .unwrap()
                .hash(state);
        })
    }
}

#[pyclass(module = "_cerialize", name = "Shaped", subclass)]
pub struct PyShaped();

#[pymethods]
impl PyShaped {
    #[new]
    pub fn new() -> Self {
        PyShaped()
    }

    #[pyo3(signature = (*args))]
    #[classmethod]
    fn __class_getitem__(cls: &PyType, py: Python<'_>, args: &PyTuple) -> PyResult<Py<PyType>> {
        // The shape is the 0th element of the tuple
        // If there is only one dimension then args[0] is just an int
        // If there are more than one dimensions then args[0] is a tuple of ints
        let shape_arg = args.get_item(0)?;

        let shape = match (
            shape_arg.is_instance_of::<PyTuple>(),
            shape_arg.is_instance_of::<PyLong>(),
        ) {
            (true, false) => shape_arg
                .downcast::<PyTuple>()?
                .into_iter()
                .map(|value| value.extract::<usize>())
                .into_iter()
                .collect::<Vec<_>>(),
            (false, true) => {
                vec![shape_arg.extract::<usize>()]
            }
            _ => unreachable!("Uuh... What happened?"),
        }
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

        println!("__class_getitem__(cls={}, args={})", cls, args);

        let cache_key = TypeCacheKey(
            cls.into(),
            Into::<Py<PyTuple>>::into(PyTuple::new(py, &shape)),
        );

        // Cache the generated type to avoid issues with overwriting attributes
        if !TYPE_CACHE.read().unwrap().contains_key(&cache_key) {
            let types = PyModule::import(py, "types").unwrap();
            let class_name = format!("{}[{}]", cls.name().unwrap(), shape.iter().format(","));
            let kwds: HashMap<&str, &pyo3::PyAny> =
                HashMap::from_iter([("shape", PyTuple::new(py, &shape).into())]);

            TYPE_CACHE.write().unwrap().insert(
                cache_key.clone(),
                Into::<Py<PyType>>::into(
                    types
                        .getattr("new_class")
                        .unwrap()
                        .call1((class_name, (cls,), kwds))
                        .unwrap()
                        .downcast::<PyType>()
                        .unwrap(),
                ),
            );
        }

        let cache = TYPE_CACHE.read().unwrap();
        let new_type = cache.get(&cache_key).unwrap();
        Ok(new_type.clone())
    }

    #[classmethod]
    #[pyo3(signature = (*args, shape = None, **kwargs))]
    fn __init_subclass__(
        cls: &PyType,
        args: &PyTuple,
        shape: Option<&PyTuple>,
        kwargs: Option<&PyDict>,
    ) -> PyResult<()> {
        println!(
            "__init_subclass__(cls={}, args={:?}, shape={:?}, kwargs={:?})",
            cls, args, shape, kwargs
        );
        cls.py_super()?
            .call_method("__init_subclass__", args, kwargs)?;
        cls.setattr("_SHAPE", shape)?;
        Ok(())
    }
}
