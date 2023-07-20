use crate::movement::Movement;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

impl Serialize for Piece {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let piece_char = self.to_ascii();
        serializer.serialize_str(piece_char.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Piece {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let piece_str: &str = Deserialize::deserialize(deserializer)?;

        let (piece_type, color) = match piece_str {
            "K" => (PieceType::King, Color::White),
            "k" => (PieceType::King, Color::Black),
            "Q" => (PieceType::Queen, Color::White),
            "q" => (PieceType::Queen, Color::Black),
            "R" => (PieceType::Rook, Color::White),
            "r" => (PieceType::Rook, Color::Black),
            "B" => (PieceType::Bishop, Color::White),
            "b" => (PieceType::Bishop, Color::Black),
            "N" => (PieceType::Knight, Color::White),
            "n" => (PieceType::Knight, Color::Black),
            "P" => (PieceType::Pawn, Color::White),
            "p" => (PieceType::Pawn, Color::Black),
            _ => {
                return Err(serde::de::Error::custom("Invalid piece type or color"));
            }
        };

        Ok(Piece { piece_type, color })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

pub type Board = [[Option<Piece>; 8]; 8];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,
    pub player_to_move: Color,
    pub last_move: Option<Movement>,
    pub white_can_castle_queen_side: bool,
    pub white_can_castle_king_side: bool,
    pub black_can_castle_queen_side: bool,
    pub black_can_castle_king_side: bool,
}

pub fn write_game_state_to_json(
    game_state: &GameState,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, game_state)?;
    println!("{}", serde_json::to_string_pretty(game_state).unwrap());
    Ok(())
}

pub fn load_game_state_from_json(file_path: &str) -> Result<GameState, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let game_state: GameState = serde_json::from_str(&contents)?;
    Ok(game_state)
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
            white_can_castle_queen_side: true,
            white_can_castle_king_side: true,
            black_can_castle_king_side: true,
            black_can_castle_queen_side: true,
        }
    }

    #[allow(dead_code)]
    pub const fn new_empty() -> Self {
        Self {
            board: [[None; 8]; 8],
            player_to_move: Color::White,
            last_move: None,
            white_can_castle_queen_side: true,
            white_can_castle_king_side: true,
            black_can_castle_king_side: true,
            black_can_castle_queen_side: true,
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

    fn update_can_castle(&mut self, movement: &Movement) {
        let Movement::Normal { from: source, to: destination } = movement else {
            return;
        };
        if self.player_to_move == Color::White {
            if self.white_can_castle_king_side {
                if source == &[0, 4] || source == &[0, 7] {
                    self.white_can_castle_king_side = false
                }
            }
            if self.white_can_castle_queen_side {
                if source == &[0, 4] || source == &[0, 0] {
                    self.white_can_castle_queen_side = false
                }
            }
            if destination == &[7, 0] {
                self.black_can_castle_queen_side = false
            }
            if destination == &[7, 7] {
                self.black_can_castle_king_side = false
            }
        } else {
            if self.black_can_castle_king_side {
                if source == &[7, 4] || source == &[7, 7] {
                    self.black_can_castle_king_side = false
                }
            }
            if self.black_can_castle_queen_side {
                if source == &[7, 4] || source == &[7, 0] {
                    self.black_can_castle_queen_side = false
                }
            }
            if destination == &[0, 0] {
                self.white_can_castle_queen_side = false
            }
            if destination == &[0, 7] {
                self.white_can_castle_king_side = false
            }
        }
    }

    pub fn make_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Normal {
                from: source,
                to: destination,
            } => {
                let [x, y] = source;
                let [x2, y2] = destination;
                if (y != y2 && (x2 == 4 || x2 == 5))
                    && self.board[x][y].unwrap().piece_type == PieceType::Pawn
                    && self.board[x2][y2] == None
                {
                    // en passant
                    self.board[x][y2] = None;
                }
                self.board[x2][y2] = mem::take(&mut self.board[x][y]);
                if (x2 == 0 || x2 == 7) && self.board[x2][y2].unwrap().piece_type == PieceType::Pawn
                {
                    self.board[x2][y2] = Some(Piece {
                        piece_type: PieceType::Queen,
                        color: self.player_to_move,
                    }); // promote the pawn
                }
                self.update_can_castle(&movement);
                self.player_to_move = self.player_to_move.get_opponent_color();
                self.last_move = Some(movement);
            }
            Movement::CastleKingSide(_) => self.castle_king_side(),
            Movement::CastleQueenSide(_) => self.castle_queen_side(),
        }
    }

    pub fn castle_king_side(&mut self) {
        if self.player_to_move == Color::White {
            self.white_can_castle_king_side = false;
            self.white_can_castle_queen_side = false;
        } else {
            self.black_can_castle_king_side = false;
            self.black_can_castle_queen_side = false;
        }
        let king_row = match self.player_to_move {
            Color::White => 0,
            Color::Black => 7,
        };
        self.board[king_row][6] = Some(Piece {
            piece_type: PieceType::King,
            color: self.player_to_move,
        });
        self.board[king_row][5] = Some(Piece {
            piece_type: PieceType::Rook,
            color: self.player_to_move,
        });
        self.board[king_row][7] = None;
        self.board[king_row][4] = None;

        self.last_move = Some(Movement::CastleKingSide(self.player_to_move));
        self.player_to_move = self.player_to_move.get_opponent_color();
    }

    pub fn castle_queen_side(&mut self) {
        if self.player_to_move == Color::White {
            self.white_can_castle_king_side = false;
            self.white_can_castle_queen_side = false;
        } else {
            self.black_can_castle_king_side = false;
            self.black_can_castle_queen_side = false;
        }
        let king_row = match self.player_to_move {
            Color::White => 0,
            Color::Black => 7,
        };
        self.board[king_row][2] = Some(Piece {
            piece_type: PieceType::King,
            color: self.player_to_move,
        });
        self.board[king_row][3] = Some(Piece {
            piece_type: PieceType::Rook,
            color: self.player_to_move,
        });
        self.board[king_row][0] = None;
        self.board[king_row][4] = None;

        self.last_move = Some(Movement::CastleQueenSide(self.player_to_move));
        self.player_to_move = self.player_to_move.get_opponent_color();
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
