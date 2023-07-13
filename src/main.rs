mod board;
mod move_generator;
mod movement;
mod rules;
use std::io;

fn main() {
    let mut b: board::GameState = board::GameState::new();
    println!("{}", b);
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        let _ = stdin.read_line(&mut buffer);
        let Ok(m) = movement::Movement::from_str(&buffer, &b) else {
            buffer = String::new();
            println!("Invalid move");
            println!("{}", b);
            continue;
        };
        b.make_movement(m);
        if rules::is_in_check(&b, b.player_to_move) {
            println!("Check!");
        }
        println!("{}", b);
        buffer = String::new();
    }
}
