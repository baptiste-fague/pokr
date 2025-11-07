pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {}

pub struct Card {
    suit: Suit,
    value: Value,
}

pub enum Suit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

pub struct Value {
    n: u32,
}
