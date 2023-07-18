mod control;
mod evaluation;
mod game;
mod model;
mod movement;
mod rules;
mod view;
mod menu;

use crate::game::Game;
use menu::menu;

fn main() {
    let mut game: Game = menu();
    game.play();
}
