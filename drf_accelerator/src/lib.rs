mod formatters;
mod serializer;

use pyo3::prelude::*;
use crate::serializer::FastSerializer;

#[pymodule]
fn drf_accelerator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FastSerializer>()?;
    Ok(())
}


