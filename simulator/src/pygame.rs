use crate::{action::Action, *};

use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

#[gen_stub_pyclass]
#[pyclass]
pub struct PySettings {
    settings: Settings,
}

#[gen_stub_pymethods]
#[pymethods]
impl PySettings {
    #[new]
    pub fn new(
        n_players: usize,
        initial_stack: usize,
        small_blind: usize,
        max_hands: usize,
    ) -> Self {
        PySettings {
            settings: Settings {
                n_players,
                initial_stack,
                small_blind,
                max_hands,
            },
        }
    }
}

#[gen_stub_pyclass]
#[pyclass]
pub struct PyAction {
    pub(crate) action: Action,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyAction {
    #[staticmethod]
    pub fn new_fold() -> Self {
        PyAction {
            action: Action::Fold,
        }
    }

    #[staticmethod]
    pub fn new_raise(amount: usize) -> Self {
        PyAction {
            action: Action::Raise(amount),
        }
    }

    #[staticmethod]
    pub fn new_check() -> Self {
        PyAction {
            action: Action::Check,
        }
    }

    #[staticmethod]
    pub fn new_call() -> Self {
        PyAction {
            action: Action::Call,
        }
    }
}

#[gen_stub_pyclass]
#[pyclass]
pub struct PyGame {
    game: Game,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyGame {
    #[new]
    pub fn new(settings: &PySettings) -> Self {
        PyGame {
            game: Game::new(settings.settings.to_owned()).unwrap(),
        }
    }

    pub fn play_turn(&mut self, action: &PyAction) {
        self.game.play_turn(action.action).unwrap();
    }

    pub fn current_seat(&self) -> usize {
        self.game.current_seat()
    }

    pub fn is_over(&self) -> bool {
        self.game.is_over()
    }
}

define_stub_info_gatherer!(stub_info);
