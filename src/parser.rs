use derive_into::Convert;
use pyo3::{PyTypeInfo, exceptions::PyTypeError, prelude::*};
use avt;

#[pyclass(module = "avt.parser", skip_from_py_object)]
#[derive(Debug, Default)]
pub(crate) struct Parser(avt::parser::Parser);

#[pymethods]
impl Parser {
    #[new]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn feed(&mut self, input: char) -> Option<Function> {
        self.0.feed(input).map(Function::from)
    }
}

#[pyclass(module = "avt.parser", skip_from_py_object, frozen, eq, eq_int)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Convert)]
#[convert(from(path = "avt::parser::State"), into(path = "avt::parser::State"))]
pub(crate) enum State {
    #[default]
    Ground,
    Escape,
    EscapeIntermediate,
    CsiEntry,
    CsiParam,
    CsiIntermediate,
    CsiIgnore,
    DcsEntry,
    DcsParam,
    DcsIntermediate,
    DcsPassthrough,
    DcsIgnore,
    OscString,
    SosPmApcString,
}

#[pyclass(module = "avt.parser", from_py_object, frozen, eq)]
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Function {
    Bs(),
    Cbt(u16),
    Cha(u16),
    Cht(u16),
    Cnl(u16),
    Cpl(u16),
    Cr(),
    Ctc(CtcOp),
    Cub(u16),
    Cud(u16),
    Cuf(u16),
    Cup(u16, u16),
    Cuu(u16),
    Dch(u16),
    Decaln(),
    Decrc(),
    Decrst(Vec<DecMode>),
    Decsc(),
    Decset(Vec<DecMode>),
    Decstbm(u16, u16),
    Decstr(),
    Dl(u16),
    Ech(u16),
    Ed(EdScope),
    El(ElScope),
    G1d4(crate::Charset),
    Gzd4(crate::Charset),
    Ht(),
    Hts(),
    Ich(u16),
    Il(u16),
    Lf(),
    Nel(),
    Print(char),
    Rep(u16),
    Ri(),
    Ris(),
    Rm(Vec<AnsiMode>),
    Scorc(),
    Scosc(),
    Sd(u16),
    Sgr(Vec<SgrOp>),
    Si(),
    Sm(Vec<AnsiMode>),
    So(),
    Su(u16),
    Tbc(TbcScope),
    Vpa(u16),
    Vpr(u16),
    Xtwinops(XtwinopsOp),
}

impl From<avt::parser::Function> for Function {
    fn from(func: avt::parser::Function) -> Self {
        use avt::parser::Function::*;
        match func {
            Bs => Function::Bs(),
            Cbt(n) => Function::Cbt(n),
            Cha(n) => Function::Cha(n),
            Cht(n) => Function::Cht(n),
            Cnl(n) => Function::Cnl(n),
            Cpl(n) => Function::Cpl(n),
            Cr => Function::Cr(),
            Ctc(op) => Function::Ctc(op.into()),
            Cub(n) => Function::Cub(n),
            Cud(n) => Function::Cud(n),
            Cuf(n) => Function::Cuf(n),
            Cup(row, col) => Function::Cup(row, col),
            Cuu(n) => Function::Cuu(n),
            Dch(n) => Function::Dch(n),
            Decaln => Function::Decaln(),
            Decrc => Function::Decrc(),
            Decrst(modes) => Function::Decrst(modes.into_iter().map(Into::into).collect()),
            Decsc => Function::Decsc(),
            Decset(modes) => Function::Decset(modes.into_iter().map(Into::into).collect()),
            Decstbm(row, col) => Function::Decstbm(row, col),
            Decstr => Function::Decstr(),
            Dl(n) => Function::Dl(n),
            Ech(n) => Function::Ech(n),
            Ed(scope) => Function::Ed(scope.into()),
            El(scope) => Function::El(scope.into()),
            G1d4(charset) => Function::G1d4(charset.into()),
            Gzd4(charset) => Function::Gzd4(charset.into()),
            Ht => Function::Ht(),
            Hts => Function::Hts(),
            Ich(n) => Function::Ich(n),
            Il(n) => Function::Il(n),
            Lf => Function::Lf(),
            Nel => Function::Nel(),
            Print(c) => Function::Print(c),
            Rep(n) => Function::Rep(n),
            Ri => Function::Ri(),
            Ris => Function::Ris(),
            Rm(modes) => Function::Rm(modes.into_iter().map(Into::into).collect()),
            Scorc => Function::Scorc(),
            Scosc => Function::Scosc(),
            Sd(n) => Function::Sd(n),
            Sgr(modes) => Function::Sgr(modes.into_iter().map(Into::into).collect()),
            Si => Function::Si(),
            Sm(modes) => Function::Sm(modes.into_iter().map(Into::into).collect()),
            So => Function::So(),
            Su(n) => Function::Su(n),
            Tbc(scope) => Function::Tbc(scope.into()),
            Vpa(n) => Function::Vpa(n),
            Vpr(n) => Function::Vpr(n),
            Xtwinops(op) => Function::Xtwinops(op.into()),
        }
    }
}

