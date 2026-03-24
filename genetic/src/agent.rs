use itertools::Itertools;
use rand::{Rng, RngExt};

const VALUES_PER_CARD_SEQUENCE: usize = 3_usize.pow(4_u32);
const N_BINS: usize = 10;
const PRE_FLOP_BIN_COUNT: usize = N_BINS;
const FLOP_BIN_COUNT: usize = N_BINS * PRE_FLOP_BIN_COUNT;
const TURN_BIN_COUNT: usize = N_BINS * FLOP_BIN_COUNT;
const RIVER_BIN_COUNT: usize = N_BINS * TURN_BIN_COUNT;

#[derive(Clone)]
struct ActionProba {
    raise_p: f32,
    call_check_p: f32,
}

impl ActionProba {
    fn random(rng: &mut impl Rng) -> Self {
        let x: f32 = rng.random();
        let y: f32 = rng.random();
        let z: f32 = rng.random();
        let total = x + y + z;
        Self {
            raise_p: x / total,
            call_check_p: y / total,
        }
    }
    fn raise_p(&self) -> f32 {
        self.raise_p
    }
    fn call_check_p(&self) -> f32 {
        self.call_check_p
    }
    fn fold_p(&self) -> f32 {
        1.0 - self.raise_p - self.call_check_p
    }
}

#[derive(Clone)]
pub struct Agent {
    pre_flop_data: [ActionProba; PRE_FLOP_BIN_COUNT * VALUES_PER_CARD_SEQUENCE],
    flop_data: [ActionProba; FLOP_BIN_COUNT * VALUES_PER_CARD_SEQUENCE],
    turn_data: [ActionProba; TURN_BIN_COUNT * VALUES_PER_CARD_SEQUENCE],
    river_data: [ActionProba; RIVER_BIN_COUNT * VALUES_PER_CARD_SEQUENCE],
}

impl Agent {
    pub fn random_agent(rng: &mut impl Rng) -> Agent {
        Agent {
            pre_flop_data: (0..PRE_FLOP_BIN_COUNT * VALUES_PER_CARD_SEQUENCE)
                .map(|_| ActionProba::random(rng))
                .collect_array()
                .unwrap(),
            flop_data: (0..FLOP_BIN_COUNT * VALUES_PER_CARD_SEQUENCE)
                .map(|_| ActionProba::random(rng))
                .collect_array()
                .unwrap(),
            turn_data: (0..TURN_BIN_COUNT * VALUES_PER_CARD_SEQUENCE)
                .map(|_| ActionProba::random(rng))
                .collect_array()
                .unwrap(),
            river_data: (0..RIVER_BIN_COUNT * VALUES_PER_CARD_SEQUENCE)
                .map(|_| ActionProba::random(rng))
                .collect_array()
                .unwrap(),
        }
    }

    pub fn play_action(
        &self,
        observable_game_state: &simulator::game::ObservableState,
    ) -> simulator::action::Action {
        todo!()
    }
}
