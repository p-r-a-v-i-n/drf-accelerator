use pyo3::ffi;
use pyo3::prelude::*;
use pyo3::sync::GILOnceCell;
use pyo3::types::{
    PyAny, PyBool, PyDate, PyDateTime, PyDict, PyFloat, PyInt, PyList, PyString, PyTime, PyType,
};

use crate::formatters::{format_date, format_datetime, format_time};

static UUID_TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();
static DECIMAL_TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();

static STRING_TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();
static INT_TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();
static BOOL_TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();
static FLOAT_TYPE: GILOnceCell<Py<PyType>> = GILOnceCell::new();

enum FieldType {
    Simple,
    Method { func: PyObject },
    Dotted(Vec<Py<PyString>>),
    Nested { method: PyObject },
}

struct Field {
    out_name: Py<PyString>,
    src_attr: Py<PyString>,
    ftype: FieldType,
    // Cached `field.to_representation` for non-fast-path conversions.
    to_repr: Option<PyObject>,
}

#[pyclass]
pub struct FastSerializer {
    fields: Vec<Field>,
}

#[pymethods]
impl FastSerializer {
    #[new]
    pub fn new(py: Python<'_>, fields: Vec<(String, String, String, PyObject)>) -> PyResult<Self> {
        let mut cached_fields = Vec::with_capacity(fields.len());
        let to_repr = pyo3::intern!(py, "to_representation");

        for (out, src, ftype, fobj) in fields {
            let out_py = PyString::new(py, &out).unbind();
            let src_py = PyString::new(py, &src).unbind();

            let (field_type, cached_to_repr) = match ftype.as_str() {
                "method" => (FieldType::Method { func: fobj }, None),

                "dotted" => {
                    let parts = src
                        .split('.')
                        .map(|p| PyString::new(py, p).unbind())
                        .collect();
                    let cached = if fobj.is_none(py) {
                        None
                    } else {
                        Some(fobj.getattr(py, to_repr)?)
                    };
                    (FieldType::Dotted(parts), cached)
                }

                "nested" => {
                    let method = fobj.getattr(py, to_repr)?;
                    (FieldType::Nested { method }, None)
                }

                _ => {
                    let cached = if fobj.is_none(py) {
                        None
                    } else {
                        Some(fobj.getattr(py, to_repr)?)
                    };
                    (FieldType::Simple, cached)
                }
            };

            cached_fields.push(Field {
                out_name: out_py,
                src_attr: src_py,
                ftype: field_type,
                to_repr: cached_to_repr,
            });
        }

        Ok(Self {
            fields: cached_fields,
        })
    }

