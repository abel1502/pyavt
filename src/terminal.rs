use derive_into::Convert;
use pyo3::{exceptions::PyIndexError, prelude::*};
use avt;

#[pyclass(module = "avt.terminal", skip_from_py_object, frozen, eq)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Cursor(pub(crate) avt::terminal::Cursor);

#[pymethods]
impl Cursor {
    #[new]
    #[pyo3(signature = (col = 0, row = 0, visible = true))]
    fn new(col: usize, row: usize, visible: bool) -> Self {
        Self(avt::terminal::Cursor {
            col,
            row,
            visible,
        })
    }

    #[getter]
    fn col(&self) -> usize {
        self.0.col
    }

    #[getter]
    fn row(&self) -> usize {
        self.0.row
    }

    #[getter]
    fn visible(&self) -> bool {
        self.0.visible
    }

    fn __repr__(&self) -> String {
        format!("Cursor(col={}, row={}, visible={})", self.0.col, self.0.row, self.0.visible)
    }
}

#[pyclass(module = "avt.terminal", skip_from_py_object)]
#[derive(Debug)]
pub(crate) struct Terminal(avt::terminal::Terminal);

#[pymethods]
impl Terminal {
    #[new]
    #[pyo3(signature = (size = (80, 24), scrollback_limit = None))]
    fn new(size: (usize, usize), scrollback_limit: Option<usize>) -> Self {
        Self(avt::terminal::Terminal::new(size, scrollback_limit))
    }

    #[getter]
    fn size(&self) -> (usize, usize) {
        self.0.size()
    }

    #[getter]
    fn active_buffer_type(&self) -> BufferType {
        self.0.active_buffer_type().into()
    }

    fn execute(&mut self, fun: crate::parser::Function) {
        self.0.execute(fun.into());
    }

    #[getter]
    fn cursor(&self) -> Cursor {
        Cursor(self.0.cursor())
    }

    fn gc(&mut self) -> Vec<crate::Line> {
        self.0.gc().map(crate::Line).collect()
    }

    fn changes(&mut self) -> Vec<usize> {
        self.0.changes()
    }

    /// Returns whether the size changed
    fn resize(&mut self, cols: usize, rows: usize) -> bool {
        self.0.resize(cols, rows)
    }

    fn view(&self) -> Vec<crate::Line> {
        self.0.view().map(|l| crate::Line(l.clone())).collect()
    }

    fn lines(&self) -> Vec<crate::Line> {
        self.0.lines().map(|l| crate::Line(l.clone())).collect()
    }

    fn __getitem__(&self, index: usize) -> PyResult<crate::Line> {
        if index >= self.0.lines().count() {
            return Err(PyIndexError::new_err("Line index out of range"));
        }
        Ok(crate::Line(self.0.line(index).clone()))
    }

    fn text(&self) -> Vec<String> {
        self.0.text()
    }

    #[getter]
    fn cursor_keys_app_mode(&self) -> bool {
        self.0.cursor_keys_app_mode()
    }

    fn dump(&self) -> String {
        self.0.dump()
    }
}

#[pyclass(module = "avt.terminal", from_py_object, frozen, eq, eq_int)]
#[derive(Debug, Copy, Clone, PartialEq, Convert)]
#[convert(from(path = "avt::terminal::BufferType"), into(path = "avt::terminal::BufferType"))]
pub(crate) enum BufferType {
    Primary,
    Alternate,
}

pub(crate) fn init_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Cursor>()?;
    m.add_class::<Terminal>()?;
    m.add_class::<BufferType>()?;

    Ok(())
}
