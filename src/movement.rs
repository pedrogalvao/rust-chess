use crate::model::{GameState, Piece, PieceType, Color};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Movement {
    CastleKingSide(Color),
    CastleQueenSide(Color),
    Normal {from: [usize; 2], to: [usize; 2]},
}

impl Movement {
    pub fn get_piece(&self, game_state: &GameState) -> Piece {
        match self {
            Movement::Normal { from, .. } => {
                let [x, y] = from;
                if let Some(piece) = game_state.board[*x][*y] {
                    piece
                } else {
                    // Invalid movement. No piece
                    panic!("Invalid movement. No piece at the source square.");
                }
            }
            Movement::CastleKingSide(color) | Movement::CastleQueenSide(color) => {
                return Piece {piece_type: PieceType::King, color: *color};
            }
        }
    }
}
