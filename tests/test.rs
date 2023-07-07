
use rust_chess::board::{Board, Color, Piece, PieceType};
use rust_chess::move_generator::generate_movements;
use rust_chess::rules::is_valid_movement;
use rust_chess::movement::Movement;
use rand::seq::SliceRandom; // 0.7.2

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn init_board() {
        let mut b : Board = Board::new();
        let mut movements : Vec<Movement> = generate_movements(&b);
        for i in 0..1000 {
            for movement in &movements {
                if is_valid_movement(&movement, &b) != true {
                    dbg!(movement.clone());
                    println!("{}", b);
                    let mut b2 = b.clone();
                    b2.make_movement(movement.clone());
                    println!("{}", b2);
                    assert_eq!(false, true);
                }
                assert_eq!(is_valid_movement(&movement, &b), true);
            }
            if let Some(chosen_move) = movements.choose(&mut rand::thread_rng()) {
                b.make_movement(chosen_move.clone());
            }
        }
    }
}
