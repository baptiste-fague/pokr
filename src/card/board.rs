use itertools::Itertools;

use crate::card::*;

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

    pub fn best_poker_hand(&self, player_hand: &PlayerHand) -> Result<PokerHand, CardError> {
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

    pub fn add_card(&mut self, card: Card) -> Result<(), CardError> {
        if self.card_count >= 5 {
            return Err(CardError::BoardOverflow);
        }
        self.cards[self.card_count] = Some(card);
        self.card_count += 1;
        Ok(())
    }

    pub fn card_count(&self) -> usize {
        self.card_count
    }

    pub fn cards<'a>(&'a self) -> impl Iterator<Item = &'a Card> {
        self.cards.iter().filter_map(|c| c.as_ref())
    }
}
