use pyo3::{exceptions::PyIndexError, prelude::*};
use avt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[pyclass(skip_from_py_object)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Cell(avt::Cell);

#[pymethods]
impl Cell {
    pub fn is_default(&self) -> bool {
        self.0.is_default()
    }

    #[getter]
    pub fn char(&self) -> char {
        self.0.char()
    }

    #[getter]
    pub fn width(&self) -> usize {
        self.0.width()
    }

    #[getter]
    pub fn pen(&self) -> Pen {
        Pen(self.0.pen().clone())
    }

    pub fn set(&mut self, ch: char, width: usize, pen: &Pen) {
        self.0.set(ch, width, pen.0.clone());
    }
}

#[pyclass(skip_from_py_object, eq, eq_int)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Charset {
    Ascii,
    Drawing,
}

impl From<avt::Charset> for Charset {
    fn from(other: avt::Charset) -> Self {
        match other {
            avt::Charset::Ascii => Charset::Ascii,
            avt::Charset::Drawing => Charset::Drawing,
        }
    }
}

impl From<Charset> for avt::Charset {
    fn from(other: Charset) -> Self {
        match other {
            Charset::Ascii => avt::Charset::Ascii,
            Charset::Drawing => avt::Charset::Drawing,
        }
    }
}

#[pymethods]
impl Charset {
    pub fn translate(&self, ch: char) -> char {
        avt::Charset::from(self.clone()).translate(ch)
    }
}

#[pyclass(skip_from_py_object)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color{
    Indexed(u8),
    RGB(u8, u8, u8),
}

impl From<avt::Color> for Color {
    fn from(other: avt::Color) -> Self {
        match other {
            avt::Color::Indexed(c) => Color::Indexed(c),
            avt::Color::RGB(rgb8) => Color::RGB(rgb8.r, rgb8.g, rgb8.b),
        }
    }
}

impl From<Color> for avt::Color {
    fn from(other: Color) -> Self {
        match other {
            Color::Indexed(c) => avt::Color::Indexed(c),
            Color::RGB(r, g, b) => avt::Color::rgb(r, g, b),
        }
    }
}

#[pyclass(skip_from_py_object)]
#[derive(Clone, PartialEq)]
struct Line(avt::Line);

#[pymethods]
impl Line {
    pub fn __len__(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[getter]
    pub fn cells(&self) -> Vec<Cell> {
        self.0.cells().iter().map(|c| Cell(*c)).collect()
    }

    pub fn __getitem__(&self, index: usize) -> PyResult<Cell> {
        self.0.cells().get(index).copied().map(Cell).ok_or(PyIndexError::new_err("Cell index out of range"))
    }

    // Could theoretically be an iterator, but the memory is laughable (how big can a line be?), and the overhead from repeated calls into and out of Python for an iterator would make it pointless
    fn chunks(
        &self,
        py: Python<'_>,
        predicate: Py<PyAny>,
    ) -> PyResult<Vec<Vec<Cell>>> {
        let predicate = predicate.bind(py);

        Ok(self
            .0
            .chunks(|a, b| {
                predicate
                    .call1((Cell(*a), Cell(*b)))
                    .and_then(|value| value.extract::<bool>())
                    .unwrap_or(false)
            })
            .map(|chunk| chunk.into_iter().map(Cell).collect())
            .collect()
        )
    }

    // No chars() because without an interator, it's the exact same as text()

    #[getter]
    pub fn text(&self) -> String {
        self.0.text()
    }
}

#[pyclass(skip_from_py_object)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
struct Pen(avt::Pen);

#[pymethods]
impl Pen {
    #[getter]
    pub fn foreground(&self) -> Option<Color> {
        self.0.foreground().map(|c| Color::from(c))
    }

    #[getter]
    pub fn background(&self) -> Option<Color> {
        self.0.background().map(|c| Color::from(c))
    }

    #[getter]
    pub fn bold(&self) -> bool {
        self.0.is_bold()
    }

    #[getter]
    pub fn faint(&self) -> bool {
        self.0.is_faint()
    }

    #[getter]
    pub fn italic(&self) -> bool {
        self.0.is_italic()
    }

