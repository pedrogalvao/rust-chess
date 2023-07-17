use crate::model::{Color, GameState, PieceType};

pub fn evaluate_state(game_state: &GameState, player_color: Color) -> i32 {
    let mut score = 0;
    for x in 0..8 {
        for y in 0..8 {
            if let Some(piece) = game_state.board[x][y] {
                let value = match piece.piece_type {
                    PieceType::King => 100,
                    PieceType::Queen => 10,
                    PieceType::Rook => 5,
                    PieceType::Bishop => 3,
                    PieceType::Knight => 3,
                    PieceType::Pawn => 1,
                };

                if piece.color == player_color {
                    score += value;
                } else {
                    score -= value;
                }
            }
        }
    }
    dbg!(score);
    return score;
}
