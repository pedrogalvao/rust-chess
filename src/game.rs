use crate::control::control::Controller;
use crate::model::{Color, GameState};
use crate::rules::cmd_exec::execute_command;
use crate::rules::cmd_validator::is_valid_cmd;
use crate::rules::game_over::is_game_over;
use crate::view::GameDisplay;

pub struct Game {
    pub game_state: GameState,
    pub game_display: Box<dyn GameDisplay>,
    pub controller1: Box<dyn Controller>,
    pub controller2: Box<dyn Controller>,
}

impl Game {
    pub fn player_turn(&mut self) {
        self.game_display.display_game(&self.game_state);
        if self.game_state.player_to_move == Color::White {
            let cmd = self.controller1.choose_command(&mut self.game_state);
            if is_valid_cmd(&cmd, &self.game_state) {
                execute_command(cmd, &mut self.game_state);
            }
        } else {
            let cmd = self.controller2.choose_command(&mut self.game_state);
            if is_valid_cmd(&cmd, &self.game_state) {
                execute_command(cmd, &mut self.game_state);
            }
        }
    }

    pub fn play(&mut self) {
        self.game_display.display_game(&self.game_state);
        loop {
            self.player_turn();
            if is_game_over(&self.game_state) {
                return;
            }
        }
    }
}
