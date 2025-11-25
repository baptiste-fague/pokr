use crate::card::*;
use crate::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Action is invalid")]
    InvalidAction,
    #[error("Invalid round card count")]
    InvalidRoundCardCount,
}

#[derive(PartialEq, Eq)]
pub enum Round {
    PreFlop,
    Flop,
    Turn,
    River,
}

impl Round {
    fn n_cards(&self) -> usize {
        match self {
            Round::PreFlop => 0,
            Round::Flop => 3,
            Round::Turn => 4,
            Round::River => 5,
        }
    }

    fn from_card_count(n: usize) -> Result<Self, GameError> {
        match n {
            0 => Ok(Round::PreFlop),
            3 => Ok(Round::Flop),
            4 => Ok(Round::Turn),
            5 => Ok(Round::River),
            _ => Err(GameError::InvalidRoundCardCount),
        }
    }
}

pub struct GameState {
    current_seat: usize,

    board: Board,

    seats: Vec<Seat>,
    sb_seat: usize,

    pot: usize,

    round: Round,
}

impl GameState {
    fn next_valid_seat(&self, seat: usize) -> usize {
        let mut next_seat = seat;
        loop {
            next_seat = (next_seat + 1) % self.seats.len();
            if self.is_seat_valid(next_seat) {
                break;
            }
        }
        next_seat
    }

    fn is_seat_valid(&self, seat: usize) -> bool {
        self.seats[seat].is_valid()
    }
}

pub struct GameData {
    hand_cound: usize,
}

pub struct Game {
    settings: Settings,
    game_state: GameState,
    game_data: GameData,
}

#[derive(Clone, Copy)]
pub struct Seat {
    pub stack: usize,
    pub bet: usize,
    pub is_folded: bool,
    pub is_dead: bool,
    pub last_action_in_current_round: Option<Action>,
}

impl Seat {
    fn new(stack: usize) -> Self {
        Seat {
            stack,
            bet: 0,
            is_folded: false,
            is_dead: false,
            last_action_in_current_round: None,
        }
    }

    fn is_valid(&self) -> bool {
        !self.is_dead && !self.is_folded
    }
}

#[derive(Default)]
pub struct Settings {
    pub n_players: usize,
    pub initial_stack: usize,
}

impl Game {
    pub fn new(settings: Settings) -> Game {
        Game {
            game_state: GameState {
                current_seat: 0,
                board: Board::new(),
                seats: vec![Seat::new(settings.initial_stack); settings.n_players],
                sb_seat: 0,
                pot: 0,
                round: Round::PreFlop,
            },
            settings,
            game_data: GameData { hand_cound: 0 },
        }
    }

    pub fn play_turn(&mut self, action: Action) -> Result<(), GameError> {
        self.handle_action(action);
        self.state_logic();

        Ok(())
    }

    /// Go to next valid seat
    fn next_turn(&mut self) {
        self.game_state.current_seat = self
            .game_state
            .next_valid_seat(self.game_state.current_seat);
    }

    fn update_round(&mut self) -> Result<(), GameError> {
        let card_count = self.game_state.board.card_count();
        self.game_state.round = Round::from_card_count(card_count)?;
        Ok(())
    }

    fn is_round_over(game_state: &GameState) -> bool {
        if !game_state.seats.iter().any(|seat| seat.is_valid()) {
            return true;
        }

        // 1. everyone has played once
        let everyone_has_played_once = game_state
            .seats
            .iter()
            .all(|seat| !seat.is_valid() || seat.last_action_in_current_round.is_some());
        if !everyone_has_played_once {
            return false;
        }

        // 2. everyone (except all-in players) has the same bet (max of every valid player including
        //    all-in players)
        let max_bet = game_state
            .seats
            .iter()
            .filter_map(|seat| {
                if seat.is_valid() {
                    Some(seat.bet)
                } else {
                    None
                }
            })
            .max()
            .unwrap();
        let everyone_has_the_same_bet = game_state.seats.iter().all(|seat| seat.bet == max_bet);

        everyone_has_the_same_bet
    }

    fn next_round(game_state: &mut GameState) {
        todo!()
    }

    fn next_hand(game_state: &mut GameState) {
        // end current hand:
        //   - check for winner(s)
        //   - update winner(s) stack(s)
        //   - set dead flags

        // next_hand:
        //   - update round
        game_state.round = Round::PreFlop;
        //   - change sb
        game_state.sb_seat = game_state.next_valid_seat(game_state.sb_seat);
        //   - change current player
        game_state.current_seat = game_state.sb_seat;
        //   - deal new hand
        todo!()
    }

    fn is_hand_over(&self) -> bool {
        Self::is_round_over(&self.game_state) && self.game_state.round == Round::River
    }

    fn state_logic(&mut self) {
        if self.is_hand_over() {
            Self::next_hand(&mut self.game_state);
            return;
        }

        if Self::is_round_over(&self.game_state) {
            Self::next_round(&mut self.game_state);
            return;
        }

        self.next_turn();

        todo!()
    }

    fn handle_action(&mut self, action: Action) {
        // todo: check action validity
        match action {
            Action::Fold => self.game_state.seats[self.game_state.current_seat].is_folded = true,
            Action::Raise(amount) => {
                self.game_state.seats[self.game_state.current_seat].bet += amount
            }
            Action::Call => todo!(),
            Action::Check => {}
        }
    }

    pub fn current_seat(&self) -> usize {
        self.game_state.current_seat
    }

    fn get_observable_state(&self) -> ObservableState {
        self.into()
    }

    pub fn over(&self) -> bool {
        false
    }
}

struct ObservableState {}

impl From<&Game> for ObservableState {
    fn from(value: &Game) -> Self {
        Self {}
    }
}

#[derive(Clone, Copy)]
pub enum Action {
    Fold,
    Raise(usize),
    Call,
    Check,
}

#[test]
fn functional_test() -> Result<(), GameError> {
    let n = 3;
    let settings = Settings {
        n_players: n,
        initial_stack: 1000,
    };
    let players = vec![Player::new(); n];
    let mut game = Game::new(settings);

    while !game.over() {
        let seat_number = game.current_seat();

        let mut player = players[seat_number];
        let action = player.choose_action();
        game.play_turn(action)?;
    }

    Ok(())
}
