use itertools::Itertools;
use rand::{Rng, RngExt, rngs::SmallRng};
use simulator::{card::hand::PokerHand, round::Round};
use std::{fmt::Debug, time::Instant};

const N_BINS: usize = 10;
const PRE_FLOP_BIN_COUNT: usize = N_BINS;
const FLOP_BIN_COUNT: usize = N_BINS * PRE_FLOP_BIN_COUNT;
const TURN_BIN_COUNT: usize = N_BINS * FLOP_BIN_COUNT;
const RIVER_BIN_COUNT: usize = N_BINS * TURN_BIN_COUNT;
// raise amount in pot proportion
const RAISE_AMOUNT: f32 = 0.3;
const N_SAMPLES: usize = 1000;

pub trait Randomizable {
    fn random(rng: &mut impl Rng) -> Self;
}

#[derive(Clone, Debug)]
struct ActionProba {
    raise_p: u8,
    call_check_p: u8,
}

impl Randomizable for ActionProba {
    fn random(rng: &mut impl Rng) -> Self {
        let x: f32 = rng.random();
        let y: f32 = rng.random();
        let z: f32 = rng.random();
        let total = x + y + z;
        Self {
            raise_p: (x / total * 255.0) as u8,
            call_check_p: (y / total * 255.0) as u8,
        }
    }
}

impl ActionProba {
    fn raise_p(&self) -> f32 {
        self.raise_p as f32 / 255.0
    }
    fn call_check_p(&self) -> f32 {
        self.call_check_p as f32 / 255.0
    }
    fn fold_p(&self) -> f32 {
        (255 - self.raise_p - self.call_check_p) as f32 / 255.0
    }
    fn action(
        &self,
        current_pot: usize,
        min_raise: usize,
        can_check: bool,
        rng: &mut impl Rng,
    ) -> simulator::action::Action {
        let r: f32 = rng.random();
        let call_check_thres = self.call_check_p();
        let raise_thres = self.raise_p() + call_check_thres;

        if r < call_check_thres {
            if can_check {
                simulator::action::Action::Check
            } else {
                simulator::action::Action::Call
            }
        } else if r < raise_thres {
            let raise_amount = (current_pot as f32 * RAISE_AMOUNT) as usize;
            simulator::action::Action::Raise(raise_amount.max(min_raise))
        } else {
            simulator::action::Action::Fold
        }
    }
}

pub struct ActionHistory {
    pub action: simulator::action::Action,
    pub player: usize,
    pub round: Round,
}

const TREE_DEPTH: usize = 4;
const DECISION_TREE_SIZE: usize = (TREE_DEPTH - 1) * 2 + 1;
const HISTORY_TREE_SIZE: usize = (TREE_DEPTH - 2) * 2 + 4;

fn row_index(action: &simulator::action::Action) -> usize {
    match action {
        simulator::action::Action::Raise(_) => 1,
        simulator::action::Action::Call => 0,
        simulator::action::Action::Check => 0,
        simulator::action::Action::Fold => panic!("unexpected action history"),
    }
}

#[derive(Clone, Debug)]
pub struct DecisionTree {
    data: [ActionProba; DECISION_TREE_SIZE],
}

impl Randomizable for DecisionTree {
    fn random(rng: &mut impl Rng) -> Self {
        Self {
            data: (0..DECISION_TREE_SIZE)
                .map(|_| ActionProba::random(rng))
                .collect_array()
                .unwrap(),
        }
    }
}

impl DecisionTree {
    fn node_from_round_history(&self, actions_history: &[&ActionHistory]) -> &ActionProba {
        let depth = actions_history.len();
        assert!(depth < TREE_DEPTH);
        if depth == 0 {
            return &self.data[0];
        }
        let row_idx = match actions_history.first().unwrap().action {
            simulator::action::Action::Raise(_) => 1,
            simulator::action::Action::Call => 0,
            simulator::action::Action::Check => 0,
            simulator::action::Action::Fold => panic!("unexpected action history"),
        };
        let index = depth * 2 + row_idx - 1;
        &self.data[index]
    }
}

#[derive(Clone, Debug)]
pub struct HistoryTree<T: Clone + Debug + Randomizable> {
    data: [T; HISTORY_TREE_SIZE],
}

impl<T: Clone + Debug + Randomizable> Randomizable for HistoryTree<T> {
    fn random(rng: &mut impl Rng) -> Self {
        Self {
            data: (0..HISTORY_TREE_SIZE)
                .map(|_| T::random(rng))
                .collect_array()
                .unwrap(),
        }
    }
}

