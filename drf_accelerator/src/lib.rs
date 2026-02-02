use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyDateTime, PyDate, PyTime};
use pyo3::types::{PyDateAccess, PyTimeAccess, PyTzInfoAccess};

#[pyclass]
struct FastSerializer {
    // List of (output_name, source_attr)
    fields: Vec<(String, String)>,
}

/// Format a datetime as ISO 8601 string with timezone.
/// Uses PyO3's native accessors - NO Python getattr calls!
/// Output: "YYYY-MM-DDTHH:MM:SS.ffffff+HH:MM" or "YYYY-MM-DDTHH:MM:SS.ffffffZ"
#[inline]
fn format_datetime(dt: &Bound<'_, PyDateTime>) -> PyResult<String> {
    // Direct C struct access via PyDateAccess and PyTimeAccess traits
    let year = dt.get_year();
    let month = dt.get_month();
    let day = dt.get_day();
    let hour = dt.get_hour();
    let minute = dt.get_minute();
    let second = dt.get_second();
    let microsecond = dt.get_microsecond();

    // Handle timezone
    let tz_suffix = if let Some(tzinfo) = dt.get_tzinfo() {
        // Get UTC offset - this requires calling Python method
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
/// Uses PyO3's native accessors - NO Python getattr calls!
/// Output: "YYYY-MM-DD"
#[inline]
fn format_date(d: &Bound<'_, PyDate>) -> String {
    // Direct C struct access via PyDateAccess trait
    let year = d.get_year();
    let month = d.get_month();
    let day = d.get_day();
    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// Format a time as ISO 8601 string.
/// Uses PyO3's native accessors - NO Python getattr calls!
/// Output: "HH:MM:SS.ffffff" or "HH:MM:SS"
#[inline]
fn format_time(t: &Bound<'_, PyTime>) -> String {
    // Direct C struct access via PyTimeAccess trait
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
                } else if val_obj.is_instance_of::<PyDateTime>() {
                    // DateTime must be checked before Date (datetime is subclass of date)
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
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        format!(
                            "FastSerializer: Field '{}' (source: '{}') returned unsupported type: {}. Supported: int, float, bool, str, None, datetime, date, time.",
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

