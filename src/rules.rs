use crate::board::{Board, Color, Piece, PieceType};
use crate::movement::Movement;
// use crate::move_generator::generate_movements_for_player;

fn is_valid_movement_for_rook(m: &Movement, b: &Board) -> bool {
    if m.source[0] == m.destination[0] {
        let x = m.source[0];
        if m.source[1] < m.destination[1] {
            for y in (m.source[1] + 1)..m.destination[1] {
                if None != b.positions[x][y] {
                    return false;
                }
            }
        } else {
            for y in (m.destination[1] + 1)..m.source[1] {
                if None != b.positions[x][y] {
                    return false;
                }
            }
        }
        return true;
    } else if m.source[1] == m.destination[1] {
        let y = m.source[1];
        if m.source[0] < m.destination[0] {
            for x in (m.source[0] + 1)..m.destination[0] {
                if None != b.positions[x][y] {
                    return false;
                }
            }
        } else {
            for x in (m.destination[0] + 1)..m.source[0] {
                if None != b.positions[x][y] {
                    return false;
                }
            }
        }
        return true;
    } else {
        false
    }
}

fn is_valid_movement_for_bishop(m: &Movement, b: &Board) -> bool {
    let dist_x: i8 = m.source[0] as i8 - m.destination[0] as i8;
    let dist_y: i8 = m.source[1] as i8 - m.destination[1] as i8;
    if dist_x.abs() != dist_y.abs() {
        false
    } else {
        let mut x = m.source[0];
        let mut y = m.source[1];
        loop {
            if m.source[0] < m.destination[0] {
                x += 1;
            } else {
                x -= 1;
            }
            if m.source[1] < m.destination[1] {
                y += 1;
            } else {
                y -= 1;
            }
            if x == m.destination[0] {
                break;
            }
            if None != b.positions[x][y] {
                return false;
            }
        }
        return true;
    }
}

fn is_valid_movement_for_queen(m: &Movement, b: &Board) -> bool {
    is_valid_movement_for_rook(&m, b) || is_valid_movement_for_bishop(&m, b)
}

fn is_valid_movement_for_knight(m: &Movement) -> bool {
    match (
        (m.source[0] as i8 - m.destination[0] as i8).abs(),
        (m.source[1] as i8 - m.destination[1] as i8).abs(),
    ) {
        (2, 1) => true,
        (1, 2) => true,
        _ => false,
    }
}

fn is_valid_movement_for_pawn(m: &Movement, b: &Board, piece: &Piece) -> bool {
    let dist_x = m.destination[0] as i8 - m.source[0] as i8;
    let [x, y] = m.destination;
    let right_direction = (piece.color == Color::White && dist_x >= 1)
        || (piece.color == Color::Black && dist_x <= -1);
    if right_direction {
        if m.destination[1] != m.source[1] {
            let dist_y = (m.destination[1] as i8 - m.source[1] as i8).abs();
            if dist_y == 1 && dist_x.abs() == 1 {
                // diagonal movement
                return match b.positions[x][y] {
                    Some(piece2) if piece.color != piece2.color => true, // capture
                    _ => false,
                };
            } else {
                return false;
            }
        } else {
            match b.positions[x][y] {
                Some(piece2) => {
                    return false;
                },
                None => {
                    if dist_x.abs() == 1 { // normal movement
                        return true;
                    } else if (dist_x == -2 && m.source[0] == 6) || (dist_x == 2 && m.source[0] == 1) {
                        let x0 = m.source[0];
                        match b.positions[(x+x0)/2][y] {
                            Some(piece2) => {
                                return false;
                            },
                            None => {
                                return true;
                            }
                        }
                    }
                }
            };
        }
    }
    false // wrong direction
}

fn is_valid_movement_for_king(m: &Movement) -> bool {
    (m.destination[0] as i8 - m.source[0] as i8).abs() <= 1
        && (m.destination[1] as i8 - m.source[1] as i8).abs() <= 1
}

fn is_valid_destination(m: &Movement, b: &Board, piece: &Piece) -> bool {
    let [x, y] = m.destination;
    match b.positions[x][y] {
        None => true,
        Some(piece2) if piece2.color != piece.color => true,
        _ => false,
    }
}

pub fn is_valid_movement(m: &Movement, b: &Board) -> bool {
    let piece = m.get_piece(b);
    if piece.color == b.player_to_move && is_valid_destination(m, b, &piece) {
        match piece.piece_type {
            PieceType::King => is_valid_movement_for_king(m),
            PieceType::Queen => is_valid_movement_for_queen(m, b),
            PieceType::Bishop => is_valid_movement_for_bishop(m, b),
            PieceType::Knight => is_valid_movement_for_knight(m),
            PieceType::Rook => is_valid_movement_for_rook(m, b),
            PieceType::Pawn => is_valid_movement_for_pawn(m, b, &piece),
        }
    } else {
        false
    }
}

pub fn is_valid_movement_for_player(m: &Movement, b: &Board, player_color:Color) -> bool {
    let piece = m.get_piece(b);
    /*
    board2 = b.copy_make_move(m);
    if is_in_check(board2, player_color) {
        return false;
    }
    */
    if piece.color == player_color && is_valid_destination(m, b, &piece) {
        match piece.piece_type {
            PieceType::King => is_valid_movement_for_king(m),
            PieceType::Queen => is_valid_movement_for_queen(m, b),
            PieceType::Bishop => is_valid_movement_for_bishop(m, b),
            PieceType::Knight => is_valid_movement_for_knight(m),
            PieceType::Rook => is_valid_movement_for_rook(m, b),
            PieceType::Pawn => is_valid_movement_for_pawn(m, b, &piece),
        }
    } else {
        false
    }
}

pub fn is_in_check(board: &Board, player_color: Color) -> bool {
    let opponent_color = player_color.get_opponent_color();
    let player_king = Piece{piece_type: PieceType::King, color: player_color};
    let king_position = *board.get_piece_positions(player_king).first().unwrap();

    // let Some(ref last_move) = board.last_move else {
    //     return false;
    // };
    // let m = Movement{source: last_move.destination, destination: king_position};
    // if is_valid_movement_for_player(&m, board, opponent_color) {
    //     return true;
    // }

    // Check if any opponent's piece can attack the position
    let opponent_positions = board.get_positions_of_color(player_color.get_opponent_color());
    for opponent_position in opponent_positions {
        let m = Movement{source:opponent_position, destination:king_position};
        
        if is_valid_movement_for_player(&m, board, opponent_color) {
            return true;
        }
    }
    false
}