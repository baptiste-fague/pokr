use crate::card::*;
use itertools::Itertools;

pub struct PlayerHand {
    cards: [Card; 2],
}

impl PlayerHand {
    pub fn cards<'a>(&'a self) -> impl Iterator<Item = &'a Card> {
        self.cards.iter()
    }
}

#[derive(Clone)]
pub struct PokerHand {
    cards: [Card; 5],
}

impl PokerHand {
    pub fn new<'a>(cards: impl Iterator<Item = &'a Card>) -> Result<Self, CardError> {
        Ok(Self {
            cards: cards
                .copied()
                .collect_array::<5>()
                .ok_or(CardError::InvalidPokerHandCardCount)?,
        })
    }

    pub fn contains_hand(&self, hand_type: HandType) -> bool {
        match hand_type {
            HandType::StraightFlush => todo!(),
            HandType::FourOfAKind => todo!(),
            HandType::FullHouse => todo!(),
            HandType::Flush => todo!(),
            HandType::Straight => todo!(),
            HandType::ThreeOfAKind => todo!(),
            HandType::DoublePair => todo!(),
            HandType::Pair => todo!(),
            HandType::HighCard => true,
        }
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

pub enum HandType {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    DoublePair,
    Pair,
    HighCard,
}

impl PartialEq for PokerHand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == std::cmp::Ordering::Equal
    }
}

impl Eq for PokerHand {}

trait Indexed {
    fn index(&self) -> usize;
    fn from_index(index: usize) -> Self;
}

fn indexed_bins<'a, T: Indexed + 'a, const N: usize>(
    items: impl Iterator<Item = &'a T>,
) -> [usize; N] {
    let mut bins = [0; N];
    for e in items {
        bins[e.index()] += 1;
    }
    bins
}
