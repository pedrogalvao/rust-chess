use crate::controllers::controller::Command;
use crate::model::{
    game_state::GameState, movement::Movement, piece::Color, piece::Piece, piece::PieceType,
};
use crate::rules::cmd_validator::is_in_check;

use super::castle_validator::{king_castle_is_valid, queen_castle_is_valid};

fn generate_movements_for_pawn(
    game_state: &GameState,
    x: usize,
    y: usize,
    piece: &Piece,
) -> Vec<Movement> {
    let from = [x, y];
    let mut movements = Vec::new();

    let forward_offset = if piece.color == Color::White { 1 } else { -1 };

    // Normal one-step forward move
    let x2 = x as i8 + forward_offset;
    if x2 >= 0 && x2 < 8 && game_state.board[x2 as usize][y] == None {
        movements.push(Movement::Normal {
            from,
            to: [x2 as usize, y],
        });
    }

    // Initial two-step forward move
    let initial_row = if piece.color == Color::White { 1 } else { 6 };
    if x == initial_row {
        let x3 = x as i8 + 2 * forward_offset;
        if x3 >= 0
            && x3 < 8
            && game_state.board[x3 as usize][y] == None
            && game_state.board[x2 as usize][y] == None
        {
            movements.push(Movement::Normal {
                from,
                to: [x3 as usize, y],
            });
        }
    }

    // Diagonal capture moves
    for dx in [-1, 1] {
        let x4 = x as i8 + forward_offset;
        let y4 = y as i8 + dx;

        if x4 >= 0 && x4 < 8 && y4 >= 0 && y4 < 8 {
            if let Some(piece2) = &game_state.board[x4 as usize][y4 as usize] {
                if piece.color != piece2.color {
                    movements.push(Movement::Normal {
                        from,
                        to: [x4 as usize, y4 as usize],
                    });
                }
            }
        }
    }

    // En passant capture
    let en_passant_row = if piece.color == Color::White { 3 } else { 4 };
    if let Some(Movement::Normal {
        from: last_from,
        to: last_to,
    }) = &game_state.last_move
    {
        if x == en_passant_row {
            let left_y = y as i8 - 1;
            let right_y = y as i8 + 1;

            if left_y >= 0 {
                if let Some(piece2) = &game_state.board[x as usize][left_y as usize] {
                    if piece.color != piece2.color {
                        if last_from == &[x, left_y as usize]
                            && last_to == &[x + forward_offset as usize, y]
                        {
                            movements.push(Movement::Normal {
                                from,
                                to: [x + forward_offset as usize, left_y as usize],
                            });
                        }
                    }
                }
            }

            if right_y < 8 {
                if let Some(piece2) = &game_state.board[x][right_y as usize] {
                    if piece.color != piece2.color {
                        if last_from == &[x, right_y as usize]
                            && last_to == &[x + forward_offset as usize, y]
                        {
                            movements.push(Movement::Normal {
                                from,
                                to: [x + forward_offset as usize, right_y as usize],
                            });
                        }
                    }
                }
            }
        }
    };

    movements
}

fn generate_movements_in_one_direction(
    game_state: &GameState,
    x: usize,
    y: usize,
    piece: &Piece,
    direction: [i8; 2],
) -> Vec<Movement> {
    let source = [x, y];
    let [mut x2, mut y2] = [x as i8, y as i8];
    let [dx, dy] = direction;
    let mut movements = vec![];

    x2 += dx;
    y2 += dy;
    while x2 >= 0 && x2 < 8 && y2 >= 0 && y2 < 8 {
        match game_state.board[x2 as usize][y2 as usize] {
            None => {
                movements.push(Movement::Normal {
                    from: source,
                    to: [x2 as usize, y2 as usize],
                });
            }
            Some(piece2) => {
                if piece.color != piece2.color {
                    movements.push(Movement::Normal {
                        from: source,
                        to: [x2 as usize, y2 as usize],
                    });
                }
                break;
            }
        }
        x2 += dx;
        y2 += dy;
    }
    movements
}

