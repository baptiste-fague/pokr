pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {}

#[derive(Clone, Copy)]
pub struct Card {
    suit: Suit,
    value: Value,
}

#[derive(Clone, Copy)]
pub enum Suit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

#[derive(Clone, Copy)]
pub struct Value {
    n: u32,
}
