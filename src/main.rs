use std::ffi::CString;
use pyo3::{types::PyAnyMethods, Python};

fn main() {
    let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
    let y: Vec<f64> = x.iter().map(|xi| xi.powi(2)).collect(); 

    Python::with_gil(|py| {
        let locals = pyo3::types::PyDict::new(py);
        locals.set_item("x", &x).unwrap();
        locals.set_item("y", &y).unwrap();

        let code = CString::new(
            "import matplotlib\nimport matplotlib.pyplot as plt\nplt.plot(x, y)\nplt.show()"
        ).unwrap();

        py.run(&code, None, Some(&locals)).unwrap();
    });
}