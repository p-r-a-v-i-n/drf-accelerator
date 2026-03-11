mod formatters;
mod serializer;

use crate::serializer::FastSerializer;
use pyo3::prelude::*;

#[pymodule]
fn drf_accelerator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FastSerializer>()?;
    Ok(())
}
