use crate::model::{game_state::GameState, piece::Color, piece::PieceType};
use crate::rules::cmd_validator::is_in_check;
use crate::rules::move_generator::generate_movements;
// use crate::move_generator::generate_movements_for_player;

pub fn is_in_check_mate(game_state: &GameState, player_color: Color) -> bool {
    is_in_check(game_state, player_color) && generate_movements(game_state).len() == 0
}

fn has_insufficient_material(game_state: &GameState) -> bool {
    let mut white_piece_count: u8 = 0;
    let mut black_piece_count: u8 = 0;
    for x in 0..8 {
        for y in 0..8 {
            if let Some(piece) = game_state.board[x][y] {
                match piece.piece_type {
                    PieceType::Rook | PieceType::Queen | PieceType::Pawn => {
                        return false;
                    }
                    PieceType::Bishop | PieceType::Knight => match piece.color {
                        Color::White => {
                            white_piece_count += 1;
                        }
                        Color::Black => {
                            black_piece_count += 1;
                        }
                    },
                    _ => {
                        continue;
                    }
                }
            }
        }
    }
    if white_piece_count <= 1 && black_piece_count <= 1 {
        return true;
    } else {
        return false;
    }
}

pub fn is_draw(game_state: &GameState) -> bool {
    !is_in_check(game_state, game_state.player_to_move)
        && ((!is_in_check(game_state, game_state.player_to_move.get_opponent_color())
            && generate_movements(game_state).len() == 0)
            || has_insufficient_material(game_state))
}

#[allow(dead_code)]
pub fn is_game_over(game_state: &GameState) -> bool {
    if is_in_check(game_state, game_state.player_to_move) {
        if is_in_check_mate(game_state, game_state.player_to_move) {
            return true;
        }
    } else if is_draw(game_state) {
        return true;
    }
    return false;
}
