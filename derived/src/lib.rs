use base::MyClass;
use pyo3::prelude::{pyfunction, pymodule, PyModule, PyResult, Python};
use pyo3::wrap_pyfunction;

#[pyfunction]
fn use_my_class(value: &MyClass) {}

#[pymodule]
fn derived(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(use_my_class, module)?)
}
