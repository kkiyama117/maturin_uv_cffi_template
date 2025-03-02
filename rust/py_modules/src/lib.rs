#[cfg(feature = "log")]
use log::LevelFilter;
use pyo3::prelude::*;
#[cfg(feature = "log")]
use pyo3_log::{Caching, Logger};

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_cffi")]
mod pyo3_root_modules {
    #[pymodule_export]
    use super::sum_as_string;
    use super::*;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // pyo3_log::init();
        Python::with_gil(|py| {
            // Some time in the future when logging changes, reset the caches:
            #[cfg(feature = "log")]
            {
                let handle = Logger::new(py, Caching::LoggersAndLevels)?
                    .filter(LevelFilter::Debug)
                    .filter_target("nature_remo_rust".to_owned(), LevelFilter::Debug)
                    .install()
                    .expect("Someone installed a logger before us :-(");
                handle.reset();
                log::debug!("Initialize Nature Remo PyO3 Rust");
            }
            // import to `sys.modules` for `from` import
            py.import("sys")?
                .getattr("modules")?
                .set_item("nature_remo_api._rust_api", m)
        })?;
        log::debug!("Initialized");
        Ok(())
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[cfg(feature = "tokio")]
async fn rust_sleep2() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

#[cfg(feature = "tokio")]
#[pyfunction]
fn call_rust_sleep(py: Python) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        rust_sleep2().await;
        Ok(())
    })
}

#[cfg(feature = "tokio")]
#[pyfunction]
fn rust_sleep(py: Python) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok(())
    })
}
