use rust_chess::board::{Board, Color, Piece, PieceType};
use rust_chess::move_generator::generate_movements;
use rust_chess::movement::Movement;
use rust_chess::rules::{is_in_check, is_valid_movement};
mod boards;
use boards::*;
use rand::seq::SliceRandom; // 0.7.2

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games() {
        // verify that all generated movements are valid
        for j in 0..100 {
            let mut b: Board = Board::new();
            for i in 0..200 {
                let mut movements: Vec<Movement> = generate_movements(&b);
                for movement in &movements {
                    //print!("{}", b);
                    //dbg!(movement);
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
            // println!("New game...");
        }
    }

    #[test]
    fn test_is_in_check() {
        assert_eq!(is_in_check(&boards::test_board1, Color::White), true);
        assert_eq!(is_in_check(&boards::test_board1, Color::Black), false);
        assert_eq!(is_in_check(&boards::test_board2, Color::White), false);
        assert_eq!(is_in_check(&boards::test_board2, Color::Black), false);
    }

    #[test]
    fn test_rook() {
        let mut movements: Vec<Movement> = generate_movements(&boards::one_rook_board);
        assert_eq!(movements.len(), 14);
    }

    #[test]
    fn test_bishop() {
        let mut movements: Vec<Movement> = generate_movements(&boards::one_bishop_board);
        assert_eq!(movements.len(), 9);
    }
}
