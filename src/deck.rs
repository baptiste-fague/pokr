use itertools::{self, Itertools};
use rand::{
    RngCore,
    seq::{IteratorRandom, SliceRandom},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CardError {
    #[error("Invalid board card count")]
    InvalidBoardCardCount,
    #[error("Invalid poker hand card count")]
    InvalidPokerHandCardCount,
}

pub struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    fn draw_card(&mut self) -> Card {
        todo!()
    }

    fn shuffle(&mut self, rng: &mut impl RngCore) {}

    fn new() -> Self {
        todo!()
    }
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

impl PlayerHand {
    fn cards<'a>(&'a self) -> impl Iterator<Item = &'a Card> {
        self.cards.iter()
    }
}

#[derive(Clone)]
pub struct PokerHand {
    cards: [Card; 5],
}

impl PokerHand {
    fn new<'a>(cards: impl Iterator<Item = &'a Card>) -> Result<Self, CardError> {
        Ok(Self {
            cards: cards
                .copied()
                .collect_array::<5>()
                .ok_or(CardError::InvalidPokerHandCardCount)?,
        })
    }
}

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl PartialEq for PokerHand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == std::cmp::Ordering::Equal
    }
}

impl Eq for PokerHand {}

pub struct Board {
    card_count: usize,
    cards: [Option<Card>; 5],
}

impl Board {
    fn new() -> Self {
        Self {
            cards: [None; 5],
            card_count: 0,
        }
    }

    fn best_poker_hand(&self, player_hand: &PlayerHand) -> Result<PokerHand, CardError> {
        let mut best_hand = None;

        for h in player_hand
            .cards()
            .chain(self.cards())
            // combinations uses vecs
            // todo: remove dynamic allocation with a custom combinations function
            .combinations(5)
            .map(|cards| PokerHand::new(cards.into_iter()))
        {
            let hand = h?;
            best_hand = Some(best_hand.map_or(hand.clone(), |m: PokerHand| m.max(hand)));
        }

        Ok(best_hand.unwrap())
    }

    fn add_card(&mut self, card: Card) -> Result<(), CardError> {
        if self.card_count >= 5 {
            return Err(CardError::InvalidBoardCardCount);
        }
        self.cards[self.card_count] = Some(card);
        self.card_count += 1;
        Ok(())
    }

    fn card_count(&self) -> usize {
        self.card_count
    }

    fn cards<'a>(&'a self) -> impl Iterator<Item = &'a Card> {
        self.cards.iter().filter_map(|c| c.as_ref())
    }
}
