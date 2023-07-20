use crate::control::control::Command;
use crate::model::{write_game_state_to_json, GameState};

pub fn execute_command(cmd: Command, game_state: &mut GameState) {
    match cmd {
        Command::Move(movement) => game_state.make_movement(movement),
        Command::Resign => todo!(),
        Command::Save => {
            let _ = write_game_state_to_json(game_state, "game.json").unwrap();
        }
        Command::Undo => todo!(),
    }
}
