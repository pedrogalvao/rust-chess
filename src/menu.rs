use std::io;

use crate::controllers::alphabeta::AlphaBetaBot;
use crate::controllers::controller::Controller;
use crate::controllers::local_human::LocalHuman;
use crate::controllers::minimax::MinimaxBot;
use crate::controllers::random_bot::RandomBot;
use crate::controllers::remote_human::RemoteHuman;
use crate::game::Game;
use crate::model::game_state::{load_game_state_from_json, GameState};
use crate::model::piece::Color;
use crate::view::UnicodeDisplay;

fn read_number() -> u32 {
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

pub fn accept_undo_menu() -> bool {
    println!("The opponent wants to undo the last movement");
    println!("Accept? [y/n]");
    let mut buffer: String = String::new();
    let stdin = io::stdin();
    let Ok(_) = stdin.read_line(&mut buffer) else {
        return false;
    };
    let s = buffer.trim();
    match s {
        "y"|"Y" => true,
        "f"|"F" => false,
        _ => accept_undo_menu()
    }
}

fn opponent_menu(game_state: &GameState, opponent_color: Color) -> Box<dyn Controller> {
    println!("Play against:");
    println!(" 1 - Human");
    println!(" 2 - RandomBot");
    println!(" 3 - MinimaxBot");
    println!(" 4 - AlphaBetaBot");
    println!(" 5 - Remote Human");
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
        5 => {
            println!("Waiting for connection");
            let mut remote_human = RemoteHuman::new_listener(opponent_color);
            remote_human.reply_to_initial_messages(game_state);
            Box::new(remote_human)
        }
        _ => {
            println!("Invalid option\n");
            opponent_menu(game_state, opponent_color)
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

fn new_game_menu() -> GameState {
    println!("Game type");
    println!(" 1 - Normal");
    println!(" 2 - Fischer Random Chess (960)");
    match read_number() {
        1 => GameState::new(),
        2 => GameState::new960(),
        _ => {
            println!("Invalid option\n");
            new_game_menu()
        }
    }
}

fn load_game() -> GameState {
    println!("Type file path:");
    let mut file_path: String = String::new();
    let stdin: io::Stdin = io::stdin();
    let Ok(_) = stdin.read_line(&mut file_path) else {
        println!("Error");
        return load_game();
    };
    let Ok(game_state) = load_game_state_from_json(file_path.trim()) else {
        println!("No such file");
        return load_game();
    };
    game_state
}

fn join_host() -> Game {
    let mut buffer: String = String::new();
    let stdin = io::stdin();
    println!("Type host address");
    let _ = stdin.read_line(&mut buffer);
    println!("trying to connect to {}", buffer.as_str().trim());
    let Ok(mut remote_human) = RemoteHuman::new_client(buffer.as_str().trim()) else {
        // try again if connection fails
        return join_host();
    };
    let game_state = remote_human.get_game_state();
    println!("received game state");
    let controllers: [Box<dyn Controller>; 2] = match remote_human.color {
        Color::White => [Box::new(remote_human), Box::new(LocalHuman)],
        Color::Black => [Box::new(LocalHuman), Box::new(remote_human)],
    };
    return Game::new(game_state, Box::new(UnicodeDisplay), controllers);
}

pub fn main_menu() -> Game {
    println!(" 1 - Start new game");
    println!(" 2 - Open saved game");
    println!(" 3 - Join host");
    let n = read_number();
    if n == 1 || n == 2 {
        let game_state;
        if n == 1 {
            game_state = new_game_menu();
        } else {
            game_state = load_game();
        }
        let controllers: [Box<dyn Controller>; 2] = match color_menu() {
            1 => [
                Box::new(LocalHuman),
                opponent_menu(&game_state, Color::Black),
            ],
            2 => [
                opponent_menu(&game_state, Color::White),
                Box::new(LocalHuman),
            ],
            _ => panic!(), // unreachable
        };
        return Game::new(game_state, Box::new(UnicodeDisplay), controllers);
    } else if n == 3 {
        return join_host();
    } else {
        main_menu()
    }
}
