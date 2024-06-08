use ::android_bp::Module as RsModule;
use ::android_bp::BluePrint as RsBluePrint;
use ::android_bp::Map as RsMap;
use ::android_bp::Value as RsValue;
use std::collections::HashMap;

use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass(unsendable)]
pub struct Module {
    #[pyo3(get, name="__type__")]
    pub typ: String,
    #[pyo3(get, name="__dict__")]
    pub entries: HashMap<String, Value>,
}
impl From<&RsModule> for Module {
    fn from(module: &RsModule) -> Self {
        let entries = module.entries.iter().map(value_to_pyvalue).collect();
        Module {
            typ: module.typ.to_owned(),
            entries,
        }
    }
}
#[pymethods]
impl Module {
    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }
    fn __getattr__(&self, attr: &str) -> Option<Value> {
        self.entries.get(attr).cloned()
    }
}

fn map_to_py(dict: &RsMap) -> HashMap<String, Value> {
    dict.iter().map(value_to_pyvalue).collect()
}
#[derive(Debug, Clone, FromPyObject)]
pub enum Value {
    String(String),
    Array(Vec<Value>),
    Boolean(bool),
    Map(HashMap<String, Value>),
    Ident(String),
}
impl IntoPy<Py<PyAny>> for Value {
    fn into_py(self, py: Python) -> Py<PyAny> {
        match self {
            Value::String(s) => s.into_py(py),
            Value::Array(a) => a.into_py(py),
            Value::Boolean(b) => b.into_py(py),
            Value::Map(d) => d.into_py(py),
            Value::Ident(i) => i.into_py(py),
        }
    }
}
#[derive(Debug, Clone)]
#[pyclass(unsendable)]
pub struct BluePrint {
    #[pyo3(get)]
    pub variables: HashMap<String, Value>,
    #[pyo3(get)]
    pub modules: Vec<Module>,
}
impl From<&RsBluePrint> for BluePrint {
    fn from(bp: &RsBluePrint) -> Self {
        let variables = bp.variables.iter().map(value_to_pyvalue).collect();
        let modules = bp.modules.iter().map(|b| Module::from(b)).collect();
        BluePrint { variables, modules }
    }
}
#[pymethods]
impl BluePrint {
    #[pyo3(name = "parse", signature = (input))]
    #[staticmethod]
    fn parse(input: &str) -> PyResult<Self> {
        let bp = RsBluePrint::parse(input).map_err(|e| e.to_string());
        match bp {
            Ok(bp) => Ok(BluePrint::from(&bp)),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PySyntaxError, _>(e)),
        }
    }
    #[staticmethod]
    #[pyo3(name = "from_file", signature = (path))]
    pub fn from_file(path: &str) -> PyResult<Self> {
        let contents = std::fs::read_to_string(&path).map_err(|e| e.to_string());
        let contents = match contents {
            Ok(c) => c,
            Err(e) => return Err(PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>(e)),
        };
        BluePrint::parse(&contents)
    }
    pub fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }
    fn modules_by_type(&self, typ: &str) -> Vec<Module> {
        self.modules
            .iter()
            .filter(|b| b.typ == typ)
            .cloned()
            .collect()
    }
}
impl From<&RsValue> for Value {
    fn from(v: &RsValue) -> Self {
        match v {
            RsValue::String(s) => Value::String(s.to_owned()),
            RsValue::Array(a) => Value::Array(a.iter().map(|x| Value::from(x)).collect()),
            RsValue::Boolean(b) => Value::Boolean(b.to_owned()),
            RsValue::Map(d) => Value::Map(map_to_py(&d)),
            RsValue::Ident(i) => Value::Ident(i.to_owned()),
        }
    }
}
fn value_to_pyvalue(t: (&String, &RsValue)) -> (String, Value) {
    let (k, v) = t;
    (k.to_owned(), v.into())
}
#[pymodule]
fn android_bp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BluePrint>()?;
    Ok(())
}
