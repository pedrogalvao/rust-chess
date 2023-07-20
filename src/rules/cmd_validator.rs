use crate::control::control::Command;
use crate::model::{Color, GameState, Piece, PieceType};
use crate::movement::Movement;

fn is_valid_movement_for_rook(movement: &Movement, game_state: &GameState) -> bool {
    let Movement::Normal { from, to } = movement else {
        return false;
    };
    if from[0] == to[0] {
        let x = from[0];
        if from[1] < to[1] {
            for y in (from[1] + 1)..to[1] {
                if None != game_state.board[x][y] {
                    return false;
                }
            }
        } else {
            for y in (to[1] + 1)..from[1] {
                if None != game_state.board[x][y] {
                    return false;
                }
            }
        }
        return true;
    } else if from[1] == to[1] {
        let y = from[1];
        if from[0] < to[0] {
            for x in (from[0] + 1)..to[0] {
                if None != game_state.board[x][y] {
                    return false;
                }
            }
        } else {
            for x in (to[0] + 1)..from[0] {
                if None != game_state.board[x][y] {
                    return false;
                }
            }
        }
        return true;
    } else {
        false
    }
}

fn is_valid_movement_for_bishop(movement: &Movement, game_state: &GameState) -> bool {
    let Movement::Normal { from, to } = movement else {
        return false;
    };
    let dist_x: i8 = from[0] as i8 - to[0] as i8;
    let dist_y: i8 = from[1] as i8 - to[1] as i8;
    if dist_x.abs() != dist_y.abs() {
        false
    } else {
        let mut x = from[0];
        let mut y = from[1];
        loop {
            if from[0] < to[0] {
                x += 1;
            } else {
                x -= 1;
            }
            if from[1] < to[1] {
                y += 1;
            } else {
                y -= 1;
            }
            if x == to[0] {
                break;
            }
            if None != game_state.board[x][y] {
                return false;
            }
        }
        return true;
    }
}

fn is_valid_movement_for_queen(movement: &Movement, game_state: &GameState) -> bool {
    is_valid_movement_for_rook(&movement, game_state)
        || is_valid_movement_for_bishop(&movement, game_state)
}

fn is_valid_movement_for_knight(movement: &Movement) -> bool {
    let Movement::Normal { from, to } = movement else {
        return false;
    };
    match (
        (from[0] as i8 - to[0] as i8).abs(),
        (from[1] as i8 - to[1] as i8).abs(),
    ) {
        (2, 1) => true,
        (1, 2) => true,
        _ => false,
    }
}

fn is_valid_movement_for_pawn(movement: &Movement, game_state: &GameState, piece: &Piece) -> bool {
    let Movement::Normal { from, to } = movement else {
        return false;
    };
    let dist_x = to[0] as i8 - from[0] as i8;
    let [x, y] = to;
    let right_direction = (piece.color == Color::White && dist_x >= 1)
        || (piece.color == Color::Black && dist_x <= -1);
    if right_direction {
        if to[1] != from[1] {
            let dist_y = (to[1] as i8 - from[1] as i8).abs();
            if dist_y == 1 && dist_x.abs() == 1 {
                // diagonal movement
                return match game_state.board[*x][*y] {
                    Some(piece2) if piece.color != piece2.color => true, // capture
                    Some(_) => false,                                    // blocked
                    None => {
                        let [x0, _] = from;

                        let en_passant_row = match game_state.player_to_move {
                            Color::White => 4,
                            Color::Black => 3,
                        };
                        let opponent_pawn_row = match game_state.player_to_move {
                            Color::White => 6,
                            Color::Black => 1,
                        };
                        if let Some(Movement::Normal {
                            from: last_from,
                            to: last_to,
                        }) = &game_state.last_move
                        {
                            if last_to[0] == en_passant_row
                                && last_from[0] == opponent_pawn_row
                                && last_to[0] == *x0
                                && last_to[1] == *y
                            {
                                if let Some(captured_piece) = game_state.board[*x0][*y] {
                                    if captured_piece.piece_type == PieceType::Pawn {
                                        return true;
                                    }
                                };
                            }
                        };
                        return false;
                    }
                };
            } else {
                return false;
            }
        } else {
            match game_state.board[*x][*y] {
                Some(_) => {
                    return false;
                }
                None => {
                    if dist_x.abs() == 1 {
                        // normal movement
                        return true;
                    } else if (dist_x == -2 && from[0] == 6) || (dist_x == 2 && from[0] == 1) {
                        let x0 = from[0];
                        return match game_state.board[(x + x0) / 2][*y] {
                            Some(_) => false,
                            None => true,
                        };
                    }
                }
            };
        }
    }
    false // wrong direction
}

fn is_valid_movement_for_king(movement: &Movement) -> bool {
    let Movement::Normal { from, to } = movement else {
        return false;
    };
    (to[0] as i8 - from[0] as i8).abs() <= 1 && (to[1] as i8 - from[1] as i8).abs() <= 1
}

pub fn queen_castle_is_valid(game_state: &GameState) -> bool {
    if !((game_state.player_to_move == Color::White && game_state.white_can_castle_queen_side)
        || (game_state.player_to_move == Color::Black && game_state.black_can_castle_queen_side))
    {
        return false;
    }

    if is_in_check(game_state, game_state.player_to_move) {
        return false;
    }

    let king_row = match game_state.player_to_move {
        Color::White => 0,
        Color::Black => 7,
    };
    return game_state.board[king_row][1] == None
        && game_state.board[king_row][2] == None
        && game_state.board[king_row][3] == None
        && game_state.board[king_row][0]
            == Some(Piece {
                piece_type: PieceType::Rook,
                color: game_state.player_to_move,
            })
        && game_state.board[king_row][4]
            == Some(Piece {
                piece_type: PieceType::King,
                color: game_state.player_to_move,
            });
}

