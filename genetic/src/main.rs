pub mod agent;

use agent::Agent;
use itertools::Itertools;
use rand::Rng;
use rand::seq::{IndexedRandom, SliceRandom};
use rand_distr::{Distribution, Normal};

struct GeneticSettings {
    agents_per_gen: usize,
    generations: usize,
    games_per_gen: usize,
    luck_factor: f32,
    survival_rate: f32,
    mutation_rate: f32,
    mutation_density: f32,
}

fn generate_agents(count: usize, rng: &mut impl Rng) -> Vec<Agent> {
    (0..count).map(|_| Agent::random_agent(rng)).collect()
}

struct Pairing {
    pub player0: usize,
    pub player1: usize,
}

fn pair_agents(count: usize, rng: &mut impl Rng) -> Vec<Pairing> {
    let mut indices: Vec<usize> = (0..count).collect();
    indices.shuffle(rng);
    indices
        .chunks(2)
        .map(|chunk| Pairing {
            player0: chunk[0],
            player1: chunk[1],
        })
        .collect()
}

struct GameResult {
    pub agent0_won: bool,
}

fn fight(agent0: &Agent, agent1: &Agent, game_settings: &simulator::game::Settings) -> GameResult {
    let mut game = simulator::game::Game::new(game_settings.clone()).unwrap();
    while !game.is_over() {
        let observable_state = game.get_observable_state();
        if game.current_seat() == 0 {
            let action = agent0.play_action(&observable_state);
            game.play_turn(action).unwrap();
        } else if game.current_seat() == 1 {
            let action = agent1.play_action(&observable_state);
            game.play_turn(action).unwrap();
        } else {
            panic!("Unexpected current seat")
        }
    }
    let agent0_won = game.get_game_state().seats[0].stack > game.get_game_state().seats[1].stack;
    GameResult { agent0_won }
}

fn update_agent_info(agent0_info: &mut [AgentInfo], pairing: &Pairing, game_result: &GameResult) {
    if game_result.agent0_won {
        agent0_info[pairing.player0].win_count += 1;
    } else {
        agent0_info[pairing.player1].win_count += 1;
    }
}

#[derive(Default, Clone)]
struct AgentInfo {
    agent_idx: usize,
    win_count: usize,
}

fn kill_the_weak(
    agents: Vec<Agent>,
    agents_info: &[AgentInfo],
    genetic_settings: &GeneticSettings,
    rng: &mut impl Rng,
) -> Vec<Agent> {
    let normal = Normal::new(0.0, 3.0).unwrap();
    let mut sorted_agents_info = agents_info.to_vec();
    sorted_agents_info.sort_by_key(|info| info.win_count);
    let mut sorted_agents_winrate = sorted_agents_info
        .iter()
        .map(|info| {
            (
                info.agent_idx,
                info.win_count as f32 / genetic_settings.games_per_gen as f32,
            )
        })
        .collect::<Vec<_>>();
    sorted_agents_winrate.iter_mut().for_each(|(_, win_rate)| {
        *win_rate = (*win_rate + normal.sample(rng) * genetic_settings.luck_factor).clamp(0.0, 1.0)
    });
    let survivors_count =
        (genetic_settings.agents_per_gen as f32 * genetic_settings.survival_rate) as usize;
    let survivors_info = &sorted_agents_info[genetic_settings.agents_per_gen - survivors_count..];
    survivors_info
        .iter()
        .map(|info| agents[info.agent_idx].clone())
        .collect()
}

fn reproduce(
    mut agents: Vec<Agent>,
    genetic_settings: &GeneticSettings,
    rng: &mut impl Rng,
) -> Vec<Agent> {
    agents.extend(
        agents
            .sample(rng, genetic_settings.agents_per_gen - agents.len())
            .cloned()
            .collect_vec(),
    );
    agents
}

fn mutate(mut agents: Vec<Agent>, genetic_settings: &GeneticSettings) -> Vec<Agent> {
    todo!()
}

fn main() {
    let game_settings = simulator::game::Settings {
        n_players: 2,
        initial_stack: 1000,
        small_blind: 10,
        max_hands: 1000,
    };

    let genetic_settings = GeneticSettings {
        agents_per_gen: 1000,
        generations: 1000,
        games_per_gen: 10,
        luck_factor: 0.05,
        survival_rate: 0.5,
        mutation_rate: 0.02,
        mutation_density: 0.1,
    };

    let mut rng = rand::rng();
    let mut agents = generate_agents(genetic_settings.agents_per_gen, &mut rng);

    for _ in 0..genetic_settings.generations {
        let mut agents_info = vec![AgentInfo::default(); genetic_settings.agents_per_gen];
        for _ in 0..genetic_settings.games_per_gen {
            let pairings = pair_agents(agents.len(), &mut rng);
            for pairing in pairings {
                let (agent0, agent1) = (&agents[pairing.player0], &agents[pairing.player1]);
                let game_result = fight(agent0, agent1, &game_settings);
                update_agent_info(&mut agents_info, &pairing, &game_result);
            }
        }
        let survivors = kill_the_weak(agents, &agents_info, &genetic_settings, &mut rng);
        agents = reproduce(survivors, &genetic_settings, &mut rng);
        agents = mutate(agents, &genetic_settings)
    }
}
