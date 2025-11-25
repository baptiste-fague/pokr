use rand::{RngCore, seq::SliceRandom};

pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    pub fn draw_card(&mut self) -> Card {
        self.deck.pop().unwrap()
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

#[derive(Clone, Copy)]
pub struct Card {
    suit: Suit,
    value: Value,
}

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
        number_value(self.value).partial_cmp(number_value(other.value))
    }
}

impl Eq for Card {}

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

pub struct PlayerHand {
    cards: [Card; 2],
}

pub struct PokerHand {
    cards: [Card; 5],
}

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl PartialEq for PokerHand {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Eq for PokerHand {}

pub struct Board {
    cards: [Option<Card>; 5],
}

impl Board {
    fn new() -> Self {
        Board { cards: [None; 5] }
    }

    fn best_poker_hand(&self, player_hand: &PlayerHand) -> PokerHand {
        todo!()
    }

    fn add_card(&mut self, card: Card) {
        todo!()
    }

    fn card_count(&self) -> usize {
        todo!()
    }
}
