mod card;
mod game;
mod player;
pub mod pygame;
mod round;
mod turn;

use game::*;
use player::*;
use pygame::*;

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;

#[pymodule]
fn pokr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGame>()?;
    m.add_class::<PySettings>()?;
    m.add_class::<PyAction>()?;
    Ok(())
}
