pub mod board;
pub mod deck;
pub mod hand;

use std::collections::btree_map::Values;

use rand::seq::IndexedRandom;

pub use board::*;
pub use deck::*;
pub use hand::*;

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Card { suit, value }
    }
    pub fn random() -> Self {
        let suit = Suit::random();
        let value = Value::random();
        Card {suit , value }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Value {
    pub fn number_value(&self) -> usize {
        match self {
            Value::Ace => 14,
            Value::King => 13,
            Value::Queen => 12,
            Value::Jack => 11,
            Value::Ten => 10,
            Value::Nine => 9,
            Value::Eight => 8,
            Value::Seven => 7,
            Value::Six => 6,
            Value::Five => 5,
            Value::Four => 4,
            Value::Three => 3,
            Value::Two => 2,
        }
    }

    pub fn random() -> Self{
        let mut rng = rand::rng();
        let values = [Value::Ace, 
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
                                Value::Two];
        *values.choose(&mut rng).unwrap()
    }

}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.value == other.value
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value
            .number_value()
            .partial_cmp(&other.value.number_value())
    }
}

impl Eq for Card {}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Suit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

impl Suit{
    pub fn random() -> Self{
        let mut rng = rand::rng();
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        *suits.choose(&mut rng).unwrap()
    }
}
