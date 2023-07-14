use crate::board::{GameState, Piece, PieceType};
use crate::movement::Movement;
use crate::rules::{is_draw, is_in_check, is_in_check_mate, is_valid_movement};
use std::io;
use substring::Substring;

// enum Player {
//     LocalHuman,
//     RemoteHuman,
//     Bot
// }

pub enum Command {
    Undo,
    Resign,
    Save,
    Move(Movement),
}

pub trait Controller {
    fn control(&self, game_state: &mut GameState);
}

pub struct LocalHuman;
impl LocalHuman {
    pub fn parse_command(&self, cmd_str: &str, game_state: &GameState) -> Result<Command, ()> {
        match cmd_str {
            "undo" => Ok(Command::Undo),
            "resign" => Ok(Command::Resign),
            _ => match self.parse_movement(cmd_str, game_state) {
                Err(()) => Err(()),
                Ok(m) => Ok(Command::Move(m)),
            },
        }
    }

    fn str_to_position(position_str: &str) -> Result<[usize; 2], ()> {
        let mut chars = position_str.chars();
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
        if column_number > 7 || row_number > 7 {
            return Err(());
        }
        return Ok([row_number, column_number]);
    }

    pub fn parse_movement(&self, move_str: &str, game_state: &GameState) -> Result<Movement, ()> {
        let mut chars = move_str.chars();
        let piece: Piece;
        let move_str = move_str.replace(&['\n', '\r'][..], "");

        // Get the piece type
        if move_str.len() == 3 || move_str.len() == 6 {
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

        // In case the player indicated the source (e.g.: Bc6xf5):
        if move_str.contains("x") {
            let move_str_split: Vec<&str> = move_str.split("x").collect();
            if move_str_split.len() == 2 {
                let part1 = move_str_split[0];
                let source_str;
                let dest_str = move_str_split[1];
                if part1.len() == 3 {
                    source_str = part1.substring(1, 3);
                } else {
                    source_str = part1;
                }
                if source_str.len() == 2 && dest_str.len() == 2 {
                    let Ok(source) = Self::str_to_position(&source_str) else {return Err(());};
                    let Ok(dest) = Self::str_to_position(&dest_str) else {return Err(());};
                    let movement = Movement {
                        source: source,
                        destination: dest,
                    };
                    if is_valid_movement(&movement, &game_state) {
                        return Ok(movement);
                    }
                } else {
                    return Err(());
                }
            } else {
                return Err(());
            }
        }

        let Ok(dest) = Self::str_to_position(move_str.substring(move_str.len()-2, move_str.len())) else {
        return Err(());
    };
        for source in game_state.get_piece_positions(piece) {
            let movement = Movement {
                source: source,
                destination: dest,
            };
            if is_valid_movement(&movement, &game_state) {
                return Ok(movement);
            }
        }
        Err(())
    }
}

impl Controller for LocalHuman {
    fn control(&self, game_state: &mut GameState) {
        let mut buffer: String = String::new();
        let stdin = io::stdin();
        let _ = stdin.read_line(&mut buffer);
        let Ok(cmd) = self.parse_command(buffer.as_str(), game_state) else {
            println!("Invalid move");
            return;
        };
        if let Command::Move(m) = cmd {
            game_state.make_movement(m);
            if is_in_check(&game_state, game_state.player_to_move) {
                if is_in_check_mate(&game_state, game_state.player_to_move) {
                    println!("Check mate!");
                    return;
                }
                println!("Check!");
            } else if is_draw(&game_state) {
                println!("Draw!");
                return;
            }
        }
    }
}
