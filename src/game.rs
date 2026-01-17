use itertools::Itertools;
use log::debug;

use crate::card::*;
use crate::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Round {
    PreFlop,
    Flop,
    Turn,
    River,
}

impl Round {
    fn next_round_name(&self) -> Result<Self, GameError> {
        match self {
            Round::PreFlop => Ok(Round::Flop),
            Round::Flop => Ok(Round::Turn),
            Round::Turn => Ok(Round::River),
            Round::River => Err(GameError::InvalidRoundCall),
        }
    }
}

#[derive(Debug)]
pub struct GameState {
    current_seat: usize,
    /// maximum total amount bet by a player so far during the hand
    current_bet: usize,
    /// maximum raise (amount_bet - amount_bet_by_previous_player) so far during the hand
    current_raise: usize,

    deck: Deck,
    board: Board,

    seats: Vec<Seat>,
    sb_seat: usize,

    round: Round,
}

impl GameState {
    fn next_playing_seat(&self, seat: usize) -> usize {
        let mut next_seat = seat;
        loop {
            next_seat = (next_seat + 1) % self.seats.len();
            if self.seats[next_seat].can_play() {
                break;
            }
        }
        next_seat
    }
}

pub struct GameData {
    hand_count: usize,
}

pub struct Game {
    settings: Settings,
    game_state: GameState,
    game_data: GameData,
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
    fn new(stack: usize) -> Self {
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

    fn is_valid(&self) -> bool {
        !self.is_dead && !self.is_folded
    }

    fn can_play(&self) -> bool {
        self.is_valid() && self.stack > 0
    }
}

#[derive(Default)]
pub struct Settings {
    pub n_players: usize,
    pub initial_stack: usize,
    pub small_blind: usize,
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
        Self::start_hand(&mut game_state, &settings)?;

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

    fn start_hand(game_state: &mut GameState, settings: &Settings) -> Result<(), GameError> {
        // update round
        game_state.round = Round::PreFlop;

        // change current player
        game_state.current_seat = game_state.sb_seat;

        // put blinds
        Self::raise(game_state, settings.small_blind)?;
        game_state.current_seat = game_state.next_playing_seat(game_state.current_seat);
        Self::raise(game_state, 2 * settings.small_blind)?;
        game_state.current_seat = game_state.next_playing_seat(game_state.current_seat);

        // deal new hand
        game_state.deck = Deck::new();
        game_state.seats.iter_mut().try_for_each(|seat| {
            seat.hand = Some(game_state.deck.draw_hand()?);
            Ok(())
        })?;

        Ok(())
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

    fn is_round_over(game_state: &GameState) -> bool {
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

    fn finish_round(game_state: &mut GameState) {
        game_state.current_bet = 0;
        game_state.current_raise = 0;

        // move round bet to total bet
        game_state.seats.iter_mut().for_each(|s| {
            s.total_bet += s.round_bet;
            s.round_bet = 0;
            s.played_this_round = false;
        });
    }

    fn start_round(game_state: &mut GameState) -> Result<(), GameError> {
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

    fn finish_hand(game_state: &mut GameState) -> Result<(), GameError> {
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

        // update seats states
        game_state.seats.iter_mut().for_each(|s| {
            s.is_dead = s.stack == 0;
            s.is_folded = false
        });

        Ok(())
    }

    fn is_hand_over(&self) -> bool {
        self.game_state.round == Round::River
    }

    fn state_logic(&mut self) -> Result<(), GameError> {
        debug!("state logic");

        if !Self::is_round_over(&self.game_state) {
            self.next_turn();
            return Ok(());
        }

        debug!("round is over");
        Self::finish_round(&mut self.game_state);

        if !self.is_hand_over() {
            // update round
            self.game_state.round = self.game_state.round.next_round_name()?;
            return Self::start_round(&mut self.game_state);
        }

        debug!("hand is over");
        Self::finish_hand(&mut self.game_state)?;

        if self.over() {
            debug!("game is over");
            return Ok(());
        }

        self.game_state.sb_seat = self.game_state.next_playing_seat(self.game_state.sb_seat);
        Self::start_hand(&mut self.game_state, &self.settings)
    }

    fn raise(game_state: &mut GameState, amount_to_bet: usize) -> Result<(), GameError> {
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

    fn handle_action(&mut self, action: Action) -> Result<(), GameError> {
        match action {
            Action::Fold => {
                let current_seat = &mut self.game_state.seats[self.game_state.current_seat];
                current_seat.is_folded = true;
            }
            Action::Raise(amount_to_bet) => {
                Self::raise(&mut self.game_state, amount_to_bet)?;
            }
            Action::Call => {
                let current_seat = &mut self.game_state.seats[self.game_state.current_seat];
                let amount_to_put = self
                    .game_state
                    .current_bet
                    .saturating_sub(current_seat.round_bet);
                current_seat.round_bet += amount_to_put;
                current_seat.stack -= amount_to_put;
            }
            Action::Check => {
                let current_seat = &mut self.game_state.seats[self.game_state.current_seat];
                if self.game_state.current_bet > current_seat.round_bet {
                    return Err(GameError::InvalidAction);
                }
            }
        }
        let current_seat = &mut self.game_state.seats[self.game_state.current_seat];
        current_seat.played_this_round = true;
        Ok(())
    }

    pub fn current_seat(&self) -> usize {
        self.game_state.current_seat
    }

    fn get_observable_state(&self) -> ObservableState {
        self.into()
    }

    pub fn over(&self) -> bool {
        self.game_state
            .seats
            .iter()
            .filter(|seat| !seat.is_dead)
            .count()
            == 1
    }
}

struct ObservableState {}

impl From<&Game> for ObservableState {
    fn from(_value: &Game) -> Self {
        Self {}
    }
}

#[derive(Clone, Copy)]
pub enum Action {
    Fold,
    Raise(usize), // takes the player's total_bet after the turn (not the additional money) as argument
    Call,
    Check,
}

#[test]
fn functional_test() -> Result<(), GameError> {
    let n = 3;
    let settings = Settings {
        n_players: n,
        initial_stack: 1000,
        small_blind: 10,
    };
    let players = vec![Player::new(); n];
    let mut game = Game::new(settings)?;

    while !game.over() {
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
    assert!(game.over());
    assert_eq!(game.game_state.seats[2].stack, 3100);

    Ok(())
}
