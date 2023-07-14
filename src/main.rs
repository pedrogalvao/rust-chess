mod board;
mod bot;
mod control;
mod game;
mod move_generator;
mod movement;
mod rules;
mod view;

use crate::game::Game;
use crate::view::{AsciiDisplay, GameDisplay, UnicodeDisplay};
use control::local_human::LocalHuman;
use control::random_bot::RandomBot;

fn main() {
    let mut game: Game<UnicodeDisplay, LocalHuman, RandomBot> = Game {
        game_state: board::GameState::new(),
        game_display: UnicodeDisplay,
        controller1: LocalHuman,
        controller2: RandomBot,
    };
    game.play();
}
