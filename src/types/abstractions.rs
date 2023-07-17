use pyo3::prelude::*;
use pyo3::types::{PyCFunction, PyDict, PyLong, PyTuple, PyType};

use itertools::Itertools;

use once_cell::sync::Lazy;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::RwLock;

static TYPE_CACHE: Lazy<RwLock<HashMap<TypeCacheKey, Py<PyType>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Clone)]
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
                .collect::<PyResult<Vec<usize>>>()
                .unwrap()
                .hash(state);
        })
    }
}

#[pyclass(module = "_cerialize", name = "Shaped", subclass)]
#[derive(Default)]
pub struct PyShaped();

impl PyShaped {
    fn derived_packed_size(cls: &PyType) -> PyResult<usize> {
        let base_cls = cls.getattr("__origin__")?;
        Ok(base_cls
            .call_method1("__packed_size__", ())?
            .extract::<usize>()?
            * cls
                .getattr("_SHAPE")?
                .downcast::<PyTuple>()?
                .into_iter()
                .fold(1_usize, |prod, val| prod * val.extract::<usize>().unwrap()))
    }

    fn wrap_function<F>(py: Python<'_>, func: F) -> PyResult<Py<PyAny>>
    where
        F: Fn(&PyTuple, Option<&PyDict>) -> PyResult<Py<PyAny>> + Sync + Send + 'static,
    {
        // A simple function wrapper
        // This is necessary so that you can take functions which are defined in rust and assign them to dynamically created classes with the correct arguments
        let wrapper_fn: Py<PyAny> = PyModule::from_code(
            py,
            // TODO: This currently only supports functions that are meant to be class methods. It'd be nice to support other function types as well
            "
def wrapper(fn):
    def _cerialize_function_wrapper(*args, **kwargs):
        return fn(*args, **kwargs)
    
    return classmethod(_cerialize_function_wrapper)
            ",
            "abstractions.rs",
            "",
        )?
        .getattr("wrapper")?
        .into();

        // Create a builtin python function from the function being called
        let fn_arg = PyCFunction::new_closure(
            py,
            None,
            None,
            move |args: &PyTuple, kwargs: Option<&PyDict>| -> PyResult<Py<PyAny>> {
                func(args, kwargs)
            },
        )?;

        wrapper_fn.call1(py, (fn_arg,))
    }
}

#[pymethods]
impl PyShaped {
    #[new]
    pub fn new() -> Self {
        PyShaped::default()
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
                .collect::<Vec<_>>(),
            (false, true) => {
                vec![shape_arg.extract::<usize>()]
            }
            _ => unreachable!("Uuh... What happened?"),
        }
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

        let cache_key = TypeCacheKey(
            cls.into(),
            Into::<Py<PyTuple>>::into(PyTuple::new(py, &shape)),
        );

        // Cache the generated type to avoid issues with overwriting attributes
        if !TYPE_CACHE.read().unwrap().contains_key(&cache_key) {
            let types = PyModule::import(py, "types").unwrap();
            let class_name = format!("{}[{}]", cls.name().unwrap(), shape.iter().format(","));
            let packed_size_fn = Self::wrap_function(
                py,
                |args: &PyTuple, _kwargs: Option<&PyDict>| -> PyResult<_> {
                    let cls = args.get_item(0)?.downcast::<PyType>()?;
                    Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                        Ok(Self::derived_packed_size(cls)?.to_object(py))
                    })
                },
            )?;
            let kwds: HashMap<&str, &pyo3::PyAny> = HashMap::from_iter([
                ("module", cls.getattr("__module__")?),
                ("origin", cls.into()),
                ("shape", PyTuple::new(py, &shape).into()),
                ("packed_size_fn", packed_size_fn.as_ref(py)),
            ]);

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
    #[pyo3(signature = (*args, module, origin, packed_size_fn, shape = None, **kwargs))]
    fn __init_subclass__(
        cls: &PyType,
        args: &PyTuple,
        module: &PyAny,
        origin: &PyAny,
        packed_size_fn: &PyAny,
        shape: Option<&PyTuple>,
        kwargs: Option<&PyDict>,
    ) -> PyResult<()> {
        cls.py_super()?
            .call_method("__init_subclass__", args, kwargs)?;

        packed_size_fn.setattr("__name__", "__packed_size__")?;
        // This isn't working at the moment
        // It may be due to the fact that this is a kinda bad way of doing it
        // Either that or there is a bug in Pyo3
        packed_size_fn.setattr(
            "__qualname__",
            format!(
                "{}.__packed_size__",
                cls.getattr("__qualname__")?.extract::<String>()?
            ),
        )?;

        cls.setattr("__module__", module)?;
        cls.setattr("__origin__", origin)?;
        cls.setattr("__packed_size__", packed_size_fn)?;
        cls.setattr("_SHAPE", shape)?;
        Ok(())
    }
}