impl<T: Clone + Debug + Randomizable> HistoryTree<T> {
    fn node_from_round_history(&self, actions_history: &[&ActionHistory]) -> &T {
        let depth = actions_history.len();
        assert!(depth <= TREE_DEPTH);
        assert!(depth >= 2);

        let first_row_idx = row_index(&actions_history.first().unwrap().action);

        let index = if depth < TREE_DEPTH {
            2 * (depth - 2) + first_row_idx
        } else {
            let last_row_idx = row_index(&actions_history.last().unwrap().action);
            4 + first_row_idx * 2 + last_row_idx
        };

        &self.data[index]
    }
}

#[derive(Clone)]
pub struct AgentData {
    pub pre_flop: Vec<DecisionTree>,
    pub flop: Vec<HistoryTree<DecisionTree>>,
    pub turn: Vec<HistoryTree<HistoryTree<DecisionTree>>>,
    pub river: Vec<HistoryTree<HistoryTree<HistoryTree<DecisionTree>>>>,
}

#[derive(Clone)]
pub struct Agent {
    data: AgentData,
}

impl Randomizable for Agent {
    fn random(rng: &mut impl Rng) -> Self {
        let data = AgentData {
            pre_flop: (0..PRE_FLOP_BIN_COUNT)
                .map(|_| DecisionTree::random(rng))
                .collect_vec(),
            flop: (0..FLOP_BIN_COUNT)
                .map(|_| HistoryTree::<DecisionTree>::random(rng))
                .collect_vec(),
            turn: (0..TURN_BIN_COUNT)
                .map(|_| HistoryTree::<HistoryTree<DecisionTree>>::random(rng))
                .collect_vec(),
            river: (0..RIVER_BIN_COUNT)
                .map(|_| HistoryTree::<HistoryTree<HistoryTree<DecisionTree>>>::random(rng))
                .collect_vec(),
        };
        Agent { data }
    }
}

impl Agent {
    pub fn choose_action(
        &self,
        observable_game_state: &simulator::game::ObservableState,
        actions_history: &[ActionHistory],
        rng: &mut impl Rng,
    ) -> simulator::action::Action {
        let bin = hand_strength(observable_game_state, N_SAMPLES, N_BINS);

        let mut actions_grouped: Vec<Vec<&ActionHistory>> = Vec::new();
        for (_, chunk) in &actions_history.iter().chunk_by(|a| &a.round) {
            actions_grouped.push(chunk.collect());
        }

        let probas = match observable_game_state.round {
            Round::PreFlop => self.data.pre_flop[bin].node_from_round_history(&actions_grouped[0]),
            Round::Flop => self.data.flop[bin]
                .node_from_round_history(&actions_grouped[0])
                .node_from_round_history(&actions_grouped[1]),
            Round::Turn => self.data.turn[bin]
                .node_from_round_history(&actions_grouped[0])
                .node_from_round_history(&actions_grouped[1])
                .node_from_round_history(&actions_grouped[2]),
            Round::River => self.data.river[bin]
                .node_from_round_history(&actions_grouped[0])
                .node_from_round_history(&actions_grouped[1])
                .node_from_round_history(&actions_grouped[2])
                .node_from_round_history(&actions_grouped[3]),
        };

        let current_pot = observable_game_state.current_pot;
        let min_raise = observable_game_state.min_raise;
        let can_check = observable_game_state.seats.iter().all(|b| {
            !b.is_in_current_round || b.round_bet <= observable_game_state.current_player.round_bet
        });
        probas.action(current_pot, min_raise, can_check, rng)
    }
}

pub fn hand_strength(
    observable_game_state: &simulator::game::ObservableState,
    n_bins: usize,
    n_samples: usize,
) -> usize {
    let my_player_hand = &observable_game_state.current_player.hand;
    let board = &observable_game_state.board;
    let n_wins: usize = (0..n_samples)
        .map(|_| PokerHand::random_hand_win(board, my_player_hand) as usize)
        .sum();

    let strength = (n_wins as f32) / ((n_samples + 1) as f32);
    (n_bins as f32 * strength).floor() as usize
}

#[test]
fn random_agent() {
    // let mut rng = rand::rng();
    let mut rng: SmallRng = rand::make_rng();
    let start = Instant::now();
    let _ = Agent::random(&mut rng);
    let duration = start.elapsed();
    println!("Duration: {} ms", duration.as_millis());
}
