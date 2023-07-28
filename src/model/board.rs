use super::piece::{Color, Piece, PieceType};

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
