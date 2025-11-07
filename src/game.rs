use crate::*;

use thiserror::Error;

#[derive(Error, Debug)]
enum GameError {
    #[error("Action is invalid")]
    InvalidAction,
}

pub struct Game {
    settings: Settings,

    round_count: usize,
    seats: Vec<Seat>,
    current_seat: usize,
    pot: usize,
}

#[derive(Clone, Copy)]
pub struct Seat {
    pub stack: usize,
    pub bet: usize,
    pub is_folded: bool,
    pub is_dead: bool,
}

impl Seat {
    fn new(stack: usize) -> Self {
        Seat {
            stack,
            bet: 0,
            is_folded: false,
            is_dead: false,
        }
    }
}

#[derive(Default)]
pub struct Settings {
    n_players: usize,
    initial_stack: usize,
}

impl Game {
    fn new(settings: Settings) -> Game {
        Game {
            seats: vec![Seat::new(settings.initial_stack); settings.n_players],
            settings,
            round_count: 0,
            current_seat: 0,
            pot: 0,
        }
    }

    fn play_action(&mut self, action: Action) -> Result<(), GameError> {
        self.handle_action(action);
        self.state_logic();

        Ok(())
    }

    fn is_current_seat_valid(&self) -> bool {
        !(self.seats[self.current_seat].is_dead || self.seats[self.current_seat].is_folded)
    }

    fn advance_player(&mut self) {
        loop {
            self.current_seat = (self.current_seat + 1) % self.settings.n_players;
            if self.is_current_seat_valid() {
                break;
            }
        }
    }

    fn state_logic(&mut self) {
        self.advance_player();

        todo!()
    }

    fn handle_action(&mut self, action: Action) {
        // todo: check action validity
        match action {
            Action::Fold => self.seats[self.current_seat].is_folded = true,
            Action::Raise(amount) => self.seats[self.current_seat].bet += amount,
            Action::Call => todo!(),
            Action::Check => {}
        }
    }

    fn current_seat(&self) -> usize {
        self.current_seat
    }

    fn get_observable_state(&self) -> ObservableState {
        self.into()
    }

    fn over(&self) -> bool {
        false
    }
}

struct ObservableState {}

impl From<&Game> for ObservableState {
    fn from(value: &Game) -> Self {
        Self {}
    }
}

enum Action {
    Fold,
    Raise(usize),
    Call,
    Check,
}

#[derive(Clone, Copy)]
struct Player {}

impl Player {
    fn new() -> Self {
        Player {}
    }
    fn choose_action(&mut self) -> Action {
        Action::Check
    }
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
        game.play_action(action)?;
    }

    Ok(())
}
