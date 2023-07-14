use crate::movement::Movement;
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
    pub fn to_ascii(&self) -> char {
        match self.color {
            Color::White => match self.piece_type {
                PieceType::King => 'K',
                PieceType::Queen => 'Q',
                PieceType::Rook => 'R',
                PieceType::Bishop => 'B',
                PieceType::Knight => 'N',
                PieceType::Pawn => 'P',
            },
            Color::Black => match self.piece_type {
                PieceType::King => 'k',
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                PieceType::Pawn => 'p',
            },
        }
    }
    pub fn to_unicode(&self) -> char {
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
    pub can_castle_queen_side: bool,
    pub can_castle_king_side: bool,
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
    pub const fn new() -> Self {
        Self {
            board: INIT_POSITIONS,
            player_to_move: Color::White,
            last_move: None,
            can_castle_queen_side: true,
            can_castle_king_side: true,
        }
    }

    #[allow(dead_code)]
    pub const fn new_empty() -> Self {
        Self {
            board: [[None; 8]; 8],
            player_to_move: Color::White,
            last_move: None,
            can_castle_queen_side: true,
            can_castle_king_side: true,
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
