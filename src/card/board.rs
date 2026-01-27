use itertools::Itertools;

use crate::{GameError, card::*};

#[derive(Debug)]
pub struct Board {
    card_count: usize,
    cards: [Option<Card>; 5],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cards: [None; 5],
            card_count: 0,
        }
    }

    pub fn from_cards(cards: Vec<Card>) -> Board {
        assert!(cards.len() <= 5);
        let card_count = cards.len();
        let cards = cards
            .into_iter()
            .map(Some)
            .chain(std::iter::repeat_n(None, 5 - card_count))
            .collect_array::<5>()
            .unwrap();
        Self { card_count, cards }
    }

    pub fn best_poker_hand(&self, player_hand: &PlayerHand) -> Result<PokerHand, GameError> {
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

    pub fn add_card(&mut self, card: Card) -> Result<(), GameError> {
        if self.card_count >= 5 {
            return Err(GameError::BoardOverflow);
        }
        self.cards[self.card_count] = Some(card);
        self.card_count += 1;
        Ok(())
    }

    pub fn cards(&self) -> impl Iterator<Item = &Card> {
        self.cards.iter().filter_map(|c| c.as_ref())
    }
}