    #[getter]
    pub fn underline(&self) -> bool {
        self.0.is_underline()
    }

    #[getter]
    pub fn strikethrough(&self) -> bool {
        self.0.is_strikethrough()
    }

    #[getter]
    pub fn blink(&self) -> bool {
        self.0.is_blink()
    }

    #[getter]
    pub fn inverse(&self) -> bool {
        self.0.is_inverse()
    }

    #[setter]
    pub fn set_italic(&mut self, value: bool) {
        if value {
            self.0.set_italic();
        } else {
            self.0.unset_italic();
        }
    }

    #[setter]
    pub fn set_underline(&mut self, value: bool) {
        if value {
            self.0.set_underline();
        } else {
            self.0.unset_underline();
        }
    }

    #[setter]
    pub fn set_blink(&mut self, value: bool) {
        if value {
            self.0.set_blink();
        } else {
            self.0.unset_blink();
        }
    }

    #[setter]
    pub fn set_strikethrough(&mut self, value: bool) {
        if value {
            self.0.set_strikethrough();
        } else {
            self.0.unset_strikethrough();
        }
    }

    #[setter]
    pub fn set_inverse(&mut self, value: bool) {
        if value {
            self.0.set_inverse();
        } else {
            self.0.unset_inverse();
        }
    }

    pub fn is_default(&self) -> bool {
        self.0.is_default()
    }
}

#[pyclass]
#[derive(Debug)]
struct Vt(avt::Vt);

#[pymethods]
impl Vt {
    #[new]
    #[pyo3(signature = (cols, rows, scrollback_limit = None))]
    fn new(cols: usize, rows: usize, scrollback_limit: Option<usize>) -> Self {
        let mut builder = avt::Vt::builder();
        builder.size(cols, rows);
        if let Some(limit) = scrollback_limit {
            builder.scrollback_limit(limit);
        }
        Self(builder.build())
    }

    // TODO: The scrollback might actually benefit from being an iterator
    // TODO: Return a namedtuple?
    /// Returns a tuple of `(lines, scrollback)`
    fn feed_str(&mut self, s: &str) -> (Vec<usize>, Vec<Line>) {
        let changes = self.0.feed_str(s);
        (changes.lines, changes.scrollback.map(Line).collect())
    }

    fn feed(&mut self, input: char) {
        self.0.feed(input)
    }

    #[getter]
    fn size(&self) -> (usize, usize) {
        self.0.size()
    }

    // TODO: Same as feed_str. Also, perhaps unify the behavior regardless
    /// Returns a tuple of `(lines, scrollback)`
    fn resize(&mut self, cols: usize, rows: usize) -> (Vec<usize>, Vec<Line>) {
        let changes = self.0.resize(cols, rows);
        (changes.lines, changes.scrollback.map(Line).collect())
    }

    // TODO: Can be an interator
    fn view(&self) -> Vec<Line> {
        self.0.view().map(|l| Line(l.clone())).collect()
    }

    // TODO: Can be an interator
    fn lines(&self) -> Vec<Line> {
        self.0.lines().map(|l| Line(l.clone())).collect()
    }

    // TODO: Maybe preserve the reference somehow?
    fn __getitem__(&self, index: usize) -> PyResult<Line> {
        if index >= self.0.lines().count() {
            return Err(PyIndexError::new_err("Line index out of range"));
        }
        Ok(Line(self.0.line(index).clone()))
    }

    fn text(&self) -> Vec<String> {
        self.0.text()
    }

    // TODO: uncomment once mod terminal is ported
    // #[getter]
    // fn cursor(&self) -> Cursor {
    //     self.0.cursor()
    // }

    #[getter]
    fn cursor_key_app_mode(&self) -> bool {
        self.0.cursor_key_app_mode()
    }

    fn dump(&self) -> String {
        self.0.dump()
    }
}

// TODO: pub modules too

#[pymodule(name = "avt")]
fn avt_module(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", VERSION)?;
    m.add_class::<Cell>()?;
    m.add_class::<Charset>()?;
    m.add_class::<Color>()?;
    m.add_class::<Line>()?;
    m.add_class::<Pen>()?;
    m.add_class::<Vt>()?;

    Ok(())
}
