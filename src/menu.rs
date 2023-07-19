use std::io;

use crate::control::control::Controller;
use crate::game::Game;
use crate::control::local_human::LocalHuman;
use crate::control::random_bot::RandomBot;
use crate::control::minimax::MinimaxBot;
use crate::model::GameState;
use crate::view::UnicodeDisplay;


pub fn read_number() -> i32 {
    let mut buffer: String = String::new();
    let stdin = io::stdin();
    let Ok(_) = stdin.read_line(&mut buffer) else {
        return -1;
    };
    let s = buffer.trim();
    let Ok(number) = s.parse::<i32>() else {
        return -1;
    };
    return number;
}

fn opponent_menu() -> Box<dyn Controller> {
    println!("Play against:");
    println!(" 1 - Human");
    println!(" 2 - RandomBot");
    println!(" 3 - MinimaxBot");
    let controller: Box<dyn Controller> = match read_number() {
        1 => Box::new(LocalHuman),
        2 => Box::new(RandomBot),
        3 => Box::new(MinimaxBot::new(4)),
        _ => {
            println!("Invalid option\n");
            opponent_menu()
        }
    };
    return controller;
}

fn color_menu() -> u32 {
    println!("Play as:");
    println!(" 1 - White");
    println!(" 2 - Black");
    match read_number() {
        1 => 1,
        2 => 2,
        _ => {
            println!("Invalid option\n");
            color_menu()
        }
    }

}

pub fn menu() -> Game {
    let opponent_controller = opponent_menu();
    let [controller1, controller2]: [Box<dyn Controller>; 2] = match color_menu() {
        1 => [Box::new(LocalHuman), opponent_controller],
        2 => [opponent_controller, Box::new(LocalHuman)],
        _ => panic!() // unreachable
    };
    Game {
        game_state: GameState::new(),
        game_display: Box::new(UnicodeDisplay),
        controller1: controller1,
        controller2: controller2,
    }
}