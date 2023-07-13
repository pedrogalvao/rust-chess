use rust_chess::board::{Color, GameState};
use rust_chess::move_generator::generate_movements;
use rust_chess::movement::Movement;
use rust_chess::rules::{is_draw, is_in_check, is_in_check_mate, is_valid_movement};
mod boards;
use boards::*;
use rand::seq::SliceRandom; // 0.7.2

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_games() {
        // verify that all generated movements are valid
        for _ in 0..50 {
            let mut game_state: GameState = GameState::new();
            for i in 0..200 {
                let movements: Vec<Movement> = generate_movements(&game_state);
                for movement in &movements {
                    //print!("{}", b);
                    //dbg!(movement);
                    assert_eq!(is_valid_movement(&movement, &game_state), true);
                }
                if let Some(chosen_move) = movements.choose(&mut rand::thread_rng()) {
                    game_state.make_movement(chosen_move.clone());
                } else {
                    println!("Game over {}", i);
                    print!("{}", game_state);
                    let check_mate = is_in_check_mate(&game_state, game_state.player_to_move);
                    let draw = is_draw(&game_state);
                    if check_mate {
                        println!("{:?} won", game_state.player_to_move.get_opponent_color());
                    } else if draw {
                        println!("Draw");
                    }
                    assert_eq!(check_mate || draw, true);
                    break;
                }
            }
            // println!("New game...");
        }
    }

    #[test]
    fn test_is_in_check() {
        assert_eq!(is_in_check(&boards::TEST_STATE_1, Color::White), true);
        assert_eq!(is_in_check(&boards::TEST_STATE_1, Color::Black), false);
        assert_eq!(is_in_check(&boards::TEST_STATE_2, Color::White), false);
        assert_eq!(is_in_check(&boards::TEST_STATE_2, Color::Black), false);
    }

    #[test]
    fn test_rook() {
        let movements: Vec<Movement> = generate_movements(&ONE_ROOK_STATE);
        assert_eq!(movements.len(), 14);
    }

    #[test]
    fn test_bishop() {
        let movements: Vec<Movement> = generate_movements(&ONE_BISHOP_STATE);
        assert_eq!(movements.len(), 9);
    }

    #[test]
    fn test_king() {
        let movements: Vec<Movement> = generate_movements(&ONE_KING_STATE);
        assert_eq!(movements.len(), 8);
    }
}
