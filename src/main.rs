mod board;
mod move_generator;
mod movement;
mod rules;
use std::io;

fn main() {
    let mut game_state: board::GameState = board::GameState::new();
    println!("{}", game_state);
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let _ = stdin.read_line(&mut buffer);
        let Ok(m) = movement::Movement::from_str(&buffer, &game_state) else {
            buffer = String::new();
            println!("Invalid move");
            println!("{}", game_state);
            continue;
        };
        game_state.make_movement(m);
        if rules::is_in_check(&game_state, game_state.player_to_move) {
            println!("Check!");
        }
        println!("{}", game_state);
        buffer = String::new();
    }
}
