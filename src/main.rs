mod control;
mod evaluation;
mod game;
mod menu;
mod model;
mod rules;
mod view;

use crate::game::Game;
use menu::main_menu;

fn main() {
    let mut game: Game = main_menu();
    game.play();
}
