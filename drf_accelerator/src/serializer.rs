use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyType, PyDateTime, PyDate, PyTime, PyString, PyInt, PyFloat, PyBool};
use pyo3::sync::GILOnceCell;
use crate::formatters::{format_datetime, format_date, format_time};

static UUID_TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();
static DECIMAL_TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();

#[pyclass]
pub struct FastSerializer {
    fields: Vec<(Py<PyString>, Py<PyString>, String, String)>,
}

#[pymethods]
impl FastSerializer {
    #[new]
    pub fn new(py: Python<'_>, fields: Vec<(String, String)>) -> PyResult<Self> {
        let mut cached_fields = Vec::with_capacity(fields.len());
        for (out, src) in fields {
            let out_py = PyString::new(py, out.as_str()).unbind();
            let src_py = PyString::new(py, src.as_str()).unbind();
            cached_fields.push((out_py, src_py, out, src));
        }

        Ok(FastSerializer {
            fields: cached_fields,
        })
    }

    pub fn serialize(&self, py: Python<'_>, instances: &Bound<'_, PyAny>) -> PyResult<Py<PyList>> {
        let results = PyList::empty(py);

        for instance in instances.try_iter()? {
            let instance = instance?;
            let dict = PyDict::new(py);

            for (output_name_py, source_attr_py, output_name, source_attr) in &self.fields {
                let val_obj = instance.getattr(source_attr_py.bind(py))?;

                if val_obj.is_instance_of::<PyString>()
                    || val_obj.is_instance_of::<PyInt>()
                    || val_obj.is_none()
                    || val_obj.is_instance_of::<PyBool>()
                    || val_obj.is_instance_of::<PyFloat>()
                {
                    dict.set_item(output_name_py, val_obj)?;
                } else if val_obj.is_instance_of::<PyDateTime>() {
                    let dt = val_obj.downcast::<PyDateTime>()?;
                    let iso_str = format_datetime(dt)?;
                    dict.set_item(output_name_py, iso_str)?;
                } else if val_obj.is_instance_of::<PyDate>() {
                    let d = val_obj.downcast::<PyDate>()?;
                    let iso_str = format_date(d);
                    dict.set_item(output_name_py, iso_str)?;
                } else if val_obj.is_instance_of::<PyTime>() {
                    let t = val_obj.downcast::<PyTime>()?;
                    let iso_str = format_time(t);
                    dict.set_item(output_name_py, iso_str)?;
                } else {
                    let mut handled = false;
                    
                    let uuid_cls = UUID_TYPE.get_or_init(py, || {
                        py.import("uuid").and_then(|m| m.getattr("UUID")).and_then(|c| c.extract::<Py<PyType>>()).unwrap()
                    });
                    if val_obj.as_any().is_instance(uuid_cls.bind(py))? {
                        dict.set_item(output_name_py, val_obj.str()?)?;
                        handled = true;
                    }

                    if !handled {
                        let decimal_cls = DECIMAL_TYPE.get_or_init(py, || {
                            py.import("decimal").and_then(|m| m.getattr("Decimal")).and_then(|c| c.extract::<Py<PyType>>()).unwrap()
                        });
                        if val_obj.as_any().is_instance(decimal_cls.bind(py))? {
                            dict.set_item(output_name_py, val_obj.str()?)?;
                            handled = true;
                        }
                    }

                    if !handled {
                        return Err(pyo3::exceptions::PyTypeError::new_err(
                            format!(
                                "FastSerializer: Field '{}' (source: '{}') returned unsupported type: {}. Supported: primitives, datetime, date, time, uuid, decimal.",
                                output_name, source_attr, val_obj.get_type()
                            )
                        ));
                    }
                }
            }
            results.append(dict)?;
        }

        Ok(results.into())
    }
}
