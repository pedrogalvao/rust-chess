mod control;
mod evaluation;
mod game;
mod menu;
mod model;
mod movement;
mod rules;
mod view;

use crate::game::Game;
use menu::menu;

fn main() {
    let mut game: Game = menu();
    game.play();
}
