use crate::model::GameState;
use crate::movement::Movement;
use crate::rules::{king_castle_is_valid, queen_castle_is_valid};

// enum Player {
//     LocalHuman,
//     RemoteHuman,
//     Bot
// }

pub enum Command {
    Undo,
    Resign,
    Save,
    CastleKingSide,
    CastleQueenSide,
    Move(Movement),
}

pub trait Controller {

    fn execute_command(&self, game_state: &mut GameState, cmd: Command) {
        match cmd {
            Command::CastleKingSide => {
                if king_castle_is_valid(game_state) {
                    game_state.castle_king_side();
                }
            },
            Command::CastleQueenSide => {
                if queen_castle_is_valid(game_state) {
                    game_state.castle_queen_side();
                }
            }
            Command::Move(movement) => {
                game_state.make_movement(movement);
            },
            Command::Resign => todo!(),
            Command::Save => todo!(),
            Command::Undo => todo!(),
        }
    }

    fn control(&self, game_state: &mut GameState);
}
