use itertools::Itertools;

use crate::{GameError, action::Action, card::Deck, game_state::GameState, round::Round};

pub struct Hand {}

impl Hand {
    pub fn start(game_state: &mut GameState, small_blind: usize) -> Result<(), GameError> {
        // update round
        game_state.round = Round::PreFlop;

        // change current player
        game_state.current_seat = game_state.sb_seat;

        // put blinds
        Action::raise(game_state, small_blind)?;
        game_state.current_seat = game_state.next_playing_seat(game_state.current_seat);
        Action::raise(game_state, 2 * small_blind)?;
        game_state.current_seat = game_state.next_playing_seat(game_state.current_seat);

        // deal new hand
        game_state.deck = Deck::new();
        game_state.seats.iter_mut().try_for_each(|seat| {
            seat.hand = Some(game_state.deck.draw_hand()?);
            Ok(())
        })?;

        Ok(())
    }

    pub fn finish(game_state: &mut GameState) -> Result<(), GameError> {
        if game_state.seats.iter().filter(|s| s.is_valid()).count() == 1 {
            let gains = game_state
                .seats
                .iter_mut()
                .map(|s| {
                    // winner takes it all
                    let gain = s.total_bet;
                    s.total_bet = 0;
                    gain
                })
                .sum::<usize>();
            let valid_seat = game_state.seats.iter_mut().find(|s| s.is_valid()).unwrap();
            valid_seat.stack += gains;
        } else {
            // end current hand:
            //   - check for winner(s)s
            //   - update winner(s) stack(s)
            //   - set dead flags
            // 1. rank the remaining players
            let mut player_hands = game_state
                .seats
                .iter()
                .enumerate()
                .filter_map(|(i, s)| {
                    if !s.is_valid() {
                        return None;
                    }
                    Some((
                        i,
                        game_state.board.best_poker_hand(&s.hand.unwrap()).unwrap(),
                    ))
                })
                .sorted_by(|(_, h1), (_, h2)| h1.cmp(h2))
                // todo: remove dynamic allocation here
                .collect_vec();

            let mut pot: usize = game_state.seats.iter().map(|s| s.total_bet).sum();
            while pot > 0 {
                // 2. winner takes from the pot
                let winner_idx = player_hands.last().unwrap().0;
                let winner_bet = game_state.seats[winner_idx].total_bet;
                let winner_gains: usize = game_state
                    .seats
                    .iter_mut()
                    .map(|s| {
                        // winner can't take more than their bet amount
                        let gain = s.total_bet.min(winner_bet);
                        s.total_bet -= gain;
                        gain
                    })
                    .sum();

                game_state.seats[winner_idx].stack += winner_gains;
                player_hands.pop();

                // build the side pot
                pot -= winner_gains;
            }
        }

        // update seats states
        game_state.seats.iter_mut().for_each(|s| {
            s.is_dead = s.stack == 0;
            s.is_folded = false
        });

        Ok(())
    }

    pub fn is_over(game_state: &GameState) -> bool {
        game_state.round == Round::River
            || game_state.seats.iter().filter(|s| s.is_valid()).count() == 1
    }
}
