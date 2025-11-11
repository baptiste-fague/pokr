mod deck;
mod game;
mod player;
mod round;
mod turn;

use game::*;
use player::*;

fn main() -> Result<(), GameError> {
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
