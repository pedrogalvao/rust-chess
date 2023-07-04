use crate::board::{Board, Piece, PieceType};
use crate::rules::is_valid_movement;

#[derive(Debug)]
pub struct Movement {
    pub source: [usize; 2],
    pub destination: [usize; 2],
}

impl Movement {
    pub fn get_piece(&self, b: &Board) -> Piece {
        let [x, y] = self.source;
        if let Some(piece) = b.positions[x][y] {
            return piece;
        } else {
            todo!()
        }
    }

    pub fn from_str(move_str: &String, b: &Board) -> Result<Movement, ()> {
        let mut chars = move_str.chars();
        let Some(piece_char) = chars.next() else {
            return Err(());
        };
        let piece: Piece = match piece_char {
            'K' | 'k' => Piece {
                piece_type: PieceType::King,
                color: b.player_to_move,
            },
            'Q' | 'q' => Piece {
                piece_type: PieceType::Queen,
                color: b.player_to_move,
            },
            'B' | 'b' => Piece {
                piece_type: PieceType::Bishop,
                color: b.player_to_move,
            },
            'N' | 'n' => Piece {
                piece_type: PieceType::Knight,
                color: b.player_to_move,
            },
            'R' | 'r' => Piece {
                piece_type: PieceType::Rook,
                color: b.player_to_move,
            },
            'P' | 'p' => Piece {
                piece_type: PieceType::Pawn,
                color: b.player_to_move,
            },
            _ => return Err(()),
        };
        let Some(letter) = chars.next() else {
            return Err(());
        };
        let Some(number_char) = chars.next() else {
            return Err(());
        };
        let column_number: usize = (letter as usize) - ('a' as usize);
        let row_number: usize = (number_char as usize) - ('1' as usize);
        let dest: [usize; 2] = [row_number, column_number];
        if column_number > 7 || row_number > 7 {
            return Err(());
        }
        dbg!(piece);
        for source in b.get_piece_positions(piece) {
            let m = Movement {
                source: source,
                destination: dest,
            };
            if is_valid_movement(&m, &b) {
                return Ok(Movement {
                    source: source,
                    destination: dest,
                });
            }
        }
        Err(())
    }
}
