use flowx_core::ai::{AIAgent, TaskResult};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass]
pub struct PyAIAgent {
    agent: AIAgent,
}

#[pymethods]
impl PyAIAgent {
    #[new]
    #[pyo3(signature = (config_path = "flowx.toml"))]
    fn new(config_path: &str) -> PyResult<Self> {
        let agent = AIAgent::from_config(config_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        Ok(Self { agent })
    }

    #[staticmethod]
    fn from_default() -> PyResult<Self> {
        let agent = AIAgent::from_default_config()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        Ok(Self { agent })
    }

    fn execute(&self, instruction: String, screenshot: &[u8]) -> PyResult<PyTaskResult> {
        let result = self
            .agent
            .execute(&instruction, screenshot)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        Ok(PyTaskResult { inner: result })
    }

    fn ask(&self, question: String, screenshot: &[u8]) -> PyResult<String> {
        self.agent
            .ask(&question, screenshot)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))
    }

    fn set_max_steps(&mut self, max_steps: usize) {
        self.agent.set_max_steps(max_steps);
    }
}

#[pyclass]
pub struct PyTaskResult {
    inner: TaskResult,
}

#[pymethods]
impl PyTaskResult {
    #[getter]
    fn success(&self) -> bool {
        self.inner.success
    }

    #[getter]
    fn error(&self) -> Option<String> {
        self.inner.error.clone()
    }

    #[getter]
    fn steps(&self) -> usize {
        self.inner.steps.len()
    }
}
