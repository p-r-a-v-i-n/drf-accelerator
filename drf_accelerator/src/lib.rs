use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

#[pyclass]
struct FastSerializer {
    // List of (output_name, source_attr)
    fields: Vec<(String, String)>,
}

#[pymethods]
impl FastSerializer {
    #[new]
    fn new(fields: Vec<(String, String)>) -> Self {
        FastSerializer { fields }
    }

    fn serialize(&self, py: Python<'_>, instances: &Bound<'_, PyAny>) -> PyResult<Py<PyList>> {
        let results = PyList::empty(py);

        for instance in instances.try_iter()? {
            let instance = instance?;
            let dict = PyDict::new(py);

            for (output_name, source_attr) in &self.fields {
                let val_obj = instance.getattr(source_attr.as_str())?;

                if val_obj.is_none()
                    || val_obj.is_instance_of::<pyo3::types::PyString>()
                    || val_obj.is_instance_of::<pyo3::types::PyInt>()
                    || val_obj.is_instance_of::<pyo3::types::PyFloat>()
                    || val_obj.is_instance_of::<pyo3::types::PyBool>()
                {
                    dict.set_item(output_name, val_obj)?;
                } else {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        format!(
                            "FastSerializer: Field '{}' (source: '{}') returned unsupported type: {}. Only primitives (int, float, bool, str, None) are supported.",
                            output_name, source_attr, val_obj.get_type()
                        )
                    ));
                }
            }
            results.append(dict)?;
        }

        Ok(results.into())
    }
}

#[pymodule]
fn drf_accelerator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FastSerializer>()?;
    Ok(())
}