pub fn king_castle_is_valid(game_state: &GameState) -> bool {
    if !((game_state.player_to_move == Color::White && game_state.white_can_castle_king_side)
        || (game_state.player_to_move == Color::Black && game_state.black_can_castle_king_side))
    {
        return false;
    }
    if is_in_check(game_state, game_state.player_to_move) {
        return false;
    }
    let king_row = match game_state.player_to_move {
        Color::White => 0,
        Color::Black => 7,
    };
    return game_state.board[king_row][5] == None
        && game_state.board[king_row][6] == None
        && game_state.board[king_row][7]
            == Some(Piece {
                piece_type: PieceType::Rook,
                color: game_state.player_to_move,
            })
        && game_state.board[king_row][4]
            == Some(Piece {
                piece_type: PieceType::King,
                color: game_state.player_to_move,
            });
}

fn is_valid_destination(movement: &Movement, game_state: &GameState, piece: &Piece) -> bool {
    let Movement::Normal { from: _, to: [x, y] } = movement else {
        return false;
    };
    match game_state.board[*x][*y] {
        None => true,
        Some(piece2) if piece2.color != piece.color => true,
        _ => false,
    }
}

pub fn is_valid_movement(movement: &Movement, game_state: &GameState) -> bool {
    is_valid_movement_for_player(movement, game_state, game_state.player_to_move)
}

pub fn is_valid_movement_for_player(
    movement: &Movement,
    game_state: &GameState,
    player_color: Color,
) -> bool {
    match movement {
        Movement::Normal { from: _, to: _ } => {
            is_valid_normal_movement_for_player(movement, game_state, player_color)
        }
        Movement::CastleKingSide(_) => king_castle_is_valid(game_state),
        Movement::CastleQueenSide(_) => queen_castle_is_valid(game_state),
    }
}

pub fn is_valid_normal_movement_for_player(
    movement: &Movement,
    game_state: &GameState,
    player_color: Color,
) -> bool {
    let Movement::Normal { from: _, to } = movement else {
        return false;
    };
    let piece: Piece = movement.get_piece(game_state);
    match game_state.board[to[0]][to[1]] {
        Some(captured_piece) if captured_piece.piece_type == PieceType::King => {
            // capturing the king must be a valid movement even if it puts the player in check
        }
        _ => {
            let mut game_state2 = game_state.clone();
            game_state2.make_movement(movement.clone());
            if is_in_check(&game_state2, player_color) {
                return false;
            }
        }
    };

    if piece.color == player_color && is_valid_destination(&movement, game_state, &piece) {
        match piece.piece_type {
            PieceType::King => is_valid_movement_for_king(&movement),
            PieceType::Queen => is_valid_movement_for_queen(&movement, game_state),
            PieceType::Bishop => is_valid_movement_for_bishop(&movement, game_state),
            PieceType::Knight => is_valid_movement_for_knight(&movement),
            PieceType::Rook => is_valid_movement_for_rook(&movement, game_state),
            PieceType::Pawn => is_valid_movement_for_pawn(&movement, game_state, &piece),
        }
    } else {
        false
    }
}

pub fn is_in_check(game_state: &GameState, player_color: Color) -> bool {
    let player_king = Piece {
        piece_type: PieceType::King,
        color: player_color,
    };

    let king_positions = game_state.get_piece_positions(player_king);
    let Some(king_position) = king_positions.first() else { // else there is no king
        return false;
    };

    // Check if any opponent's piece can attack the position
    return square_is_threatened_by(
        *king_position,
        game_state,
        player_color.get_opponent_color(),
    );
}

// #[allow(dead_code)]
// pub fn move_is_check(movement: Movement, game_state: &GameState) -> bool {
//     let mut next_game_state = game_state.clone();
//     next_game_state.make_movement(movement);
//     return last_move_was_check(&next_game_state);
// }

fn square_is_threatened_by(position: [usize; 2], game_state: &GameState, color: Color) -> bool {
    for position2 in game_state.get_positions_of_color(color) {
        if is_valid_movement_for_player(
            &Movement::Normal {
                from: position2,
                to: position,
            },
            game_state,
            color,
        ) {
            return true;
        }
    }
    return false;
}

// #[allow(dead_code)]
// pub fn last_move_was_check(game_state: &GameState) -> bool {
//     let Some(ref last_move) = game_state.last_move else {
//         return false;
//     };
//     let king_positions = game_state.get_piece_positions(Piece {
//         piece_type: PieceType::King,
//         color: game_state.player_to_move,
//     });
//     for king_position in king_positions {
//         if is_valid_movement(
//             &Movement::Normal {
//                 source: last_move.destination,
//                 destination: king_position,
//             },
//             game_state,
//         ) {
//             return true;
//         }
//     }
//     return false;
// }

pub fn is_valid_cmd(cmd: &Command, game_state: &GameState) -> bool {
    match cmd {
        Command::Resign => true,
        Command::Save => true,
        Command::Move(movement) => is_valid_movement(&movement, game_state),
        _ => false,
    }
}
