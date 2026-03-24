pub mod agent;

use std::default;

use agent::Agent;
use rand::Rng;

struct GeneticSettings {
    agents_per_gen: usize,
    generations: usize,
    games_per_gen: usize,
}

fn generate_agents(count: usize, rng: &mut impl Rng) -> Vec<Agent> {
    (0..count).map(|_| Agent::random_agent(rng)).collect()
}

struct Pairing {
    pub player0: usize,
    pub player1: usize,
}

fn pair_agents(count: usize) -> Vec<Pairing> {
    todo!()
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
    todo!()
}

#[derive(Default, Clone)]
struct AgentInfo {
    agent_idx: usize,
    win_count: usize,
}

fn kill_the_weak(
    mut agents: Vec<Agent>,
    agents_info: &[AgentInfo],
    genetic_settings: &GeneticSettings,
) -> Vec<Agent> {
    todo!()
}

fn reproduce(mut agents: Vec<Agent>, genetic_settings: &GeneticSettings) -> Vec<Agent> {
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
    };

    let mut rng = rand::rng();
    let mut agents = generate_agents(genetic_settings.agents_per_gen, &mut rng);

    for _ in 0..genetic_settings.generations {
        let mut agents_info = vec![AgentInfo::default(); genetic_settings.agents_per_gen];
        for _ in 0..genetic_settings.games_per_gen {
            let pairings = pair_agents(agents.len());
            for pairing in pairings {
                let (agent0, agent1) = (&agents[pairing.player0], &agents[pairing.player1]);
                let game_result = fight(agent0, agent1, &game_settings);
                update_agent_info(&mut agents_info, &pairing, &game_result);
            }
        }
        let survivors = kill_the_weak(agents, &agents_info, &genetic_settings);
        agents = reproduce(survivors, &genetic_settings);
    }
}
