use crate::model::{Color, GameState, Piece};

pub trait GameDisplay {
    fn display_line() {
        print!("\n  ");
        for _ in 0..8 {
            print!("|---");
        }
        print!("|\n");
    }

    fn choose_ranges(game_state: &GameState) -> [Vec<usize>; 2] {
        let range1: Vec<usize>;
        if game_state.player_to_move == Color::White {
            range1 = (0..8).rev().collect();
        } else {
            range1 = (0..8).collect();
        }
        let range2: Vec<usize>;
        if game_state.player_to_move == Color::White {
            range2 = (0..8).collect();
        } else {
            range2 = (0..8).rev().collect();
        }
        return [range1, range2];
    }

    fn display_game(&self, game_state: &GameState) {
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let _ = match game_state.player_to_move {
            Color::White => print!("White to move:\n  "),
            Color::Black => print!("Black to move:\n  "),
        };
        if game_state.player_to_move == Color::White {
            for c in 'a'..='h' {
                print!("  {} ", c);
            }
        } else {
            for c in ('a'..='h').rev() {
                print!("  {} ", c);
            }
        }
        Self::display_line();
        let [range1, range2] = Self::choose_ranges(game_state);
        for i in range1 {
            print!("{} ", 1 + i);
            for j in &range2 {
                let piece_opt: Option<Piece> = game_state.board[i][*j];

                let piece_char: char = match piece_opt {
                    Some(piece) => self.piece_to_char(&piece),
                    None => ' ',
                };
                print!("| {} ", piece_char);
            }
            Self::display_line();
        }
    }
    fn piece_to_char(&self, piece: &Piece) -> char;
}

pub struct UnicodeDisplay;
impl GameDisplay for UnicodeDisplay {
    fn piece_to_char(&self, piece: &Piece) -> char {
        piece.to_unicode()
    }
}

pub struct AsciiDisplay;
impl GameDisplay for AsciiDisplay {
    fn piece_to_char(&self, piece: &Piece) -> char {
        piece.to_ascii()
    }
}

// impl fmt::Display for GameState {
//     fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         todo!()
//     }
// }

pub struct NoDisplay;
impl GameDisplay for NoDisplay {
    fn display_game(&self, _game_state: &GameState) {
        return;
    }
    fn piece_to_char(&self, _piece: &Piece) -> char {
        return ' ';
    }
}
