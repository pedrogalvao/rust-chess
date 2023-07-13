use crate::board::{GameState, Piece, PieceType};
use crate::rules::is_valid_movement;

#[derive(Debug, Clone)]
pub struct Movement {
    pub source: [usize; 2],
    pub destination: [usize; 2],
}

impl Movement {
    pub fn get_piece(&self, game_state: &GameState) -> Piece {
        let [x, y] = self.source;
        if let Some(piece) = game_state.board[x][y] {
            return piece;
        } else {
            todo!()
        }
    }

    pub fn from_str(move_str: &String, game_state: &GameState) -> Result<Movement, ()> {
        let mut chars = move_str.chars();
        let piece: Piece;
        let move_str = move_str.replace(&['\n', '\r'][..], "");
        if move_str.len() >= 3 {
            // The player indicated the piece type
            let Some(piece_char) = chars.next() else {
                return Err(());
            };
            match Piece::from_char(piece_char, game_state.player_to_move) {
                Ok(piece2) => piece = piece2,
                _ => {
                    return Err(());
                }
            };
        } else {
            // Assume the piece is a pawn
            piece = Piece {
                piece_type: PieceType::Pawn,
                color: game_state.player_to_move,
            }
        }
        let Some(letter) = chars.next() else {
            return Err(());
        };
        let Some(number_char) = chars.next() else {
            return Err(());
        };
        if letter < 'a' || letter > 'h' {
            return Err(());
        }
        let column_number: usize = (letter as usize) - ('a' as usize);
        if number_char < '1' || number_char > '8' {
            return Err(());
        }
        let row_number: usize = (number_char as usize) - ('1' as usize);
        let dest: [usize; 2] = [row_number, column_number];
        if column_number > 7 || row_number > 7 {
            return Err(());
        }
        for source in game_state.get_piece_positions(piece) {
            let movement = Movement {
                source: source,
                destination: dest,
            };
            if is_valid_movement(&movement, &game_state) {
                return Ok(Movement {
                    source: source,
                    destination: dest,
                });
            }
        }
        Err(())
    }
}
