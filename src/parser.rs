use pyo3::prelude::*;
use avt;

// TODO: parser.{Parser.{new, feed}, State, Function, AnsiMode, CtcOp, DecMode, EdScope, ElScope, SgrOp, TbcScope, XtwinopsOp}

pub fn init_module(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
