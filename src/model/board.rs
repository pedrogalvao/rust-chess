use super::piece::{Color, Piece, PieceType};
use rand::seq::SliceRandom;
use rand::Rng;

pub type Board = [[Option<Piece>; 8]; 8];

pub const INIT_POSITIONS: Board = [
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

pub fn create_960_board() -> Board {
    let rook_col1 = rand::thread_rng().gen_range(0, 5);
    let rook_col2 = rand::thread_rng().gen_range(rook_col1 + 2, 8);
    let king_col = rand::thread_rng().gen_range(rook_col1 + 1, rook_col2);
    let mut unused_cols: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7];
    unused_cols.remove(unused_cols.iter().position(|x| *x == rook_col1).unwrap());
    unused_cols.remove(unused_cols.iter().position(|x| *x == rook_col2).unwrap());
    unused_cols.remove(unused_cols.iter().position(|x| *x == king_col).unwrap());
    let bishop_col1 = *unused_cols.choose(&mut rand::thread_rng()).unwrap();
    unused_cols.remove(unused_cols.iter().position(|x| *x == bishop_col1).unwrap());
    let mut bishop_col2 = *unused_cols.choose(&mut rand::thread_rng()).unwrap();
    while bishop_col1 % 2 == bishop_col2 % 2 {
        bishop_col2 = *unused_cols.choose(&mut rand::thread_rng()).unwrap();
    }
    unused_cols.remove(unused_cols.iter().position(|x| *x == bishop_col2).unwrap());
    let queen_col = *unused_cols.choose(&mut rand::thread_rng()).unwrap();
    unused_cols.remove(unused_cols.iter().position(|x| *x == queen_col).unwrap());
    let knight_col1 = *unused_cols.choose(&mut rand::thread_rng()).unwrap();
    unused_cols.remove(unused_cols.iter().position(|x| *x == knight_col1).unwrap());
    let knight_col2 = *unused_cols.choose(&mut rand::thread_rng()).unwrap();
    unused_cols.remove(unused_cols.iter().position(|x| *x == knight_col2).unwrap());
    let mut board = INIT_POSITIONS;
    for (row, player_color) in [(0, Color::White), (7, Color::Black)] {
        board[row][king_col] = Some(Piece {
            piece_type: PieceType::King,
            color: player_color,
        });
        board[row][rook_col1] = Some(Piece {
            piece_type: PieceType::Rook,
            color: player_color,
        });
        board[row][rook_col2] = Some(Piece {
            piece_type: PieceType::Rook,
            color: player_color,
        });
        board[row][bishop_col1] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: player_color,
        });
        board[row][bishop_col2] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: player_color,
        });
        board[row][queen_col] = Some(Piece {
            piece_type: PieceType::Queen,
            color: player_color,
        });
        board[row][knight_col1] = Some(Piece {
            piece_type: PieceType::Knight,
            color: player_color,
        });
        board[row][knight_col2] = Some(Piece {
            piece_type: PieceType::Knight,
            color: player_color,
        });
    }
    return board;
}
