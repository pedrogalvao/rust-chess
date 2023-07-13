use crate::movement::Movement;
use std::fmt;
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Piece {
    pub fn to_char(&self) -> char {
        match self.color {
            Color::White => match self.piece_type {
                PieceType::King => '♔',
                PieceType::Queen => '♕',
                PieceType::Rook => '♖',
                PieceType::Bishop => '♗',
                PieceType::Knight => '♘',
                PieceType::Pawn => '♙',
            },
            Color::Black => match self.piece_type {
                PieceType::King => '♚',
                PieceType::Queen => '♛',
                PieceType::Rook => '♜',
                PieceType::Bishop => '♝',
                PieceType::Knight => '♞',
                PieceType::Pawn => 'p',
            },
        }
    }
    pub fn from_char(piece_char: char, color: Color) -> Result<Self, ()> {
        let piece_type = match piece_char {
            'K' | 'k' => PieceType::King,
            'Q' | 'q' => PieceType::Queen,
            'B' | 'b' => PieceType::Bishop,
            'N' | 'n' => PieceType::Knight,
            'R' | 'r' => PieceType::Rook,
            'P' | 'p' => PieceType::Pawn,
            _ => {
                return Err(());
            }
        };
        Ok(Piece {
            piece_type: piece_type,
            color: color,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn get_opponent_color(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

pub type Board = [[Option<Piece>; 8]; 8];

#[derive(Debug, Clone)]
pub struct GameState {
    pub board: Board,
    pub player_to_move: Color,
    pub last_move: Option<Movement>,
}

const INIT_POSITIONS: Board = [
    [
        Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        }),
        Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        }),
        Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        }),
        Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
        }),
        Some(Piece {
            piece_type: PieceType::King,
            color: Color::White,
        }),
        Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        }),
        Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        }),
        Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        }),
    ],
    [Some(Piece {
        piece_type: PieceType::Pawn,
        color: Color::White,
    }); 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [Some(Piece {
        piece_type: PieceType::Pawn,
        color: Color::Black,
    }); 8],
    [
        Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        }),
        Some(Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        }),
        Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        }),
    ],
];

impl GameState {
    pub fn new() -> Self {
        Self {
            board: INIT_POSITIONS,
            player_to_move: Color::White,
            last_move: None,
        }
    }

    pub fn get_piece_positions(&self, piece: Piece) -> Vec<[usize; 2]> {
        let mut results = vec![];
        for i in 0..8 {
            for j in 0..8 {
                match self.board[i][j] {
                    Some(piece2) if piece == piece2 => results.push([i, j]),
                    _ => continue,
                }
            }
        }
        results
    }

    pub fn make_movement(&mut self, movement: Movement) {
        let [x, y] = movement.source;
        let [x2, y2] = movement.destination;
        self.board[x2][y2] = mem::take(&mut self.board[x][y]);
        if (x2 == 0 || x2 == 7) && self.board[x2][y2].unwrap().piece_type == PieceType::Pawn {
            self.board[x2][y2] = Some(Piece {
                piece_type: PieceType::Queen,
                color: self.player_to_move,
            }); // promote the pawn
        }
        self.player_to_move = self.player_to_move.get_opponent_color();
        self.last_move = Some(movement);
    }

    pub fn get_positions_of_color(&self, color: Color) -> Vec<[usize; 2]> {
        let mut results = vec![];
        for i in 0..8 {
            for j in 0..8 {
                match self.board[i][j] {
                    Some(piece) if piece.color == color => results.push([i, j]),
                    _ => continue,
                }
            }
        }
        results
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = match self.player_to_move {
            Color::White => print!("White to move:\n  "),
            Color::Black => print!("Black to move:\n  "),
        };
        let range: Vec<usize>;
        if self.player_to_move == Color::White {
            range = (0..8).rev().collect();
        } else {
            range = (0..8).collect();
        }
        let range2: Vec<usize>;
        if self.player_to_move == Color::White {
            range2 = (0..8).collect();
        } else {
            range2 = (0..8).rev().collect();
        }
        if self.player_to_move == Color::White {
            for c in 'a'..='h' {
                print!("  {} ", c);
            }
        } else {
            for c in ('a'..='h').rev() {
                print!("  {} ", c);
            }
        }
        print!("\n  ");
        for _ in 0..8 {
            print!("|---");
        }
        print!("|\n");
        for i in range {
            print!("{} ", 1 + i);
            for j in &range2 {
                let piece_opt: Option<Piece> = self.board[i][*j];

                let piece_char: char = match piece_opt {
                    Some(piece) => piece.to_char(),
                    None => ' ',
                };
                print!("| {} ", piece_char);
            }
            print!("|\n  ");
            for _ in 0..8 {
                print!("|---");
            }
            print!("|\n");
        }
        Ok(())
    }
}
