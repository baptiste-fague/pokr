use crate::{GameError, game, game_state::GameState};

#[derive(Clone, Copy)]
pub enum Action {
    Fold,
    Raise(usize), // takes the player's total_bet after the turn (not the additional money) as argument
    Call,
    Check,
}

impl Action {
    pub fn raise(game_state: &mut GameState, amount_to_bet: usize) -> Result<(), GameError> {
        let current_seat = &mut game_state.seats[game_state.current_seat];

        // check if money is where your mouth is
        if current_seat.stack + current_seat.round_bet < amount_to_bet {
            return Err(GameError::InvalidRaise);
        }

        let min_raise = game_state.current_bet + game_state.current_raise;

        if current_seat.stack + current_seat.round_bet >= min_raise && amount_to_bet >= min_raise
            || current_seat.stack < min_raise && amount_to_bet > game_state.current_bet
        {
            game_state.current_raise = game_state
                .current_raise
                .max(amount_to_bet - game_state.current_bet);
            current_seat.stack -= game_state.current_raise;
            current_seat.round_bet = amount_to_bet;
            game_state.current_bet = game_state.current_bet.max(current_seat.round_bet);
            Ok(())
        } else {
            Err(GameError::InvalidRaise)
        }
    }

    pub fn call(game_state: &mut GameState) {
        let amount_to_put = game_state
            .current_bet
            .saturating_sub(game_state.current_seat().round_bet);
        let current_seat = game_state.current_seat_mut();
        current_seat.round_bet += amount_to_put;
        current_seat.stack -= amount_to_put;
    }

    pub fn check(game_state: &mut GameState) -> Result<(), GameError> {
        if game_state.current_bet > game_state.current_seat().round_bet {
            return Err(GameError::InvalidAction);
        }
        Ok(())
    }
    pub fn fold(game_state: &mut GameState) {
        game_state.current_seat_mut().is_folded = true;
    }
}
