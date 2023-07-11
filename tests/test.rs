
use rust_chess::board::{Board, Color, Piece, PieceType};
use rust_chess::move_generator::generate_movements;
use rust_chess::rules::is_valid_movement;
use rust_chess::movement::Movement;
use rand::seq::SliceRandom; // 0.7.2

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn random_games() {
        for j in 0..100 {
            let mut b : Board = Board::new();
            for i in 0..200 {
                let mut movements : Vec<Movement> = generate_movements(&b);
                for movement in &movements {
                    assert_eq!(is_valid_movement(&movement, &b), true);
                }
                if let Some(chosen_move) = movements.choose(&mut rand::thread_rng()) {
                    b.make_movement(chosen_move.clone());
                } else {
                    println!("Game over {}", i);
                    print!("{}", b);
                    break;
                }
            }
            println!("New game...");
        }
    }
}
