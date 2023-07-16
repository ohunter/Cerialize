use pyo3::prelude::*;
use pyo3::types::{PyLong, PyTuple, PyType, PyDict};

use itertools::Itertools;

use std::collections::HashMap;

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

        let types = PyModule::import(py, "types")?;
        let class_name = format!("{}[{}]", cls.name()?, shape.iter().format(","));
        let kwds: HashMap<&str, &pyo3::PyAny> = HashMap::from_iter([
            ("shape", PyTuple::new(py, &shape).into()),
        ]);

        let new_type = types
            .getattr("new_class")?
            .call1((class_name, (cls,), kwds))?;
        Ok(new_type.downcast::<PyType>()?.into())

    }

    #[classmethod]
    #[pyo3(signature = (*args, shape = None, **kwargs))]
    fn __init_subclass__(cls: &PyType, args: &PyTuple, shape: Option<&PyTuple>, kwargs: Option<&PyDict>) -> PyResult<()> {
        println!("__init_subclass__(cls={}, args={:?}, shape={:?}, kwargs={:?})", cls, args, shape, kwargs);
        cls.py_super()?.call_method("__init_subclass__", args, kwargs)?;
        cls.setattr("_SHAPE", shape)?;
        Ok(())
    }
}
