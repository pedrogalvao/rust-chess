mod board;
mod movement;
mod rules;
use std::io;

fn main() {
    let mut b: board::Board = board::Board::new();
    println!("{}", b);
    //movement::Movement{source:[0,0], destination:[0,0]}.is_valid_movement(&b);

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
        b.make_movement(&m);
        println!("{}", b);
        buffer = String::new();
    }
}
