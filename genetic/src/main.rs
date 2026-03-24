pub mod agent;

use agent::Agent;

struct GeneticSettings {
    agents_per_gen: usize,
    generations: usize,
    games_per_gen: usize,
}

fn generate_agents(count: usize) -> Vec<Agent> {
    (0..count).map(|_| Agent::random_agent()).collect()
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
    todo!()
}

fn update_agent_info(agent0_info: &mut [AgentInfo], pairing: &Pairing, game_result: &GameResult) {
    todo!()
}

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

    let mut agents = generate_agents(genetic_settings.agents_per_gen);

    for _ in 0..genetic_settings.generations {
        let mut agents_info: Vec<AgentInfo> = todo!();
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
