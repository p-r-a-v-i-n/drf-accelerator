use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDate, PyTime, PyDateAccess, PyTimeAccess, PyTzInfoAccess};

#[inline]
pub fn format_datetime(dt: &Bound<'_, PyDateTime>) -> PyResult<String> {
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
            let total_seconds = offset.call_method0("total_seconds")?.extract::<f64>()? as i32;
            if total_seconds == 0 {
                "Z".to_string()
            } else {
                let sign = if total_seconds >= 0 { '+' } else { '-' };
                let abs_secs = total_seconds.abs();
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

#[inline]
pub fn format_date(d: &Bound<'_, PyDate>) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        d.get_year(),
        d.get_month(),
        d.get_day()
    )
}

#[inline]
pub fn format_time(t: &Bound<'_, PyTime>) -> String {
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
