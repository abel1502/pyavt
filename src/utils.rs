use pyo3::prelude::*;
use avt;

// TODO: utils.{TextUnwrapper.{new, push, flush}, TextCollector.{new, feed_str, resize, flush}}

pub(crate) fn init_module(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
