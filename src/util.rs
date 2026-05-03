use std::mem::swap;

use pyo3::prelude::*;
use avt;

// TODO: util.{TextUnwrapper.{new, push, flush}, TextCollector.{new, feed_str, resize, flush}}

#[pyclass(module = "avt.util", skip_from_py_object)]
#[derive(Default)]
pub(crate) struct TextUnwrapper(avt::util::TextUnwrapper);

#[pymethods]
impl TextUnwrapper {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, line: Bound<'_, crate::Line>) -> Option<String> {
        self.0.push(&line.borrow().0)
    }

    /// Note: this consumes the TextUnwrapper (replacing it with an empty one)
    fn flush(&mut self) -> Option<String> {
        let mut old = avt::util::TextUnwrapper::default();
        swap(&mut self.0, &mut old);
        old.flush()
    }
}

// This one cannot be a wrapper because the original wants to own a Vt. This is a reimplementation with Py references
#[pyclass(module = "avt.util", skip_from_py_object)]
pub(crate) struct TextCollector {
    #[pyo3(get, name = "_vt")]
    vt: Py<crate::Vt>,
    #[pyo3(get, name = "_unwrapper")]
    unwrapper: Py<TextUnwrapper>,
}

#[pymethods]
impl TextCollector {
    #[new]
    fn new(py: Python<'_>, vt: Py<crate::Vt>) -> PyResult<Self> {
        Ok(Self {
            vt,
            unwrapper: Py::new(py, TextUnwrapper::default())?,
        })
    }

    fn feed_str(&mut self, py: Python<'_>, s: &str) -> PyResult<Vec<String>> {
        let scrollback: Vec<_> = {
            let vt = self.vt.bind(py);
            let mut vt = vt.try_borrow_mut()?;
            vt.0.feed_str(s).scrollback.collect()
        };

        let unwrapper = self.unwrapper.bind(py);
        let mut unwrapper = unwrapper.try_borrow_mut()?;

        Ok(
            scrollback
            .into_iter()
            .filter_map(|l| unwrapper.0.push(&l))
            .collect()
        )
    }

    fn resize(&mut self, py: Python<'_>, cols: u16, rows: u16) -> PyResult<Vec<String>> {
        let scrollback: Vec<_> = {
            let vt = self.vt.bind(py);
            let mut vt = vt.try_borrow_mut()?;
            vt.0.resize(cols.into(), rows.into())
                .scrollback
                .collect()
        };

        let unwrapper = self.unwrapper.bind(py);
        let mut unwrapper = unwrapper.try_borrow_mut()?;

        Ok(
            scrollback
            .into_iter()
            .filter_map(|l| unwrapper.0.push(&l))
            .collect()
        )
    }

    fn flush(&mut self, py: Python<'_>) -> PyResult<Vec<String>> {
        let vt = self.vt.bind(py);
        let vt = vt.try_borrow_mut()?;

        let unwrapper = self.unwrapper.bind(py);
        let mut unwrapper = unwrapper.try_borrow_mut()?;

        let mut lines: Vec<String> = vt.0.lines().filter_map(|l| unwrapper.0.push(l)).collect();
        lines.extend(unwrapper.flush());

        while matches!(lines.last(), Some(last) if last.is_empty()) {
            lines.pop();
        }

        Ok(lines)
    }
}

pub(crate) fn init_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<TextUnwrapper>()?;
    m.add_class::<TextCollector>()?;

    Ok(())
}
