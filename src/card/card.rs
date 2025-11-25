#[derive(Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

#[derive(Clone, Copy, PartialEq)]
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
    fn number_value(&self) -> usize {
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

#[derive(Clone, Copy, PartialEq)]
pub enum Suit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}
