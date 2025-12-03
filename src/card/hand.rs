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

#[derive(Clone, Debug)]
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

    fn order_remaining_hand(
        hand_start: impl Iterator<Item = Value>,
        bin: &[usize; 13],
    ) -> [Value; 5] {
        hand_start
            .chain(
                bin.into_iter()
                    .enumerate()
                    .filter_map(|(i, count)| {
                        if *count == 1 {
                            Some(Value::from_index(i))
                        } else {
                            None
                        }
                    })
                    .rev(),
            )
            .collect_array::<5>()
            .unwrap()
    }

    pub fn get_hand_ordering(&self, hand_type: HandType) -> [Value; 5] {
        let bins = indexed_bins(self.cards.iter().map(|c| &c.value));

        match hand_type {
            HandType::StraightFlush => PokerHand::order_remaining_hand(std::iter::empty(), &bins),
            HandType::FourOfAKind => {
                let index_value = bins
                    .iter()
                    .enumerate()
                    .filter(|(_, count)| **count == 4)
                    .next()
                    .unwrap()
                    .0;

                let value = Value::from_index(index_value);

                PokerHand::order_remaining_hand(std::iter::repeat(value).take(4), &bins)
            }
            HandType::FullHouse => {
                let value_index_3 = bins
                    .iter()
                    .enumerate()
                    .filter(|(_, count)| **count == 3)
                    .next()
                    .unwrap()
                    .0;
                let value_index_2 = bins
                    .iter()
                    .enumerate()
                    .filter(|(_, count)| **count == 2)
                    .next()
                    .unwrap()
                    .0;

                std::iter::repeat(Value::from_index(value_index_3))
                    .take(3)
                    .chain(std::iter::repeat(Value::from_index(value_index_2)).take(2))
                    .collect_array::<5>()
                    .unwrap()
            }
            HandType::Flush => PokerHand::order_remaining_hand(std::iter::empty(), &bins),
            HandType::Straight => PokerHand::order_remaining_hand(std::iter::empty(), &bins),
            HandType::ThreeOfAKind => {
                let value_index = bins
                    .iter()
                    .enumerate()
                    .filter(|(_, count)| **count == 3)
                    .next()
                    .unwrap()
                    .0;

                let value = Value::from_index(value_index);

                PokerHand::order_remaining_hand(std::iter::repeat(value).take(3), &bins)
            }
            HandType::DoublePair => {
                let (value_low, value_high) = bins
                    .iter()
                    .enumerate()
                    .filter_map(|(i, count)| {
                        if *count == 2 {
                            Some(Value::from_index(i))
                        } else {
                            None
                        }
                    })
                    .collect_tuple()
                    .unwrap();

                PokerHand::order_remaining_hand(
                    std::iter::repeat(value_high)
                        .take(2)
                        .chain(std::iter::repeat(value_low).take(2)),
                    &bins,
                )
            }
            HandType::Pair => {
                let value_index = bins
                    .iter()
                    .enumerate()
                    .filter(|(_, count)| **count == 2)
                    .next()
                    .unwrap()
                    .0;

                let value = Value::from_index(value_index);

                PokerHand::order_remaining_hand(std::iter::repeat(value).take(2), &bins)
            }
            HandType::HighCard => PokerHand::order_remaining_hand(std::iter::empty(), &bins),
        }
    }

    fn best_hand_type(&self) -> HandType {
        for s in (0..=8).rev() {
            let hand_type = HandType::from_strength(s);
            if self.contains_hand(hand_type.clone()) {
                return hand_type;
            }
        }
        unreachable!()
    }

    pub fn contains_hand(&self, hand_type: HandType) -> bool {
        match hand_type {
            HandType::StraightFlush => {
                self.contains_hand(HandType::Flush) && self.contains_hand(HandType::Straight)
            }
            HandType::FourOfAKind => {
                let bins = indexed_bins(self.cards.iter().map(|c| &c.value));
                bins.into_iter().any(|count| count >= 4)
            }
            HandType::FullHouse => {
                let bins = indexed_bins(self.cards.iter().map(|c| &c.value));
                let contains_pair = bins.into_iter().any(|count| count == 2);
                let contains_three_of_a_kind = bins.into_iter().any(|count| count == 3);
                contains_pair && contains_three_of_a_kind
            }
            HandType::Flush => {
                let bins = indexed_bins(self.cards.iter().map(|c| &c.suit));
                bins.into_iter().any(|count| count == 5)
            }
            HandType::Straight => {
                let bins = indexed_bins(self.cards.iter().map(|c| &c.suit));
                let mut low_i = 12;
                let mut high_i = 3;
                let mut sum = bins.iter().cycle().skip(12).take(5).sum::<usize>();

                while high_i < 13 {
                    if sum == 5 {
                        return true;
                    }

                    sum += bins[high_i] - bins[low_i];
                    high_i += 1;
                    low_i = (low_i + 1) % 13;
                }

                sum == 5
            }
            HandType::ThreeOfAKind => {
                let bins = indexed_bins(self.cards.iter().map(|c| &c.value));
                bins.into_iter().any(|count| count >= 3)
            }
            HandType::DoublePair => {
                let bins = indexed_bins(self.cards.iter().map(|c| &c.value));
                bins.into_iter().filter(|&count| count >= 2).count() == 2
            }
            HandType::Pair => {
                let bins = indexed_bins(self.cards.iter().map(|c| &c.value));
                bins.into_iter().any(|count| count >= 2)
            }
            HandType::HighCard => true,
        }
    }
}

