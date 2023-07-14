use crate::board::{Color, GameState};
use crate::control::control::Controller;
use crate::rules::{is_draw, is_in_check, is_in_check_mate};
use crate::view::GameDisplay;

pub struct Game<T1: GameDisplay, T2: Controller, T3: Controller> {
    pub game_state: GameState,
    pub game_display: T1,
    pub controller1: T2,
    pub controller2: T3,
}

impl<T1: GameDisplay, T2: Controller, T3: Controller> Game<T1, T2, T3> {
    fn is_game_over(&self) -> bool {
        if is_in_check(&self.game_state, self.game_state.player_to_move) {
            if is_in_check_mate(&self.game_state, self.game_state.player_to_move) {
                println!("Check mate!");
                return true;
            }
            println!("Check!");
        } else if is_draw(&self.game_state) {
            println!("Draw!");
            return true;
        }
        return false;
    }

    fn player_turn(&mut self) {
        self.game_display.display_game(&self.game_state);
        if self.game_state.player_to_move == Color::White {
            self.controller1.control(&mut self.game_state);
        } else {
            self.controller2.control(&mut self.game_state);
        }
    }

    pub fn play(&mut self) {
        self.game_display.display_game(&self.game_state);
        loop {
            self.player_turn();
            if self.is_game_over() {
                return;
            }
        }
    }
}