impl From<Function> for avt::parser::Function {
    fn from(func: Function) -> Self {
        use avt::parser::Function::*;
        match func {
            Function::Bs() => Bs,
            Function::Cbt(n) => Cbt(n),
            Function::Cha(n) => Cha(n),
            Function::Cht(n) => Cht(n),
            Function::Cnl(n) => Cnl(n),
            Function::Cpl(n) => Cpl(n),
            Function::Cr() => Cr,
            Function::Ctc(op) => Ctc(op.into()),
            Function::Cub(n) => Cub(n),
            Function::Cud(n) => Cud(n),
            Function::Cuf(n) => Cuf(n),
            Function::Cup(row, col) => Cup(row, col),
            Function::Cuu(n) => Cuu(n),
            Function::Dch(n) => Dch(n),
            Function::Decaln() => Decaln,
            Function::Decrc() => Decrc,
            Function::Decrst(modes) => Decrst(modes.into_iter().map(Into::into).collect()),
            Function::Decsc() => Decsc,
            Function::Decset(modes) => Decset(modes.into_iter().map(Into::into).collect()),
            Function::Decstbm(row, col) => Decstbm(row, col),
            Function::Decstr() => Decstr,
            Function::Dl(n) => Dl(n),
            Function::Ech(n) => Ech(n),
            Function::Ed(scope) => Ed(scope.into()),
            Function::El(scope) => El(scope.into()),
            Function::G1d4(charset) => G1d4(charset.into()),
            Function::Gzd4(charset) => Gzd4(charset.into()),
            Function::Ht() => Ht,
            Function::Hts() => Hts,
            Function::Ich(n) => Ich(n),
            Function::Il(n) => Il(n),
            Function::Lf() => Lf,
            Function::Nel() => Nel,
            Function::Print(c) => Print(c),
            Function::Rep(n) => Rep(n),
            Function::Ri() => Ri,
            Function::Ris() => Ris,
            Function::Rm(modes) => Rm(modes.into_iter().map(Into::into).collect()),
            Function::Scorc() => Scorc,
            Function::Scosc() => Scosc,
            Function::Sd(n) => Sd(n),
            Function::Sgr(modes) => Sgr(modes.into_iter().map(Into::into).collect()),
            Function::Si() => Si,
            Function::Sm(modes) => Sm(modes.into_iter().map(Into::into).collect()),
            Function::So() => So,
            Function::Su(n) => Su(n),
            Function::Tbc(scope) => Tbc(scope.into()),
            Function::Vpa(n) => Vpa(n),
            Function::Vpr(n) => Vpr(n),
            Function::Xtwinops(op) => Xtwinops(op.into()),
        }
    }
}

#[pymethods]
impl Function {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Function.{:?}", self))
    }
}

