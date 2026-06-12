use pyo3::prelude::*;

mod ai;
mod device;
mod executor;

pub use ai::PyAIAgent;
pub use device::PyDevice;
pub use executor::PyMockExecutor;

#[pymodule]
fn flowx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyDevice>()?;
    m.add_class::<PyMockExecutor>()?;
    m.add_class::<PyAIAgent>()?;
    Ok(())
}