#[test]
fn card_ordering() {
    let suit = Suit::Clubs;
    let other_suit = Suit::Hearts;
    let ace = Card::new(suit, Value::Ace);
    let two = Card::new(suit, Value::Two);
    let three = Card::new(suit, Value::Three);
    let four = Card::new(suit, Value::Four);
    let five = Card::new(suit, Value::Five);
    let ten = Card::new(suit, Value::Ten);
    let jack = Card::new(suit, Value::Jack);
    let queen = Card::new(suit, Value::Queen);
    let king_unsuited = Card::new(other_suit, Value::King);
    let five_unsuited = Card::new(other_suit, Value::Five);

    let straight_flush = PokerHand {
        cards: [ace, two, three, four, five],
    };

    assert!(straight_flush.contains_hand(HandType::Straight));
    assert!(straight_flush.contains_hand(HandType::StraightFlush));
    assert!(!straight_flush.contains_hand(HandType::Pair));

    let straight = PokerHand {
        cards: [ace, two, three, four, five_unsuited],
    };

    assert!(straight.contains_hand(HandType::Straight));
    assert!(!straight.contains_hand(HandType::StraightFlush));

    let royal_straight = PokerHand {
        cards: [ace, king_unsuited, jack, queen, ten],
    };

    assert!(royal_straight.contains_hand(HandType::Straight));

    assert!(royal_straight > straight);
}

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_type = self.best_hand_type();
        let other_hand_type = other.best_hand_type();

        if hand_type.strength() < other_hand_type.strength() {
            return Some(std::cmp::Ordering::Less);
        } else if hand_type.strength() > other_hand_type.strength() {
            return Some(std::cmp::Ordering::Greater);
        }

        let ordered_values = self.get_hand_ordering(hand_type);
        let other_ordered_values = other.get_hand_ordering(other_hand_type);

        for i in 0..5 {
            if ordered_values[i].number_value() < other_ordered_values[i].number_value() {
                return Some(std::cmp::Ordering::Less);
            } else if ordered_values[i].number_value() > other_ordered_values[i].number_value() {
                return Some(std::cmp::Ordering::Greater);
            }
        }

        Some(std::cmp::Ordering::Equal)
    }
}

#[derive(Clone)]
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

impl HandType {
    pub fn strength(&self) -> usize {
        match self {
            HandType::StraightFlush => 8,
            HandType::FourOfAKind => 7,
            HandType::FullHouse => 6,
            HandType::Flush => 5,
            HandType::Straight => 4,
            HandType::ThreeOfAKind => 3,
            HandType::DoublePair => 2,
            HandType::Pair => 1,
            HandType::HighCard => 0,
        }
    }

    pub fn from_strength(strength: usize) -> Self {
        match strength {
            8 => HandType::StraightFlush,
            7 => HandType::FourOfAKind,
            6 => HandType::FullHouse,
            5 => HandType::Flush,
            4 => HandType::Straight,
            3 => HandType::ThreeOfAKind,
            2 => HandType::DoublePair,
            1 => HandType::Pair,
            0 => HandType::HighCard,
            _ => panic!("wrong hand type strength"),
        }
    }
}

impl PartialEq for PokerHand {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == std::cmp::Ordering::Equal
    }
}

impl Eq for PokerHand {}

trait Indexed<const N: usize> {
    fn index(&self) -> usize;
    fn from_index(index: usize) -> Self;
}

impl Indexed<4> for Suit {
    fn index(&self) -> usize {
        match self {
            Suit::Spades => 0,
            Suit::Diamonds => 1,
            Suit::Clubs => 2,
            Suit::Hearts => 3,
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => Suit::Spades,
            1 => Suit::Diamonds,
            2 => Suit::Clubs,
            3 => Suit::Hearts,
            _ => panic!("wrong index"),
        }
    }
}

impl Indexed<13> for Value {
    fn index(&self) -> usize {
        match self {
            Value::Two => 0,
            Value::Three => 1,
            Value::Four => 2,
            Value::Five => 3,
            Value::Six => 4,
            Value::Seven => 5,
            Value::Eight => 6,
            Value::Nine => 7,
            Value::Ten => 8,
            Value::Jack => 9,
            Value::Queen => 10,
            Value::King => 11,
            Value::Ace => 12,
        }
    }

    fn from_index(index: usize) -> Self {
        match index {
            0 => Value::Two,
            1 => Value::Three,
            2 => Value::Four,
            3 => Value::Five,
            4 => Value::Six,
            5 => Value::Seven,
            6 => Value::Eight,
            7 => Value::Nine,
            8 => Value::Ten,
            9 => Value::Jack,
            10 => Value::Queen,
            11 => Value::King,
            12 => Value::Ace,
            _ => panic!("wrong index"),
        }
    }
}

fn indexed_bins<'a, const N: usize, T: Indexed<N> + 'a>(
    items: impl Iterator<Item = &'a T>,
) -> [usize; N] {
    let mut bins = [0; N];
    for e in items {
        bins[e.index()] += 1;
    }
    bins
}
