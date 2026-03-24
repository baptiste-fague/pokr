use log::debug;

use crate::action::Action;
use crate::card::*;
use crate::game_state::{GameState, Seat};
use crate::hand::Hand;
use crate::round::Round;
use crate::*;

pub struct GameData {
    hand_count: usize,
}

pub struct Game {
    settings: Settings,
    game_state: GameState,
    game_data: GameData,
}

#[derive(Default, Clone)]
pub struct Settings {
    pub n_players: usize,
    pub initial_stack: usize,
    pub small_blind: usize,
    pub max_hands: usize,
}

impl Game {
    pub fn new(settings: Settings) -> Result<Self, GameError> {
        let mut game_state = GameState {
            current_seat: 0,
            current_bet: 0,
            current_raise: 0,
            deck: Deck::new(),
            board: Board::new(),
            seats: vec![Seat::new(settings.initial_stack); settings.n_players],
            sb_seat: 0,
            round: Round::PreFlop,
        };
        Hand::start(&mut game_state, settings.small_blind)?;

        Ok(Self {
            game_state,
            settings,
            game_data: GameData { hand_count: 0 },
        })
    }

    pub fn from_game_state(game_state: GameState, settings: Settings) -> Self {
        Self {
            settings,
            game_state,
            game_data: GameData { hand_count: 0 },
        }
    }

    pub fn play_turn(&mut self, action: Action) -> Result<(), GameError> {
        self.handle_action(action)?;
        self.state_logic()
    }

    /// Go to next valid seat
    fn next_turn(&mut self) {
        self.game_state.current_seat = self
            .game_state
            .next_playing_seat(self.game_state.current_seat);
    }

    fn state_logic(&mut self) -> Result<(), GameError> {
        debug!("state logic");

        // Round logic
        if !Round::is_over(&self.game_state) {
            self.next_turn();
            return Ok(());
        }

        debug!("round is over");
        Round::finish(&mut self.game_state);

        // Hand logic
        if !Hand::is_over(&self.game_state) {
            // start next round
            self.game_state.round = self.game_state.round.next_round_name()?;
            return Round::start(&mut self.game_state);
        }

        debug!("hand is over");
        Hand::finish(&mut self.game_state)?;
        self.game_data.hand_count += 1;

        // Game logic
        if self.is_over() {
            debug!("game is over");
            return Ok(());
        }

        self.game_state.sb_seat = self.game_state.next_playing_seat(self.game_state.sb_seat);
        Hand::start(&mut self.game_state, self.settings.small_blind)
    }

    fn handle_action(&mut self, action: Action) -> Result<(), GameError> {
        match action {
            Action::Fold => Action::fold(&mut self.game_state),
            Action::Raise(amount_to_bet) => Action::raise(&mut self.game_state, amount_to_bet)?,
            Action::Call => Action::call(&mut self.game_state),
            Action::Check => Action::check(&mut self.game_state)?,
        }
        self.game_state.current_seat_mut().played_this_round = true;
        Ok(())
    }

    pub fn current_seat(&self) -> usize {
        self.game_state.current_seat
    }

    pub fn get_observable_state(&self) -> ObservableState {
        self.into()
    }

    pub fn is_over(&self) -> bool {
        self.game_state
            .seats
            .iter()
            .filter(|seat| !seat.is_dead)
            .count()
            == 1
            || self.game_data.hand_count >= self.settings.max_hands
    }

    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }
}

pub struct ObservableState {}

impl From<&Game> for ObservableState {
    fn from(_value: &Game) -> Self {
        Self {}
    }
}

#[test]
fn functional_test() -> Result<(), GameError> {
    let n = 3;
    let settings = Settings {
        n_players: n,
        initial_stack: 1000,
        small_blind: 10,
        max_hands: 1000,
    };
    let players = vec![Player::new(); n];
    let mut game = Game::new(settings)?;

    while !game.is_over() {
        let seat_number = game.current_seat();

        let mut player = players[seat_number];
        let action = player.choose_action();
        game.play_turn(action)?;
    }

    Ok(())
}

#[test]
fn unit_test() -> Result<(), GameError> {
    let n = 3;
    let settings = Settings {
        n_players: n,
        initial_stack: 1000,
        small_blind: 10,
        max_hands: 1000,
    };
    let game_state = GameState {
        current_seat: 2,
        current_bet: 0,
        current_raise: 0,
        deck: Deck::empty(),
        board: Board::from_cards(vec![
            Card {
                suit: Suit::Clubs,
                value: Value::King,
            },
            Card {
                suit: Suit::Hearts,
                value: Value::King,
            },
            Card {
                suit: Suit::Spades,
                value: Value::Queen,
            },
            Card {
                suit: Suit::Clubs,
                value: Value::Ace,
            },
            Card {
                suit: Suit::Hearts,
                value: Value::Ace,
            },
        ]),
        seats: vec![
            Seat {
                hand: Some(PlayerHand {
                    cards: [
                        Card {
                            suit: Suit::Clubs,
                            value: Value::Seven,
                        },
                        Card {
                            suit: Suit::Diamonds,
                            value: Value::Seven,
                        },
                    ],
                }),
                stack: 0,
                round_bet: 0,
                total_bet: 1000,
                is_folded: false,
                is_dead: false,
                played_this_round: false,
            },
            Seat {
                hand: Some(PlayerHand {
                    cards: [
                        Card {
                            suit: Suit::Diamonds,
                            value: Value::King,
                        },
                        Card {
                            suit: Suit::Hearts,
                            value: Value::Seven,
                        },
                    ],
                }),
                stack: 0,
                round_bet: 0,
                total_bet: 1000,
                is_folded: false,
                is_dead: false,
                played_this_round: false,
            },
            Seat {
                hand: Some(PlayerHand {
                    cards: [
                        Card {
                            suit: Suit::Spades,
                            value: Value::Ace,
                        },
                        Card {
                            suit: Suit::Spades,
                            value: Value::Seven,
                        },
                    ],
                }),
                stack: 100,
                round_bet: 0,
                total_bet: 1000,
                is_folded: false,
                is_dead: false,
                played_this_round: false,
            },
        ],
        sb_seat: 0,
        round: Round::River,
    };

    let mut game = Game::from_game_state(game_state, settings);

    game.play_turn(Action::Raise(100))?;
    assert!(game.is_over());
    assert_eq!(game.game_state.seats[2].stack, 3100);

    Ok(())
}
