use crate::model::game_state::GameState;
use crate::model::movement::Movement;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum Command {
    Undo,
    Resign,
    Save,
    Move(Movement),
}

/// Interface for objects responsible for the choices of one player.
pub trait Controller {
    fn choose_command(&mut self, game_state: &mut GameState) -> Command;
}
