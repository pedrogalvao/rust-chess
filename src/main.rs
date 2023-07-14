mod board;
mod bot;
mod control;
mod game;
mod move_generator;
mod movement;
mod rules;
mod view;

use control::control::LocalHuman;
use crate::game::Game;
use crate::view::{AsciiDisplay, GameDisplay, UnicodeDisplay};

use std::io;


fn main() {
    let mut game: Game<UnicodeDisplay, LocalHuman> = Game {
        game_state: board::GameState::new(),
        game_display: UnicodeDisplay,
        controller1: LocalHuman,
        controller2: LocalHuman,
    };
    game.play();
}