fn generate_movements_for_rook(
    game_state: &GameState,
    x: usize,
    y: usize,
    piece: &Piece,
) -> Vec<Movement> {
    let mut movements = vec![];
    let directions: [[i8; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

    for direction in directions {
        movements.extend(generate_movements_in_one_direction(
            game_state, x, y, piece, direction,
        ));
    }
    return movements;
}

fn generate_movements_for_bishop(
    game_state: &GameState,
    x: usize,
    y: usize,
    piece: &Piece,
) -> Vec<Movement> {
    let mut movements = vec![];
    let directions: [[i8; 2]; 4] = [[1, 1], [-1, 1], [1, -1], [-1, -1]];

    for direction in directions {
        movements.extend(generate_movements_in_one_direction(
            game_state, x, y, piece, direction,
        ));
    }
    return movements;
}

fn generate_movements_for_queen(
    game_state: &GameState,
    x: usize,
    y: usize,
    piece: &Piece,
) -> Vec<Movement> {
    let mut movements = generate_movements_for_bishop(game_state, x, y, piece);
    movements.extend(generate_movements_for_rook(game_state, x, y, piece));
    return movements;
}

fn generate_movements_for_knight(
    game_state: &GameState,
    x: usize,
    y: usize,
    piece: &Piece,
) -> Vec<Movement> {
    let source = [x, y];
    let mut movements = Vec::new();

    let offsets = [
        (-2, 1),
        (-1, 2),
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
    ];

    for &(dx, dy) in &offsets {
        let x2 = x as i8 + dx;
        let y2 = y as i8 + dy;

        if x2 >= 0 && x2 < 8 && y2 >= 0 && y2 < 8 {
            if let None = game_state.board[x2 as usize][y2 as usize] {
                movements.push(Movement::Normal {
                    from: source,
                    to: [x2 as usize, y2 as usize],
                });
            } else {
                // Check if the piece at the destination is of a different color
                if let Some(piece2) = &game_state.board[x2 as usize][y2 as usize] {
                    if piece.color != piece2.color {
                        movements.push(Movement::Normal {
                            from: source,
                            to: [x2 as usize, y2 as usize],
                        });
                    }
                }
            }
        }
    }

    movements
}

fn generate_movements_for_king(
    game_state: &GameState,
    x: usize,
    y: usize,
    piece: &Piece,
) -> Vec<Movement> {
    let mut movements = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            let x2 = x as i32 + dx;
            let y2: i32 = y as i32 + dy;
            if 0 <= x2 && x2 < 8 && 0 <= y2 && y2 < 8 {
                match game_state.board[x2 as usize][y2 as usize] {
                    Some(piece2) if piece.color == piece2.color => {
                        // position is blocked
                        continue;
                    }
                    _ => {
                        movements.push(Movement::Normal {
                            from: [x, y],
                            to: [x2 as usize, y2 as usize],
                        });
                    }
                }
            }
        }
    }
    movements
}

pub fn generate_movements_for_piece(
    game_state: &GameState,
    x: usize,
    y: usize,
    piece: Piece,
) -> Vec<Movement> {
    match piece.piece_type {
        PieceType::King => generate_movements_for_king(game_state, x, y, &piece),
        PieceType::Queen => generate_movements_for_queen(game_state, x, y, &piece),
        PieceType::Rook => generate_movements_for_rook(game_state, x, y, &piece),
        PieceType::Bishop => generate_movements_for_bishop(game_state, x, y, &piece),
        PieceType::Knight => generate_movements_for_knight(game_state, x, y, &piece),
        PieceType::Pawn => generate_movements_for_pawn(game_state, x, y, &piece),
    }
}

pub fn generate_movements_for_player(game_state: &GameState, color: Color) -> Vec<Movement> {
    let mut movements = Vec::new();
    for x in 0..8 {
        for y in 0..8 {
            match game_state.board[x][y] {
                Some(piece) if piece.color == color => {
                    for movement in generate_movements_for_piece(game_state, x, y, piece) {
                        let game_state2 = game_state.clone_and_move(movement.clone());
                        if !is_in_check(&game_state2, color) {
                            // Player can't put himself in check
                            movements.push(movement);
                        }
                    }
                }
                _ => continue,
            }
        }
    }
    if king_castle_is_valid(game_state) {
        movements.push(Movement::CastleKingSide(color));
    }
    if queen_castle_is_valid(game_state) {
        movements.push(Movement::CastleQueenSide(color));
    }
    movements
}

pub fn generate_movements(game_state: &GameState) -> Vec<Movement> {
    generate_movements_for_player(game_state, game_state.player_to_move)
}

pub fn generate_commands(game_state: &GameState) -> Vec<Command> {
    let mut commands = vec![];
    for movement in generate_movements_for_player(game_state, game_state.player_to_move) {
        commands.push(Command::Move(movement));
    }
    commands
}

pub fn generate_movements_for_player_ignoring_check(
    game_state: &GameState,
    color: Color,
) -> Vec<Movement> {
    let mut movements = Vec::new();
    for x in 0..8 {
        for y in 0..8 {
            match game_state.board[x][y] {
                Some(piece) if piece.color == color => {
                    movements.extend(generate_movements_for_piece(game_state, x, y, piece));
                }
                _ => continue,
            }
        }
    }
    if king_castle_is_valid(game_state) {
        movements.push(Movement::CastleKingSide(color));
    }
    if queen_castle_is_valid(game_state) {
        movements.push(Movement::CastleQueenSide(color));
    }
    movements
}
