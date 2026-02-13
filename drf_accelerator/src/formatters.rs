use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyDate, PyTime, PyDateAccess, PyTimeAccess};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Timelike};

#[inline]
pub fn format_datetime(dt: &Bound<'_, PyDateTime>) -> PyResult<String> {
    if let Ok(aware_dt) = dt.extract::<DateTime<FixedOffset>>() {
        let mut s = aware_dt.format("%Y-%m-%dT%H:%M:%S").to_string();
        
        // Handle microseconds
        let micro = aware_dt.nanosecond() / 1000;
        if micro > 0 {
            s.push_str(&format!(".{:06}", micro));
        }
        
        // Handle timezone suffix
        let offset = *aware_dt.offset();
        if offset.local_minus_utc() == 0 {
            s.push('Z');
        } else {
            s.push_str(&offset.to_string());
        }
        Ok(s)
    } else {
        // Fallback for naive datetime or if extraction fails
        let naive = dt.extract::<NaiveDateTime>()?;
        let mut s = naive.format("%Y-%m-%dT%H:%M:%S").to_string();
        let micro = naive.nanosecond() / 1000;
        if micro > 0 {
            s.push_str(&format!(".{:06}", micro));
        }
        Ok(s)
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
