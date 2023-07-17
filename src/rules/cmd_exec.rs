use crate::control::control::Command;
use crate::model::GameState;

pub fn execute_command(cmd: Command, game_state: &mut GameState) {
    match cmd {
        Command::CastleKingSide => game_state.castle_king_side(),
        Command::CastleQueenSide => game_state.castle_queen_side(),
        Command::Move(movement) => game_state.make_movement(movement),
        Command::Resign => todo!(),
        Command::Save => todo!(),
        Command::Undo => todo!(),
    }
}
