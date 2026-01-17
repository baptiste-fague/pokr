mod action;
mod card;
mod game;
mod game_state;
mod hand;
mod player;
mod round;
mod turn;

use game::*;
use player::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Action is invalid")]
    InvalidAction,
    #[error("Invalid round card count")]
    InvalidRoundCardCount,
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
}

fn main() -> Result<(), GameError> {
    let n = 3;
    let settings = Settings {
        n_players: n,
        initial_stack: 1000,
        small_blind: 10,
    };
    let players = vec![Player::new(); n];
    let mut game = Game::new(settings)?;

    while !game.is_over() {
        let seat_number = game.current_seat();

        let mut player = players[seat_number];
        let action = player.choose_action();
        game.play_turn(action)?;
    }

    Ok(())
}
