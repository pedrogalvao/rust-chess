use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{
    board::{create_960_board, Board, INIT_POSITIONS},
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
    pub king_positions: [Option<[usize; 2]>; 2],
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
            king_positions: [Some([0, 4]), Some([7, 4])],
        }
    }

    pub fn new960() -> Self {
        let initial_positions = create_960_board();
        let mut king_positions = [None, None];
        for i in 1..6 {
            match initial_positions[0][i] {
                Some(piece) if piece.piece_type == PieceType::King => {
                    king_positions = [Some([0, i]), Some([7, i])];
                }
                _ => {}
            }
        }
        Self {
            board: initial_positions,
            player_to_move: Color::White,
            last_move: None,
            white_can_castle_queen_side: true,
            white_can_castle_king_side: true,
            black_can_castle_king_side: true,
            black_can_castle_queen_side: true,
            king_positions: king_positions,
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
            king_positions: [None, None],
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
            king_positions: self.king_positions.clone(),
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

    pub fn get_king_position(&self, player: Color) -> Option<[usize; 2]> {
        return self.king_positions[player as usize];
    }

    fn update_can_castle(&mut self, movement: &Movement) {
        let Movement::Normal { from: source, to: destination } = movement else {
            return;
        };
        let Some(white_king_position) = self.get_king_position(Color::White) else {
            self.white_can_castle_king_side = false;
            self.white_can_castle_queen_side = false;
            return;
        };
        let Some(black_king_position) = self.get_king_position(Color::Black) else {
            self.black_can_castle_king_side = false;
            self.black_can_castle_queen_side = false;
            return;
        };
        if self.player_to_move == Color::White {
            if self.white_can_castle_king_side {
                if source == &white_king_position || source == &[0, 7] {
                    self.white_can_castle_king_side = false
                }
            }
            if self.white_can_castle_queen_side {
                if source == &white_king_position || source == &[0, 0] {
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
                if source == &black_king_position || source == &[7, 7] {
                    self.black_can_castle_king_side = false
                }
            }
            if self.black_can_castle_queen_side {
                if source == &black_king_position || source == &[7, 0] {
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
        let Some([king_row, king_col]) = self.get_king_position(self.player_to_move) else {
            return;
        };
        for col in king_col..8 {
            match self.board[king_row][col] {
                Some(piece) if piece.piece_type == PieceType::Rook => {
                    self.board[king_row][col] = None;
                }
                _ => {}
            }
        }
        self.board[king_row][6] = Some(Piece {
            piece_type: PieceType::King,
            color: self.player_to_move,
        });
        self.board[king_row][king_col] = None;
        self.board[king_row][5] = Some(Piece {
            piece_type: PieceType::Rook,
            color: self.player_to_move,
        });

        self.last_move = Some(Movement::CastleKingSide(self.player_to_move));
        self.player_to_move = self.player_to_move.get_opponent_color();
    }

    pub fn castle_queen_side(&mut self) {
        self.set_curr_player_cant_castle();
        let Some([king_row, king_col]) = self.get_king_position(self.player_to_move) else {
            println!("Tried to castle without having a king");
            panic!();
        };
        for col in 0..king_col {
            match self.board[king_row][col] {
                Some(piece) if piece.piece_type == PieceType::Rook => {
                    self.board[king_row][col] = None;
                }
                _ => {}
            }
        }
        self.board[king_row][king_col] = None;
        self.board[king_row][2] = Some(Piece {
            piece_type: PieceType::King,
            color: self.player_to_move,
        });
        self.board[king_row][3] = Some(Piece {
            piece_type: PieceType::Rook,
            color: self.player_to_move,
        });

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
