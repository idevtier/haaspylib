pub mod api;

use pyo3::prelude::*;

#[pyfunction]
fn init_logger() {
    env_logger::init();
}

#[pymodule]
fn haaspylib(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_logger, m)?)?;
    api::register(py, m)?;
    Ok(())
}
