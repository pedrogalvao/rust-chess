use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{
    board::{Board, INIT_POSITIONS},
    movement::Movement,
    piece::{Color, Piece, PieceType},
};

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

    pub fn deepclone(&self) -> GameState {
        GameState {
            board: self.deepclone_board(),
            player_to_move: self.player_to_move.clone(),
            last_move: self.last_move.clone(),
            white_can_castle_queen_side: self.white_can_castle_queen_side,
            white_can_castle_king_side: self.white_can_castle_king_side,
            black_can_castle_queen_side: self.black_can_castle_queen_side,
            black_can_castle_king_side: self.black_can_castle_king_side,
        }
    }

    fn deepclone_board(&self) -> Board {
        let mut new_board = [[None; 8]; 8];

        for i in 0..8 {
            for j in 0..8 {
                new_board[i][j] = self.board[i][j].clone();
            }
        }
        new_board
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

    fn set_curr_player_cant_castle(&mut self) {
        if self.player_to_move == Color::White {
            self.white_can_castle_king_side = false;
            self.white_can_castle_queen_side = false;
        } else {
            self.black_can_castle_king_side = false;
            self.black_can_castle_queen_side = false;
        }
    }

    pub fn castle_king_side(&mut self) {
        self.set_curr_player_cant_castle();
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
        self.set_curr_player_cant_castle();
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

    pub fn clone_and_move(&self, movement: Movement) -> Self {
        let mut next_state = self.clone();
        next_state.make_movement(movement);
        next_state
    }
}
