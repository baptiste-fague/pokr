use crate::card::*;
use rand::{RngCore, seq::SliceRandom};

pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    pub fn draw_card(&mut self) -> Result<Card, CardError> {
        self.deck.pop().ok_or(CardError::EmptyDeck)
    }

    pub fn draw_hand(&mut self) -> Result<PlayerHand, CardError> {
        let card1 = self.draw_card()?;
        let card2 = self.draw_card()?;
        Ok(PlayerHand {
            cards: [card1, card2],
        })
    }

    pub fn shuffle(&mut self, rng: &mut impl RngCore) {
        self.deck.shuffle(rng)
    }

    pub fn new() -> Self {
        let mut deck = Vec::new();

        for suit in [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            for value in [
                Value::Ace,
                Value::King,
                Value::Queen,
                Value::Jack,
                Value::Ten,
                Value::Nine,
                Value::Eight,
                Value::Seven,
                Value::Six,
                Value::Five,
                Value::Four,
                Value::Three,
                Value::Two,
            ] {
                deck.push(Card { suit, value });
            }
        }

        Self { deck: deck }
    }
}
