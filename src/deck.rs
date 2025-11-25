pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    fn draw_card(&mut self) -> Card {
        todo!()
    }

    fn shuffle(&mut self, rng: &mut impl RngCore) {}

    fn new() -> Self {}
}

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

pub struct Board {
    cards: [Option<Card>; 5],
}

impl Board {
    fn new() {
        Board { cards: [None; 5] }
    }

    fn best_poker_hand(&self, player_hand: &PlayerHand) -> PokerHand {}

    fn add_card(&mut self, card: Card) {}

    fn card_count(&self) -> usize {}
}
