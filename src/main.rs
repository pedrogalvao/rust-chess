mod control;
mod evaluation;
mod game;
mod model;
mod movement;
mod rules;
mod view;

use crate::game::Game;
use crate::view::UnicodeDisplay;
use control::local_human::LocalHuman;
use control::minimax::MinimaxBot;

fn main() {
    let mut game: Game<UnicodeDisplay, LocalHuman, MinimaxBot> = Game {
        game_state: model::GameState::new(),
        game_display: UnicodeDisplay,
        controller1: LocalHuman,
        controller2: MinimaxBot::new(),
    };
    game.play();
}
