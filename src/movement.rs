use crate::model::{GameState, Piece};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Movement {
    pub source: [usize; 2],
    pub destination: [usize; 2],
}


impl Movement {
    pub fn get_piece(&self, game_state: &GameState) -> Piece {
        let [x, y] = self.source;
        if let Some(piece) = game_state.board[x][y] {
            return piece;
        } else {
            todo!()
        }
    }
}
