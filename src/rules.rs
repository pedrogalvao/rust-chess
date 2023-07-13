use crate::board::{Color, GameState, Piece, PieceType};
use crate::movement::Movement;
// use crate::move_generator::generate_movements_for_player;

fn is_valid_movement_for_rook(movement: &Movement, game_state: &GameState) -> bool {
    if movement.source[0] == movement.destination[0] {
        let x = movement.source[0];
        if movement.source[1] < movement.destination[1] {
            for y in (movement.source[1] + 1)..movement.destination[1] {
                if None != game_state.board[x][y] {
                    return false;
                }
            }
        } else {
            for y in (movement.destination[1] + 1)..movement.source[1] {
                if None != game_state.board[x][y] {
                    return false;
                }
            }
        }
        return true;
    } else if movement.source[1] == movement.destination[1] {
        let y = movement.source[1];
        if movement.source[0] < movement.destination[0] {
            for x in (movement.source[0] + 1)..movement.destination[0] {
                if None != game_state.board[x][y] {
                    return false;
                }
            }
        } else {
            for x in (movement.destination[0] + 1)..movement.source[0] {
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
    let dist_x: i8 = movement.source[0] as i8 - movement.destination[0] as i8;
    let dist_y: i8 = movement.source[1] as i8 - movement.destination[1] as i8;
    if dist_x.abs() != dist_y.abs() {
        false
    } else {
        let mut x = movement.source[0];
        let mut y = movement.source[1];
        loop {
            if movement.source[0] < movement.destination[0] {
                x += 1;
            } else {
                x -= 1;
            }
            if movement.source[1] < movement.destination[1] {
                y += 1;
            } else {
                y -= 1;
            }
            if x == movement.destination[0] {
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
    match (
        (movement.source[0] as i8 - movement.destination[0] as i8).abs(),
        (movement.source[1] as i8 - movement.destination[1] as i8).abs(),
    ) {
        (2, 1) => true,
        (1, 2) => true,
        _ => false,
    }
}

fn is_valid_movement_for_pawn(movement: &Movement, game_state: &GameState, piece: &Piece) -> bool {
    let dist_x = movement.destination[0] as i8 - movement.source[0] as i8;
    let [x, y] = movement.destination;
    let right_direction = (piece.color == Color::White && dist_x >= 1)
        || (piece.color == Color::Black && dist_x <= -1);
    if right_direction {
        if movement.destination[1] != movement.source[1] {
            let dist_y = (movement.destination[1] as i8 - movement.source[1] as i8).abs();
            if dist_y == 1 && dist_x.abs() == 1 {
                // diagonal movement
                return match game_state.board[x][y] {
                    Some(piece2) if piece.color != piece2.color => true, // capture
                    _ => false,
                };
            } else {
                return false;
            }
        } else {
            match game_state.board[x][y] {
                Some(_) => {
                    return false;
                }
                None => {
                    if dist_x.abs() == 1 {
                        // normal movement
                        return true;
                    } else if (dist_x == -2 && movement.source[0] == 6)
                        || (dist_x == 2 && movement.source[0] == 1)
                    {
                        let x0 = movement.source[0];
                        return match game_state.board[(x + x0) / 2][y] {
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
    (movement.destination[0] as i8 - movement.source[0] as i8).abs() <= 1
        && (movement.destination[1] as i8 - movement.source[1] as i8).abs() <= 1
}

fn is_valid_destination(movement: &Movement, game_state: &GameState, piece: &Piece) -> bool {
    let [x, y] = movement.destination;
    match game_state.board[x][y] {
        None => true,
        Some(piece2) if piece2.color != piece.color => true,
        _ => false,
    }
}

pub fn is_valid_movement(movement: &Movement, game_state: &GameState) -> bool {
    let piece = movement.get_piece(game_state);
    if piece.color == game_state.player_to_move
        && is_valid_destination(movement, game_state, &piece)
    {
        match piece.piece_type {
            PieceType::King => is_valid_movement_for_king(movement),
            PieceType::Queen => is_valid_movement_for_queen(movement, game_state),
            PieceType::Bishop => is_valid_movement_for_bishop(movement, game_state),
            PieceType::Knight => is_valid_movement_for_knight(movement),
            PieceType::Rook => is_valid_movement_for_rook(movement, game_state),
            PieceType::Pawn => is_valid_movement_for_pawn(movement, game_state, &piece),
        }
    } else {
        false
    }
}

pub fn is_valid_movement_for_player(
    movement: &Movement,
    game_state: &GameState,
    player_color: Color,
) -> bool {
    let piece = movement.get_piece(game_state);
    /*
    board2 = b.copy_make_move(movement);
    if is_in_check(board2, player_color) {
        return false;
    }
    */
    if piece.color == player_color && is_valid_destination(movement, game_state, &piece) {
        match piece.piece_type {
            PieceType::King => is_valid_movement_for_king(movement),
            PieceType::Queen => is_valid_movement_for_queen(movement, game_state),
            PieceType::Bishop => is_valid_movement_for_bishop(movement, game_state),
            PieceType::Knight => is_valid_movement_for_knight(movement),
            PieceType::Rook => is_valid_movement_for_rook(movement, game_state),
            PieceType::Pawn => is_valid_movement_for_pawn(movement, game_state, &piece),
        }
    } else {
        false
    }
}

pub fn is_in_check(game_state: &GameState, player_color: Color) -> bool {
    let opponent_color = player_color.get_opponent_color();
    let player_king = Piece {
        piece_type: PieceType::King,
        color: player_color,
    };
    let king_position = *game_state.get_piece_positions(player_king).first().unwrap();

    // let Some(ref last_move) = board.last_move else {
    //     return false;
    // };
    // let m = Movement{source: last_move.destination, destination: king_position};
    // if is_valid_movement_for_player(&movement, board, opponent_color) {
    //     return true;
    // }

    // Check if any opponent's piece can attack the position
    let opponent_positions: Vec<[usize; 2]> =
        game_state.get_positions_of_color(player_color.get_opponent_color());
    for opponent_position in opponent_positions {
        let movement = Movement {
            source: opponent_position,
            destination: king_position,
        };

        if is_valid_movement_for_player(&movement, game_state, opponent_color) {
            return true;
        }
    }
    false
}
