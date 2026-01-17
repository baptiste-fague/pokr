use crate::game_state::GameState;
use crate::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Round {
    PreFlop,
    Flop,
    Turn,
    River,
}

impl Round {
    pub fn next_round_name(&self) -> Result<Self, GameError> {
        match self {
            Round::PreFlop => Ok(Round::Flop),
            Round::Flop => Ok(Round::Turn),
            Round::Turn => Ok(Round::River),
            Round::River => Err(GameError::InvalidRoundCall),
        }
    }

    pub fn finish(game_state: &mut GameState) {
        game_state.current_bet = 0;
        game_state.current_raise = 0;

        // move round bet to total bet
        game_state.seats.iter_mut().for_each(|s| {
            s.total_bet += s.round_bet;
            s.round_bet = 0;
            s.played_this_round = false;
        });
    }

    pub fn start(game_state: &mut GameState) -> Result<(), GameError> {
        // add cards to board
        let cards_to_add = match game_state.round {
            Round::Flop => 3,
            Round::Turn => 1,
            Round::River => 1,
            _ => panic!(),
        };
        (0..cards_to_add).try_for_each(|_| {
            let new_card = game_state.deck.draw_card();
            game_state.board.add_card(new_card?)?;
            Ok(())
        })?;

        // update current seat
        game_state.current_seat = game_state.sb_seat;

        Ok(())
    }

    pub fn is_over(game_state: &GameState) -> bool {
        // 1. everyone has played once
        let everyone_has_played_once = game_state
            .seats
            .iter()
            .all(|seat| !seat.can_play() || seat.played_this_round);

        if !everyone_has_played_once {
            return false;
        }

        // 2. everyone (except all-in players) has the same bet (max of every valid player including
        //    all-in players)
        game_state
            .seats
            .iter()
            .filter(|s| s.can_play())
            .all(|seat| seat.round_bet == game_state.current_bet)
    }
}
