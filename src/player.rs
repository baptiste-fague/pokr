use crate::*;

#[derive(Clone, Copy)]
pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Player {}
    }
    pub fn choose_action(&mut self) -> Action {
        Action::Check
    }
}
