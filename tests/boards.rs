use rust_chess::model::{Board, Color, GameState, Piece, PieceType};

const TEST_BOARD_1: Board = [
    [
        Some(Piece {
            piece_type: PieceType::King,
            color: Color::White,
        }),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        }),
    ],
    [
        Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        }),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
];

pub const TEST_STATE_1: GameState = GameState {
    board: TEST_BOARD_1,
    player_to_move: Color::White,
    last_move: None,
    can_castle_queen_side: true,
    can_castle_king_side: true,
};

const TEST_BOARD_2: Board = [
    [
        Some(Piece {
            piece_type: PieceType::King,
            color: Color::White,
        }),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        }),
    ],
    [
        None,
        None,
        Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        }),
        None,
        None,
        None,
        None,
        None,
    ],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
];

pub const TEST_STATE_2: GameState = GameState {
    board: TEST_BOARD_2,
    player_to_move: Color::White,
    last_move: None,
    can_castle_queen_side: true,
    can_castle_king_side: true,
};

const ONE_ROOK_BOARD: Board = [
    [None; 8],
    [
        None,
        None,
        Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        }),
        None,
        None,
        None,
        None,
        None,
    ],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
];

pub const ONE_ROOK_STATE: GameState = GameState {
    board: ONE_ROOK_BOARD,
    player_to_move: Color::White,
    last_move: None,
    can_castle_queen_side: true,
    can_castle_king_side: true,
};

const ONE_BISHOP_BOARD: Board = [
    [None; 8],
    [
        None,
        None,
        Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        }),
        None,
        None,
        None,
        None,
        None,
    ],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
];

pub const ONE_BISHOP_STATE: GameState = GameState {
    board: ONE_BISHOP_BOARD,
    player_to_move: Color::White,
    last_move: None,
    can_castle_queen_side: true,
    can_castle_king_side: true,
};

const ONE_KING_BOARD: Board = [
    [None; 8],
    [
        None,
        None,
        Some(Piece {
            piece_type: PieceType::King,
            color: Color::White,
        }),
        None,
        None,
        None,
        None,
        None,
    ],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
    [None; 8],
];

pub const ONE_KING_STATE: GameState = GameState {
    board: ONE_KING_BOARD,
    player_to_move: Color::White,
    last_move: None,
    can_castle_queen_side: true,
    can_castle_king_side: true,
};
