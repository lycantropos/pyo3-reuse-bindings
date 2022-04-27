use pyo3::prelude::{pyfunction, pymodule, PyModule, PyResult, Python};
use pyo3::wrap_pyfunction;
use base::MyClass;

#[pyfunction]
fn use_my_class(value: &MyClass) {}

#[pymodule]
fn derived(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(use_my_class, module)?)?;
    Ok(())
}
