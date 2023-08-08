use crate::model::{
    game_state::GameState,
    movement::Movement,
    piece::{Color, PieceType},
};

use super::cmd_validator::is_in_check;

fn castle_king_side_destination_is_free(game_state: &GameState) -> bool {
    let king_row = game_state
        .get_king_position(game_state.player_to_move)
        .unwrap()[0];
    for col in [5, 6] {
        if game_state.board[king_row][col] != None {
            // TODO: Edge case when the other rook is in the king's destination
            if game_state.board[king_row][col].unwrap().piece_type != PieceType::King
                && game_state.board[king_row][col].unwrap().piece_type != PieceType::Rook
            {
                return false;
            }
        }
    }
    true
}

fn castle_queen_side_destination_is_free(game_state: &GameState) -> bool {
    let king_row = game_state
        .get_king_position(game_state.player_to_move)
        .unwrap()[0];
    for col in [2, 3] {
        if game_state.board[king_row][col] != None {
            // TODO: Edge case when the other rook is in the king's destination
            if game_state.board[king_row][col].unwrap().piece_type != PieceType::King
                && game_state.board[king_row][col].unwrap().piece_type != PieceType::Rook
            {
                return false;
            }
        }
    }
    true
}

pub fn king_castle_is_valid(game_state: &GameState) -> bool {
    if (game_state.player_to_move == Color::White && !game_state.white_can_castle_king_side)
        || (game_state.player_to_move == Color::Black && !game_state.black_can_castle_king_side)
    {
        return false;
    }
    if game_state.get_king_position(game_state.player_to_move) == None {
        return false;
    }
    if free_space_between_rook_and_king(game_state, true)
        && castle_king_side_destination_is_free(game_state)
    {
        let game_state2 =
            game_state.clone_and_move(Movement::CastleKingSide(game_state.player_to_move));
        if is_in_check(game_state, game_state.player_to_move) {
            return false;
        }
        if is_in_check(&game_state2, game_state.player_to_move) {
            return false;
        }
        return true;
    }
    false
}

pub fn queen_castle_is_valid(game_state: &GameState) -> bool {
    if (game_state.player_to_move == Color::White && !game_state.white_can_castle_queen_side)
        || (game_state.player_to_move == Color::Black && !game_state.black_can_castle_queen_side)
    {
        return false;
    }
    if game_state.get_king_position(game_state.player_to_move) == None {
        return false;
    }

    if free_space_between_rook_and_king(game_state, false)
        && castle_queen_side_destination_is_free(game_state)
    {
        let game_state2 =
            game_state.clone_and_move(Movement::CastleQueenSide(game_state.player_to_move));
        if is_in_check(&game_state2, game_state.player_to_move) {
            return false;
        }
        if is_in_check(game_state, game_state.player_to_move) {
            return false;
        }
        return true;
    }
    false
}

fn free_space_between_rook_and_king(game_state: &GameState, king_side: bool) -> bool {
    let mut rook_column = 0;
    let Some([_, king_col]) = game_state.get_king_position(game_state.player_to_move) else {
        return false;
    };
    if king_side {
        for col in king_col + 1..8 {
            match game_state.board[0][col] {
                Some(piece) if piece.piece_type == PieceType::Rook => {
                    rook_column = col;
                    break;
                }
                _ => {}
            }
        }
    } else {
        for col in 0..king_col {
            match game_state.board[0][col] {
                Some(piece) if piece.piece_type == PieceType::Rook => {
                    rook_column = col;
                    break;
                }
                _ => {}
            }
        }
    }
    if king_col < rook_column {
        for i in king_col + 1..rook_column {
            if game_state.board[king_col][i] != None {
                return false;
            }
        }
    } else {
        for i in rook_column + 1..king_col {
            if game_state.board[king_col][i] != None {
                return false;
            }
        }
    }
    true
}
