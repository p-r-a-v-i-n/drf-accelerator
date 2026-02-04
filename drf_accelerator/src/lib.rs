use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList,PyType, PyDateTime, PyDate, PyTime, PyDateAccess, PyTimeAccess, PyTzInfoAccess};

#[pyclass]
struct FastSerializer {
    // List of (output_name, source_attr)
    fields: Vec<(String, String)>,
    // Cached type references for specialized types
    uuid_type: Option<Py<PyType>>,
    decimal_type: Option<Py<PyType>>,
}

/// Format a datetime as ISO 8601 string with timezone.
#[inline]
fn format_datetime(dt: &Bound<'_, PyDateTime>) -> PyResult<String> {
    let year = dt.get_year();
    let month = dt.get_month();
    let day = dt.get_day();
    let hour = dt.get_hour();
    let minute = dt.get_minute();
    let second = dt.get_second();
    let microsecond = dt.get_microsecond();

    let tz_suffix = if let Some(tzinfo) = dt.get_tzinfo() {
        let offset = tzinfo.call_method1("utcoffset", (dt,))?;
        if offset.is_none() {
            String::new()
        } else {
            let total_seconds = offset.call_method0("total_seconds")?.extract::<f64>()?;
            let total_secs = total_seconds as i32;
            if total_secs == 0 {
                "Z".to_string()
            } else {
                let sign = if total_secs >= 0 { '+' } else { '-' };
                let abs_secs = total_secs.abs();
                let hours = abs_secs / 3600;
                let minutes = (abs_secs % 3600) / 60;
                format!("{}{:02}:{:02}", sign, hours, minutes)
            }
        }
    } else {
        String::new()
    };

    if microsecond > 0 {
        Ok(format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}{}",
            year, month, day, hour, minute, second, microsecond, tz_suffix
        ))
    } else {
        Ok(format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}{}",
            year, month, day, hour, minute, second, tz_suffix
        ))
    }
}

/// Format a date as ISO 8601 string.
#[inline]
fn format_date(d: &Bound<'_, PyDate>) -> String {
    let year = d.get_year();
    let month = d.get_month();
    let day = d.get_day();
    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// Format a time as ISO 8601 string.
#[inline]
fn format_time(t: &Bound<'_, PyTime>) -> String {
    let hour = t.get_hour();
    let minute = t.get_minute();
    let second = t.get_second();
    let microsecond = t.get_microsecond();

    if microsecond > 0 {
        format!("{:02}:{:02}:{:02}.{:06}", hour, minute, second, microsecond)
    } else {
        format!("{:02}:{:02}:{:02}", hour, minute, second)
    }
}

#[pymethods]
impl FastSerializer {
    #[new]
    fn new(py: Python<'_>, fields: Vec<(String, String)>) -> PyResult<Self> {
        // Cache UUID and Decimal types once at initialization
        let uuid_type = match py.import("uuid") {
            Ok(module) => {
                match module.getattr("UUID") {
                    Ok(cls) => Some(cls.downcast::<PyType>()?.clone().unbind()),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        };

        let decimal_type = match py.import("decimal") {
            Ok(module) => {
                match module.getattr("Decimal") {
                    Ok(cls) => Some(cls.downcast::<PyType>()?.clone().unbind()),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        };

        Ok(FastSerializer {
            fields,
            uuid_type,
            decimal_type,
        })
    }

    fn serialize(&self, py: Python<'_>, instances: &Bound<'_, PyAny>) -> PyResult<Py<PyList>> {
        let results = PyList::empty(py);

        for instance in instances.try_iter()? {
            let instance = instance?;
            let dict = PyDict::new(py);

            for (output_name, source_attr) in &self.fields {
                let val_obj = instance.getattr(source_attr.as_str())?;

                // Primitive check branch (fastest path for common types)
                if val_obj.is_none()
                    || val_obj.is_instance_of::<pyo3::types::PyString>()
                    || val_obj.is_instance_of::<pyo3::types::PyInt>()
                    || val_obj.is_instance_of::<pyo3::types::PyFloat>()
                    || val_obj.is_instance_of::<pyo3::types::PyBool>()
                {
                    dict.set_item(output_name, val_obj)?;
                } else if val_obj.is_instance_of::<PyDateTime>() {
                    let dt = val_obj.downcast::<PyDateTime>()?;
                    let iso_str = format_datetime(dt)?;
                    dict.set_item(output_name, iso_str)?;
                } else if val_obj.is_instance_of::<PyDate>() {
                    let d = val_obj.downcast::<PyDate>()?;
                    let iso_str = format_date(d);
                    dict.set_item(output_name, iso_str)?;
                } else if val_obj.is_instance_of::<PyTime>() {
                    let t = val_obj.downcast::<PyTime>()?;
                    let iso_str = format_time(t);
                    dict.set_item(output_name, iso_str)?;
                } else {
                    // Specialized check branch (UUID, Decimal)
                    let mut handled = false;
                    
                    if let Some(ref uuid_cls) = self.uuid_type {
                        if val_obj.is_instance(uuid_cls.bind(py))? {
                            dict.set_item(output_name, val_obj.str()?)?;
                            handled = true;
                        }
                    }

                    if !handled {
                        if let Some(ref decimal_cls) = self.decimal_type {
                            if val_obj.is_instance(decimal_cls.bind(py))? {
                                dict.set_item(output_name, val_obj.str()?)?;
                                handled = true;
                            }
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

#[pymodule]
fn drf_accelerator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FastSerializer>()?;
    Ok(())
}

