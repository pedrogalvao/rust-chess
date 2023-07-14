use crate::model::GameState;
use crate::movement::Movement;

// enum Player {
//     LocalHuman,
//     RemoteHuman,
//     Bot
// }

pub enum Command {
    Undo,
    Resign,
    Save,
    Move(Movement),
}

pub trait Controller {
    fn control(&self, game_state: &mut GameState);
}
