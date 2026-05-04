use crate::card::*;
use crate::round::Round;

#[derive(Debug, Clone)]
pub struct GameState {
    pub current_seat: usize,
    /// maximum total amount bet by a player so far during the hand
    pub current_bet: usize,
    /// maximum raise (amount_bet - amount_bet_by_previous_player) so far during the hand
    pub current_raise: usize,

    pub deck: Deck,
    pub board: Board,

    pub seats: Vec<Seat>,
    pub sb_seat: usize,

    pub round: Round,
}

impl GameState {
    pub fn next_playing_seat(&self, seat: usize) -> usize {
        let mut next_seat = seat;
        loop {
            next_seat = (next_seat + 1) % self.seats.len();
            if self.seats[next_seat].can_play() {
                break;
            }
        }
        next_seat
    }

    pub fn current_seat_mut(&mut self) -> &mut Seat {
        &mut self.seats[self.current_seat]
    }

    pub fn current_seat(&self) -> &Seat {
        &self.seats[self.current_seat]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Seat {
    pub hand: Option<PlayerHand>,
    pub stack: usize,
    pub round_bet: usize,
    pub total_bet: usize,
    pub is_folded: bool,
    pub is_dead: bool,
    pub played_this_round: bool,
}

impl Seat {
    pub fn new(stack: usize) -> Self {
        Seat {
            hand: None,
            stack,
            round_bet: 0,
            total_bet: 0,
            is_folded: false,
            is_dead: false,
            played_this_round: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.is_dead && !self.is_folded
    }

    pub fn can_play(&self) -> bool {
        self.is_valid() && self.stack > 0
    }
}
