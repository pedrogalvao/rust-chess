use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
                PieceType::Pawn => '♟',
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
