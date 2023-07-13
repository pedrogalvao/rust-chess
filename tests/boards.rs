use rust_chess::board::{Board, Color, Piece, PieceType};

const test_positions1: [[Option<Piece>; 8]; 8] = [        
        [
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }), None, None, None, None, None, None, 
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }), 
        ],
        [
            Some(Piece {
                piece_type: PieceType::Queen,
                color: Color::Black,
            }), None, None, None, None, None, None, None,
        ],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
    ];

pub const test_board1 : rust_chess::board::Board = rust_chess::board::Board {
    positions: test_positions1,
    player_to_move: Color::White,
    last_move: None,
};

const test_positions2: [[Option<Piece>; 8]; 8] = [        
        [
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White,
            }), None, None, None, None, None, None, 
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::Black,
            }), 
        ],
        [
            None, None, Some(Piece {
                piece_type: PieceType::Queen,
                color: Color::Black,
            }), None, None, None, None, None,
        ],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
    ];

pub const test_board2 : rust_chess::board::Board = rust_chess::board::Board {
    positions: test_positions2,
    player_to_move: Color::White,
    last_move: None,
};

const one_rook_position: [[Option<Piece>; 8]; 8] = [   
        [None; 8],
        [
            None, None, Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::White,
            }), None, None, None, None, None,
        ],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
    ];

pub const one_rook_board : rust_chess::board::Board = rust_chess::board::Board {
    positions: one_rook_position,
    player_to_move: Color::White,
    last_move: None,
};

const one_bishop_position: [[Option<Piece>; 8]; 8] = [   
        [None; 8],
        [
            None, None, Some(Piece {
                piece_type: PieceType::Bishop,
                color: Color::White,
            }), None, None, None, None, None,
        ],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
    ];

pub const one_bishop_board : rust_chess::board::Board = rust_chess::board::Board {
    positions: one_bishop_position,
    player_to_move: Color::White,
    last_move: None,
};