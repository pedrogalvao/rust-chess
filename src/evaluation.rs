use crate::model::{Color, GameState, PieceType};
use crate::rules::game_over::{is_draw, is_in_check_mate};

const KING_VALUE : i32 = 100;
const QUEEN_VALUE : i32 = 40;
const KNIGHT_VALUE : i32 = 12;
const BISHOP_VALUE : i32 = 13;
const ROOK_VALUE : i32 = 20;
const PAWN_VALUE : i32 = 4;


pub fn evaluate_material(game_state: &GameState, player_color: Color) -> i32 {
    let mut score = 0;
    for x in 0..8 {
        for y in 0..8 {
            if let Some(piece) = game_state.board[x][y] {
                let value = match piece.piece_type {
                    PieceType::King => KING_VALUE,
                    PieceType::Queen => QUEEN_VALUE,
                    PieceType::Rook => ROOK_VALUE,
                    PieceType::Bishop => BISHOP_VALUE,
                    PieceType::Knight => KNIGHT_VALUE,
                    PieceType::Pawn => PAWN_VALUE,
                };

                if piece.color == player_color {
                    score += value;
                } else {
                    score -= value;
                }
            }
        }
    }
    return score;
}

pub fn evaluate_positions(game_state: &GameState, player_color: Color) -> i32 {
    let mut score = 0;
    let first_row = match player_color {
        Color::White => 0,
        Color::Black => 7
    };
    for x in 0..8 {
        for y in 0..8 {
            if let Some(piece) = game_state.board[x][y] {
                let value = match piece.piece_type {
                    PieceType::King => {
                        if x == first_row {
                            KING_VALUE
                        } else {
                            KING_VALUE - 1
                        }
                    },
                    PieceType::Queen => QUEEN_VALUE,
                    PieceType::Rook => ROOK_VALUE,
                    PieceType::Bishop => {
                        if x == first_row {
                            BISHOP_VALUE - 1
                        } else {
                            BISHOP_VALUE
                        }
                    },
                    PieceType::Knight => {
                        if x == first_row {
                            KNIGHT_VALUE - 1
                        } else {
                            KNIGHT_VALUE
                        }
                    },
                    PieceType::Pawn => PAWN_VALUE // + ((x).abs_diff(first_row) as i32),
                };

                if piece.color == player_color {
                    score += value;
                } else {
                    score -= value;
                }
            }
        }
    }
    return score;
}

pub fn evaluate_game_over(game_state: &GameState, player_color: Color) -> i32 {
    if is_draw(game_state) {
        0
    } else if is_in_check_mate(game_state, player_color) {
        200
    } else if is_in_check_mate(game_state, player_color.get_opponent_color()) {
        200
    } else {
        0
    }
}