use pyo3::{
    exceptions::ValueError,
    prelude::{pyfunction, pymodule, IntoPyObject, PyModule, PyObject, PyResult, Python},
    types::PyBytes,
    wrap_pyfunction,
};

use base65536::WrapOptions;

#[pyfunction]
fn encode(input: Vec<u8>) -> PyResult<String> {
    Ok(base65536::encode(&input, WrapOptions::NoWrap))
}

#[pyfunction]
fn decode(input: String) -> PyResult<Bytes> {
    base65536::decode(&input, false)
        .map(Bytes)
        .map_err(|e| ValueError::py_err(format!("{}", e)))
}

#[pymodule]
fn base65536(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(encode))?;
    m.add_wrapped(wrap_pyfunction!(decode))?;
    Ok(())
}

struct Bytes(Vec<u8>);

impl IntoPyObject for Bytes {
    fn into_object(self, py: Python) -> PyObject {
        PyBytes::new(py, &self.0).into_object(py)
    }
}
