use crate::model::GameState;
use crate::movement::Movement;

#[derive(Clone)]
pub enum Command {
    Undo,
    Resign,
    Save,
    CastleKingSide,
    CastleQueenSide,
    Move(Movement),
}

pub trait Controller {
    fn choose_command(&mut self, game_state: &mut GameState) -> Command;
}
