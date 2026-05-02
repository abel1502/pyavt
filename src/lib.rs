use pyo3::prelude::*;
use avt;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[pyclass(skip_from_py_object)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pen(avt::Pen);

#[pyclass]
#[derive(Debug)]
struct Vt(avt::Vt);

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