    pub fn serialize(&self, py: Python<'_>, instances: &Bound<'_, PyAny>) -> PyResult<Py<PyList>> {
        let __dict__ = pyo3::intern!(py, "__dict__");
        let uuid_cls = UUID_TYPE.get_or_init(py, || {
            py.import("uuid")
                .and_then(|m| m.getattr("UUID"))
                .and_then(|c| c.extract())
                .unwrap()
        });

        let decimal_cls = DECIMAL_TYPE.get_or_init(py, || {
            py.import("decimal")
                .and_then(|m| m.getattr("Decimal"))
                .and_then(|c| c.extract())
                .unwrap()
        });

        let str_cls = STRING_TYPE.get_or_init(py, || py.get_type::<PyString>().into());
        let int_cls = INT_TYPE.get_or_init(py, || py.get_type::<PyInt>().into());
        let bool_cls = BOOL_TYPE.get_or_init(py, || py.get_type::<PyBool>().into());
        let float_cls = FLOAT_TYPE.get_or_init(py, || py.get_type::<PyFloat>().into());

        let uuid_cls = uuid_cls.bind(py);
        let decimal_cls = decimal_cls.bind(py);

        let str_cls = str_cls.bind(py);
        let int_cls = int_cls.bind(py);
        let bool_cls = bool_cls.bind(py);
        let float_cls = float_cls.bind(py);

        let results = PyList::empty(py);

        for instance in instances.try_iter()? {
            let instance = instance?;

            let dict_ptr = unsafe { ffi::_PyDict_NewPresized(self.fields.len() as isize) };

            if dict_ptr.is_null() {
                return Err(pyo3::exceptions::PyMemoryError::new_err(
                    "Failed to allocate dict",
                ));
            }

            let dict_any: Bound<'_, PyAny> = unsafe { Bound::from_owned_ptr_or_err(py, dict_ptr)? };
            let dict: Bound<'_, PyDict> = dict_any.downcast_into()?;

            let obj_dict_any = instance.getattr(__dict__).ok();
            let obj_dict = obj_dict_any
                .as_ref()
                .and_then(|d| d.downcast::<PyDict>().ok());

            for field in &self.fields {
                let mut value: Bound<PyAny>;

                match &field.ftype {
                    FieldType::Simple => {
                        let attr = field.src_attr.bind(py);

                        value = if let Some(d) = obj_dict {
                            match d.get_item(attr)? {
                                Some(v) => v,
                                None => instance.getattr(attr)?,
                            }
                        } else {
                            instance.getattr(attr)?
                        };
                    }

                    FieldType::Dotted(parts) => {
                        value = instance.clone();

                        for part in parts {
                            let part = part.bind(py);

                            let maybe_dict_any = value.getattr(__dict__).ok();
                            let maybe_dict = maybe_dict_any
                                .as_ref()
                                .and_then(|d| d.downcast::<PyDict>().ok());

                            value = if let Some(d) = maybe_dict {
                                match d.get_item(part)? {
                                    Some(v) => v,
                                    None => value.getattr(part)?,
                                }
                            } else {
                                value.getattr(part)?
                            };

                            if value.is_none() {
                                break;
                            }
                        }
                    }

                    FieldType::Method { func } => {
                        value = func.call1(py, (&instance,))?.into_bound(py);
                    }

                    FieldType::Nested { method } => {
                        let attr = instance.getattr(field.src_attr.bind(py))?;

                        if attr.is_none() {
                            dict.set_item(&field.out_name, py.None())?;
                            continue;
                        }

                        // A nested serializer/field should return a Python representation already.
                        dict.set_item(&field.out_name, method.call1(py, (attr,))?)?;
                        continue;
                    }
                }

                if value.is_none() {
                    dict.set_item(&field.out_name, py.None())?;
                    continue;
                }

                let val_type = value.get_type();

                if val_type.is(str_cls)
                    || val_type.is(int_cls)
                    || val_type.is(bool_cls)
                    || val_type.is(float_cls)
                {
                    dict.set_item(&field.out_name, value)?;
                    continue;
                }

                // UUID/Decimal occur frequently in DRF payloads; checking them before datetime/date/time
                // avoids extra type checks on those fields.
                if value.as_any().is_instance(uuid_cls)?
                    || value.as_any().is_instance(decimal_cls)?
                {
                    dict.set_item(&field.out_name, value.str()?)?;
                    continue;
                }

                if value.is_instance_of::<PyDateTime>() {
                    let dt = value.downcast::<PyDateTime>()?;
                    dict.set_item(&field.out_name, format_datetime(dt)?)?;
                    continue;
                }

                if value.is_instance_of::<PyDate>() {
                    let d = value.downcast::<PyDate>()?;
                    dict.set_item(&field.out_name, format_date(d))?;
                    continue;
                }

                if value.is_instance_of::<PyTime>() {
                    let t = value.downcast::<PyTime>()?;
                    dict.set_item(&field.out_name, format_time(t))?;
                    continue;
                }

                let Some(to_repr) = field.to_repr.as_ref() else {
                    return Err(pyo3::exceptions::PyTypeError::new_err("unsupported type"));
                };

                dict.set_item(&field.out_name, to_repr.call1(py, (value,))?)?;
            }

            results.append(dict)?;
        }

        Ok(results.into())
    }
}