#[pyclass(module = "avt.parser", from_py_object, frozen, eq, eq_int)]
#[derive(Debug, PartialEq, Clone, Copy, Convert)]
#[convert(from(path = "avt::parser::AnsiMode"), into(path = "avt::parser::AnsiMode"))]
pub(crate) enum AnsiMode {
    /// IRM
    Insert = 4,
    /// LNM
    NewLine = 20,
}

#[pyclass(module = "avt.parser", from_py_object, frozen, eq, eq_int)]
#[derive(Debug, PartialEq, Clone, Copy, Convert)]
#[convert(from(path = "avt::parser::CtcOp"), into(path = "avt::parser::CtcOp"))]
pub(crate) enum CtcOp {
    Set,
    ClearCurrentColumn,
    ClearAll,
}

// impl FromPyObject<'_, '_> for CtcOp {
//     type Error = PyErr;

//     fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
//         let py = obj.py();
//         let my_enum_type: Bound<'_, pyo3::types::PyType> = CtcOp::type_object(py);

//         if !obj.is_instance(&my_enum_type)? {
//             let repr_obj = obj.repr()?;
//             let repr = repr_obj.to_str()?;
//             return Err(PyTypeError::new_err(format!("Expected a variant of CtcOp, got {repr}")));
//         }

//         let value: u8 = obj.getattr("__int__")?.call0()?.extract()?;

//         match value {
//             x if x == CtcOp::Set as u8 => Ok(CtcOp::Set),
//             x if x == CtcOp::ClearCurrentColumn as u8 => Ok(CtcOp::ClearCurrentColumn),
//             x if x == CtcOp::ClearAll as u8 => Ok(CtcOp::ClearAll),
//             _ => Err(PyTypeError::new_err(format!("Invalid value for CtcOp: {value}"))),
//         }
//     }
// }

#[pyclass(module = "avt.parser", from_py_object, frozen, eq, eq_int)]
#[derive(Debug, PartialEq, Clone, Copy, Convert)]
#[convert(from(path = "avt::parser::DecMode"), into(path = "avt::parser::DecMode"))]
pub(crate) enum DecMode {
    /// DECCKM
    CursorKeys = 1,
    /// DECOM
    Origin = 6,
    /// DECAWM
    AutoWrap = 7,
    /// DECTCEM
    TextCursorEnable = 25,
    /// xterm
    AltScreenBuffer = 1047,
    /// xterm
    SaveCursor = 1048,
    /// xterm
    SaveCursorAltScreenBuffer = 1049,
}

#[pyclass(module = "avt.parser", from_py_object, frozen, eq, eq_int)]
#[derive(Debug, PartialEq, Clone, Copy, Convert)]
#[convert(from(path = "avt::parser::EdScope"), into(path = "avt::parser::EdScope"))]
pub(crate) enum EdScope {
    Below,
    Above,
    All,
    SavedLines,
}

#[pyclass(module = "avt.parser", from_py_object, frozen, eq, eq_int)]
#[derive(Debug, PartialEq, Clone, Copy, Convert)]
#[convert(from(path = "avt::parser::ElScope"), into(path = "avt::parser::ElScope"))]
pub(crate) enum ElScope {
    ToRight,
    ToLeft,
    All,
}

#[pyclass(module = "avt.parser", from_py_object, frozen, eq)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum SgrOp {
    /// 0
    Reset(),
    /// 1
    SetBoldIntensity(),
    /// 2
    SetFaintIntensity(),
    /// 3
    SetItalic(),
    /// 4
    SetUnderline(),
    /// 5
    SetBlink(),
    /// 7
    SetInverse(),
    /// 9
    SetStrikethrough(),
    /// 21, 22
    ResetIntensity(),
    /// 23
    ResetItalic(),
    /// 24
    ResetUnderline(),
    /// 25
    ResetBlink(),
    /// 27
    ResetInverse(),
    /// 29
    ResetStrikethrough(),
    /// 30-38
    SetForegroundColor(crate::Color),
    /// 39
    ResetForegroundColor(),
    /// 40-48
    SetBackgroundColor(crate::Color),
    /// 49
    ResetBackgroundColor(),
}

