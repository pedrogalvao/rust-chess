use crate::controllers::controller::Controller;
use crate::model::game_state::GameState;

use std::io;

use super::command_parser::CommandParser;

/// Controller for the program's user. Chooses commands based on input directly typed on the command line.
pub struct LocalHuman;

impl Controller for LocalHuman {
    fn choose_command(&mut self, game_state: &mut GameState) -> super::controller::Command {
        let parser = CommandParser;
        let mut buffer: String = String::new();
        let stdin = io::stdin();
        let _ = stdin.read_line(&mut buffer);
        let Ok(cmd) = parser.parse_command(buffer.as_str(), game_state) else {
            println!("Invalid move");
            return self.choose_command(game_state);
        };
        return cmd;
    }
}
