use crate::model::GameState;
use crate::movement::Movement;

pub enum Command {
    Undo,
    Resign,
    Save,
    CastleKingSide,
    CastleQueenSide,
    Move(Movement),
}

pub trait Controller {
    fn choose_command(&self, game_state: &mut GameState) -> Command;
}