impl From<avt::parser::SgrOp> for SgrOp {
    fn from(op: avt::parser::SgrOp) -> Self {
        use avt::parser::SgrOp::*;
        match op {
            Reset => SgrOp::Reset(),
            SetBoldIntensity => SgrOp::SetBoldIntensity(),
            SetFaintIntensity => SgrOp::SetFaintIntensity(),
            SetItalic => SgrOp::SetItalic(),
            SetUnderline => SgrOp::SetUnderline(),
            SetBlink => SgrOp::SetBlink(),
            SetInverse => SgrOp::SetInverse(),
            SetStrikethrough => SgrOp::SetStrikethrough(),
            ResetIntensity => SgrOp::ResetIntensity(),
            ResetItalic => SgrOp::ResetItalic(),
            ResetUnderline => SgrOp::ResetUnderline(),
            ResetBlink => SgrOp::ResetBlink(),
            ResetInverse => SgrOp::ResetInverse(),
            ResetStrikethrough => SgrOp::ResetStrikethrough(),
            SetForegroundColor(color) => SgrOp::SetForegroundColor(color.into()),
            ResetForegroundColor => SgrOp::ResetForegroundColor(),
            SetBackgroundColor(color) => SgrOp::SetBackgroundColor(color.into()),
            ResetBackgroundColor => SgrOp::ResetBackgroundColor(),
        }
    }
}

impl From<SgrOp> for avt::parser::SgrOp {
    fn from(op: SgrOp) -> Self {
        use avt::parser::SgrOp::*;
        match op {
            SgrOp::Reset() => Reset,
            SgrOp::SetBoldIntensity() => SetBoldIntensity,
            SgrOp::SetFaintIntensity() => SetFaintIntensity,
            SgrOp::SetItalic() => SetItalic,
            SgrOp::SetUnderline() => SetUnderline,
            SgrOp::SetBlink() => SetBlink,
            SgrOp::SetInverse() => SetInverse,
            SgrOp::SetStrikethrough() => SetStrikethrough,
            SgrOp::ResetIntensity() => ResetIntensity,
            SgrOp::ResetItalic() => ResetItalic,
            SgrOp::ResetUnderline() => ResetUnderline,
            SgrOp::ResetBlink() => ResetBlink,
            SgrOp::ResetInverse() => ResetInverse,
            SgrOp::ResetStrikethrough() => ResetStrikethrough,
            SgrOp::SetForegroundColor(color) => SetForegroundColor(color.into()),
            SgrOp::ResetForegroundColor() => ResetForegroundColor,
            SgrOp::SetBackgroundColor(color) => SetBackgroundColor(color.into()),
            SgrOp::ResetBackgroundColor() => ResetBackgroundColor,
        }
    }
}

#[pymethods]
impl SgrOp {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("SgrOp.{:?}", self))
    }
}

#[pyclass(module = "avt.parser", from_py_object, frozen, eq, eq_int)]
#[derive(Debug, PartialEq, Clone, Copy, Convert)]
#[convert(from(path = "avt::parser::TbcScope"), into(path = "avt::parser::TbcScope"))]
pub(crate) enum TbcScope {
    CurrentColumn,
    All,
}

#[pyclass(module = "avt.parser", from_py_object, frozen, eq)]
#[derive(Debug, PartialEq, Clone, Copy, Convert)]
#[convert(from(path = "avt::parser::XtwinopsOp"), into(path = "avt::parser::XtwinopsOp"))]
pub(crate) enum XtwinopsOp {
    Resize(u16, u16),
}

pub(crate) fn init_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Parser>()?;
    m.add_class::<State>()?;
    m.add_class::<Function>()?;
    m.add_class::<AnsiMode>()?;
    m.add_class::<CtcOp>()?;
    m.add_class::<DecMode>()?;
    m.add_class::<EdScope>()?;
    m.add_class::<ElScope>()?;
    m.add_class::<SgrOp>()?;
    m.add_class::<TbcScope>()?;
    m.add_class::<XtwinopsOp>()?;

    Ok(())
}
