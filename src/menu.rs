use std::io;

use crate::controllers::alphabeta::AlphaBetaBot;
use crate::controllers::controller::Controller;
use crate::controllers::local_human::LocalHuman;
use crate::controllers::minimax::MinimaxBot;
use crate::controllers::random_bot::RandomBot;
use crate::game::Game;
use crate::model::game_state::{load_game_state_from_json, GameState};
use crate::view::UnicodeDisplay;

pub fn read_number() -> u32 {
    let mut buffer: String = String::new();
    let stdin = io::stdin();
    let Ok(_) = stdin.read_line(&mut buffer) else {
        return read_number();
    };
    let s = buffer.trim();
    let Ok(number) = s.parse::<u32>() else {
        return read_number();
    };
    return number;
}

fn opponent_menu() -> Box<dyn Controller> {
    println!("Play against:");
    println!(" 1 - Human");
    println!(" 2 - RandomBot");
    println!(" 3 - MinimaxBot");
    println!(" 4 - AlphaBetaBot");
    let controller: Box<dyn Controller> = match read_number() {
        1 => Box::new(LocalHuman),
        2 => Box::new(RandomBot),
        3 => {
            println!("Depth:");
            println!(" 2 - Easy");
            println!(" 5 - Medium");
            println!(" * - Other");
            Box::new(MinimaxBot::new(read_number()))
        }
        4 => {
            println!("Depth:");
            println!(" 2 - Easy");
            println!(" 5 - Medium");
            println!(" * - Other");
            Box::new(AlphaBetaBot::new(read_number()))
        }
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

pub fn load_menu() -> GameState {
    println!(" 1 - Start new game");
    println!(" 2 - Open saved game");
    match read_number() {
        1 => {
            println!("Game type");
            println!(" 1 - Normal");
            println!(" 2 - Fischer Random Chess (960)");
            match read_number() {
                1 => {
                    return GameState::new();
                },
                2 => {
                    return GameState::new960();
                },
                _ => {
                    println!("Invalid option\n");
                    load_menu()
                }
            }
        }
        2 => {
            println!("Type file path:");
            let mut file_path: String = String::new();
            let stdin: io::Stdin = io::stdin();
            let Ok(_) = stdin.read_line(&mut file_path) else {
                println!("Error");
                return load_menu();
            };
            let Ok(game_state) = load_game_state_from_json(file_path.trim()) else {
                println!("No such file");
                return load_menu();
            };
            game_state
        }
        _ => {
            println!("Invalid option\n");
            load_menu()
        }
    }
}

pub fn main_menu() -> Game {
    let game_state = load_menu();
    let opponent_controller = opponent_menu();
    let controllers: [Box<dyn Controller>; 2] = match color_menu() {
        1 => [Box::new(LocalHuman), opponent_controller],
        2 => [opponent_controller, Box::new(LocalHuman)],
        _ => panic!(), // unreachable
    };
    Game::new(game_state, Box::new(UnicodeDisplay), controllers)
}
