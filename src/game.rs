use crate::board::{Color, GameState};
use crate::control::control::Controller;
use crate::view::GameDisplay;

pub struct Game<T1: GameDisplay, T2: Controller, T3: Controller> {
    pub game_state: GameState,
    pub game_display: T1,
    pub controller1: T2,
    pub controller2: T3,
}

impl<T1: GameDisplay, T2: Controller, T3: Controller> Game<T1, T2, T3> {
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
        }
    }
}
