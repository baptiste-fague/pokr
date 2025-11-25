mod board;
mod card;
mod deck;
mod hand;

pub use board::*;
pub use card::*;
pub use deck::*;
pub use hand::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CardError {
    #[error("Tried to add a card to a full board")]
    BoardOverflow,
    #[error("Invalid poker hand card count")]
    InvalidPokerHandCardCount,
    #[error("Tried to draw from an empty deck")]
    EmptyDeck,
}
