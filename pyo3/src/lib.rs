use ::android_bp::Block;
use ::android_bp::BluePrint;
use ::android_bp::Dict;
use ::android_bp::Value;
use std::collections::HashMap;

use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(unsendable)]
pub struct PyBlock {
    #[pyo3(get)]
    pub typ: String,
    #[pyo3(get)]
    pub entries: HashMap<String, PyValue>,
}
impl From<&Block> for PyBlock {
    fn from(block: &Block) -> Self {
                let entries = block.entries.iter().map(value_to_pyvalue).collect();
        PyBlock {
            typ: block.typ.to_owned(),
            entries,
        }
    }
}
#[pymethods]
impl PyBlock {
    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }
}

fn dict_to_py(dict: &Dict) -> HashMap<String, PyValue> {
    dict.iter().map(value_to_pyvalue).collect()
}
#[derive(Debug, Clone, FromPyObject)]
pub enum PyValue {
    String(String),
    Array(Vec<String>),
    Boolean(bool),
    Dict(HashMap<String, PyValue>),
    Ident(String),
}
impl IntoPy<Py<PyAny>> for PyValue {
    fn into_py(self, py: Python) -> Py<PyAny> {
        match self {
            PyValue::String(s) => s.into_py(py),
            PyValue::Array(a) => a.into_py(py),
            PyValue::Boolean(b) => b.into_py(py),
            PyValue::Dict(d) => d.into_py(py),
            PyValue::Ident(i) => i.into_py(py),
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass(unsendable)]
pub struct PyBluePrint {
    #[pyo3(get)]
    pub defines: HashMap<String, PyValue>,
    #[pyo3(get)]
    pub blocks: Vec<PyBlock>,
}
impl From<&BluePrint> for PyBluePrint {
    fn from(bp: &BluePrint) -> Self {
        let defines = bp.defines.iter().map(value_to_pyvalue).collect();
        let blocks = bp.blocks.iter().map(|b| PyBlock::from(b)).collect();
        PyBluePrint { defines, blocks }
    }
}
#[pymethods]
impl PyBluePrint {
    #[new]
    pub fn parse(input: String) -> PyResult<Self> {
        let bp = BluePrint::parse(&input).map_err(|e| e.to_string());
        match bp {
            Ok(bp) => Ok(PyBluePrint::from(&bp)),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e)),
        }
    }
    pub fn  __repr__(&self) -> String {
        format!("{:#?}", self)
    }
}
fn value_to_pyvalue(t: (&String, &Value)) -> (String, PyValue) {
    let (k, v) = t;
    let value = match v {
        Value::String(s) => PyValue::String(s.to_owned()),
        Value::Array(a) => PyValue::Array(a.to_owned()),
        Value::Boolean(b) => PyValue::Boolean(b.to_owned()),
        Value::Dict(d) => PyValue::Dict(dict_to_py(&d)),
        Value::Ident(i) => PyValue::Ident(i.to_owned()),
    };
    (k.to_owned(), value)
}
#[pymodule]
fn android_bp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBluePrint>()?;
    Ok(())
}

