mod action;
mod card;
mod game;
mod game_state;
mod hand;
mod player;
pub mod pygame;
mod round;

use game::*;
use player::*;
use pygame::*;

use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Action is invalid")]
    InvalidAction,
    #[error("Invalid raise amount")]
    InvalidRaise,
    #[error("Unexpected next_round call during River")]
    InvalidRoundCall,
    #[error("Tried to add a card to a full board")]
    BoardOverflow,
    #[error("Invalid poker hand card count")]
    InvalidPokerHandCardCount,
    #[error("Tried to draw from an empty deck")]
    EmptyDeck,
    #[error("Invalid round")]
    InvalidRound,
}

#[pymodule]
fn pokr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyGame>()?;
    m.add_class::<PySettings>()?;
    m.add_class::<PyAction>()?;
    Ok(())
}
