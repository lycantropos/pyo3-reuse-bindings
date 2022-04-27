use pyo3::prelude::{pyclass, pymethods, pymodule, PyModule, PyResult, Python};

#[pyclass]
pub struct MyClass;

#[pymethods]
impl MyClass {
    #[new]
    fn new() -> Self {
        MyClass {}
    }
}

#[pymodule]
fn base(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<MyClass>()
}
