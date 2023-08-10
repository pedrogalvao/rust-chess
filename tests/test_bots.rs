use rand::seq::SliceRandom;
use rust_chess::controllers::minimax::MinimaxBot;
use rust_chess::controllers::random_bot::RandomBot;
use rust_chess::game::Game;
use rust_chess::model::{game_state::GameState, movement::Movement, piece::Color};
use rust_chess::rules::cmd_validator::is_valid_movement;
use rust_chess::rules::game_over::{is_draw, is_in_check_mate};
use rust_chess::rules::move_generator::generate_movements;
use rust_chess::view::{GameDisplay, NoDisplay};
use rust_chess::{controllers::controller::Controller, game::GameResult};
use std::thread;

#[cfg(test)]
mod tests {
    use rust_chess::{
        controllers::alphabeta::AlphaBetaBot, model::piece::PieceType, view::AsciiDisplay,
    };

    use super::*;

    fn is_valid_position(game_state: &GameState) -> bool {
        let mut king_count = 0;
        let mut white_rook_count = 0;
        let mut black_rook_count = 0;
        for row in game_state.board {
            for square in row {
                match square {
                    Some(piece) if piece.piece_type == PieceType::King => {
                        king_count += 1;
                    }
                    Some(piece) if piece.piece_type == PieceType::Rook => match piece.color {
                        Color::White => {
                            white_rook_count += 1;
                        }
                        Color::Black => {
                            black_rook_count += 1;
                        }
                    },
                    _ => {}
                }
            }
        }
        return king_count == 2 && white_rook_count <= 2 && black_rook_count <= 2;
    }

    #[test]
    fn random_games() {
        // verify that all generated movements are valid
        let game_display: NoDisplay = NoDisplay {};
        for _ in 0..200 {
            let mut game_state: GameState = GameState::new960();
            for i in 0..100 {
                let movements: Vec<Movement> = generate_movements(&game_state);
                for movement in &movements {
                    assert_eq!(is_valid_movement(&movement, &game_state), true);
                }
                if let Some(chosen_move) = movements.choose(&mut rand::thread_rng()) {
                    game_state.make_movement(chosen_move.clone());
                    if !is_valid_position(&game_state) {
                        AsciiDisplay.display_game(&game_state);
                        dbg!(&game_state.last_move);
                        panic!();
                    }
                } else {
                    println!("Game over {}", i);
                    game_display.display_game(&game_state);
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
        }
    }

    fn bot_vs_bot<T1: Controller + Clone + 'static, T2: Controller + Clone + 'static>(
        bot1: T1,
        bot2: T2,
        n_games: u32,
    ) -> [u32; 3] {
        let mut n_victories = 0;
        let mut n_draws = 0;
        let mut n_defeats = 0;
        for _ in 0..n_games {
            let mut game: Game = Game::new(
                GameState::new960(),
                Box::new(NoDisplay),
                [Box::new(bot1.clone()), Box::new(bot2.clone())],
            );

            let result = game.play();
            match result {
                GameResult::Winner(Color::White) => {
                    n_victories += 1;
                }
                GameResult::Draw => {
                    n_draws += 1;
                }
                GameResult::Winner(Color::Black) => {
                    n_defeats += 1;
                }
            };
        }
        dbg!(n_victories);
        dbg!(n_draws);
        dbg!(n_defeats);
        return [n_victories, n_draws, n_defeats];
    }

    #[test]
    fn test_games() {
        let handle1 = thread::spawn(move || {
            let results = bot_vs_bot(MinimaxBot::new(3), RandomBot, 20);
            assert!(results[0] >= 18);
        });
        let handle2 = thread::spawn(move || {
            let results = bot_vs_bot(MinimaxBot::new(3), MinimaxBot::new(2), 20);
            assert!(results[0] >= results[2]);
        });
        let handle3 = thread::spawn(move || {
            let results = bot_vs_bot(MinimaxBot::new(4), MinimaxBot::new(3), 5);
            assert!(results[0] >= results[2]);
        });
        let handle4 = thread::spawn(move || {
            let results = bot_vs_bot(AlphaBetaBot::new(3), MinimaxBot::new(2), 20);
            assert!(results[0] >= results[2]);
        });
        handle1.join().unwrap();
        handle2.join().unwrap();
        handle3.join().unwrap();
        handle4.join().unwrap();
    }
}
