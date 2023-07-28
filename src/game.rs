use crate::control::control::{Command, Controller};
use crate::model::{
    game_state::write_game_state_to_json, game_state::GameState, movement::Movement, piece::Color,
};
use crate::rules::cmd_validator::is_valid_cmd;
use crate::rules::game_over::{is_draw, is_in_check_mate};
use crate::view::GameDisplay;

pub struct Game {
    pub game_state: GameState,
    pub game_display: Box<dyn GameDisplay>,
    pub controllers: [Box<dyn Controller>; 2],
    pub history: Vec<GameState>,
    move_limit: u32,
}

#[derive(PartialEq)]
pub enum GameResult {
    Winner(Color),
    Draw,
}

impl Game {
    pub fn new(
        game_state: GameState,
        game_display: Box<dyn GameDisplay>,
        controllers: [Box<dyn Controller>; 2],
    ) -> Self {
        Self {
            game_state,
            game_display,
            controllers,
            history: vec![],
            move_limit: 100, // limit of 50 moves per player without captures
        }
    }

    pub fn execute_command(&mut self, cmd: Command) {
        match cmd {
            Command::Move(movement) => {
                self.history.push(self.game_state.deepclone());
                match movement {
                    // check if it is a capture
                    Movement::Normal {
                        to: [x, y],
                        from: _,
                    } => {
                        if self.game_state.board[x][y] != None {
                            self.move_limit = 100;
                        }
                    }
                    _ => {}
                }
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

    pub fn play(&mut self) -> GameResult {
        self.game_display.display_game(&self.game_state);
        while self.move_limit > 0 {
            self.player_turn();
            if is_in_check_mate(&self.game_state, self.game_state.player_to_move) {
                self.game_display.display_game_over(&self.game_state);
                return GameResult::Winner(self.game_state.player_to_move.get_opponent_color());
            } else if is_draw(&self.game_state) {
                self.game_display.display_game_over(&self.game_state);
                self.game_display.display_game_over(&self.game_state);
                return GameResult::Draw;
            }
            self.move_limit -= 1;
        }
        return GameResult::Draw;
    }
}
