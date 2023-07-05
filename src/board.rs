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
                PieceType::King => 'K',
                PieceType::Queen => 'Q',
                PieceType::Rook => 'R',
                PieceType::Bishop => 'B',
                PieceType::Knight => 'N',
                PieceType::Pawn => 'P'
            },
            Color::Black => match self.piece_type {
                PieceType::King => 'k',
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                PieceType::Pawn => 'p'
            }
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
            _ => {return Err(());}
        };
        Ok(Piece {piece_type: piece_type, color: color})
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

pub struct Board {
    pub positions: [[Option<Piece>; 8]; 8],
    pub player_to_move: Color,
}

const INIT_POSITIONS: [[Option<Piece>; 8]; 8] = [
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
    [Some(Piece {
        piece_type: PieceType::Pawn,
        color: Color::Black,
    }); 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [Some(Piece {
        piece_type: PieceType::Pawn,
        color: Color::White,
    }); 8],
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
];

impl Board {
    pub fn new() -> Self {
        Self {
            positions: INIT_POSITIONS,
            player_to_move: Color::White,
        }
    }

    pub fn get_piece_positions(&self, piece: Piece) -> Vec<[usize; 2]> {
        let mut results = vec![];
        for i in 0..8 {
            for j in 0..8 {
                match self.positions[i][j] {
                    Some(piece2) => {
                        if piece == piece2 {
                            results.push([i, j])
                        } else {
                            continue;
                        }
                    }
                    None => continue,
                }
            }
        }
        results
    }

    pub fn make_movement(&mut self, m: &Movement) {
        let [x, y] = m.source;
        let [x2, y2] = m.destination;
        self.positions[x2][y2] = mem::take(&mut self.positions[x][y]);
        self.player_to_move = match self.player_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = match self.player_to_move {
            Color::White => write!(f, "White to move:\n  "),
            Color::Black => write!(f, "Black to move:\n  "),
        };
        for c in 'a'..='h' {
            write!(f, "  {} ", c);
        }
        write!(f, "\n  ");
        for _ in 0..8 {
            write!(f, "|---");
        }
        write!(f, "|\n");
        for i in 0..8 {
            write!(f, "{} ", 1 + i);
            for j in 0..8 {
                let piece_opt: Option<Piece> = self.positions[i][j];

                let piece_char: char = match piece_opt {
                    Some(piece) => piece.to_char(),
                    None => ' ',
                };
                write!(f, "| {} ", piece_char);
            }
            write!(f, "|\n  ");
            for _ in 0..8 {
                write!(f, "|---");
            }
            write!(f, "|\n");
        }
        Ok(())
    }
}
