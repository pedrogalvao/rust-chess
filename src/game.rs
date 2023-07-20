use crate::control::control::{Command, Controller};
use crate::model::{write_game_state_to_json, GameState};
use crate::rules::cmd_validator::is_valid_cmd;
use crate::rules::game_over::is_game_over;
use crate::view::GameDisplay;

pub struct Game {
    pub game_state: GameState,
    pub game_display: Box<dyn GameDisplay>,
    pub controllers: [Box<dyn Controller>; 2],
    pub history: Vec<GameState>,
}

impl Game {
    pub fn execute_command(&mut self, cmd: Command) {
        match cmd {
            Command::Move(movement) => {
                self.history.push(self.game_state.deepclone());
                self.game_state.make_movement(movement);
            }
            Command::Save => {
                let _ = write_game_state_to_json(&self.game_state, "game.json").unwrap();
            }
            Command::Undo => {
                let Some(_) = self.history.pop() else {
                    println!("Invalid command");
                    return;
                };
                let Some(previous_state) = self.history.pop() else {
                    println!("Invalid command");
                    return;
                };
                self.game_display.display_game(&self.game_state);
                self.game_display.display_game(&previous_state);
                self.game_state = previous_state;
            }
            Command::Resign => todo!(),
        }
    }

    pub fn player_turn(&mut self) {
        let cmd = self.controllers[self.game_state.player_to_move as usize]
            .choose_command(&mut self.game_state);
        if is_valid_cmd(&cmd, &self.game_state) {
            self.execute_command(cmd);
        }
        self.game_display.display_game(&self.game_state);
    }

    pub fn play(&mut self) {
        self.game_display.display_game(&self.game_state);
        loop {
            self.player_turn();
            if is_game_over(&self.game_state) {
                self.game_display.display_game_over(&self.game_state);
                return;
            }
        }
    }
}
