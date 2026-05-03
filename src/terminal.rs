use pyo3::prelude::*;
use avt;

// TODO: terminal.{Cursor, Terminal.{new, size, active_buffer_type, execute, cursor, gc, changes, resize, view, lines, line, text, cursor_keys_app_mode, dump}, BufferType}

pub(crate) fn init_module(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
