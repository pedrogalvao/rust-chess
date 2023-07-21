use rand::seq::SliceRandom;
use rand::Rng;
use rust_chess::control::minimax::MinimaxBot;
use rust_chess::control::random_bot::RandomBot;
use rust_chess::game::Game;
use rust_chess::model::{Color, GameState};
use rust_chess::movement::Movement;
use rust_chess::rules::cmd_validator::is_valid_movement;
use rust_chess::rules::game_over::{is_draw, is_in_check_mate};
use rust_chess::rules::move_generator::generate_movements;
use rust_chess::view::{GameDisplay, NoDisplay};
use std::thread;

#[cfg(test)]
mod tests {
    use std::{clone, time::{Duration, Instant}, thread::JoinHandle};

    use rust_chess::{
        game::GameResult,
        view::AsciiDisplay, control::control::Controller,
    };

    use super::*;

    #[test]
    fn random_games() {
        // verify that all generated movements are valid
        let game_display: NoDisplay = NoDisplay {};
        for _ in 0..10 {
            let mut game_state: GameState = GameState::new();
            for i in 0..100 {
                let movements: Vec<Movement> = generate_movements(&game_state);
                for movement in &movements {
                    assert_eq!(is_valid_movement(&movement, &game_state), true);
                }
                if let Some(chosen_move) = movements.choose(&mut rand::thread_rng()) {
                    game_state.make_movement(chosen_move.clone());
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


    fn bot_vs_bot<T1: Controller + Clone + 'static, T2: Controller + Clone + 'static>(bot1: T1, bot2: T2, n_games: u32) -> [u32; 3] {
        let mut n_victories = 0;
        let mut n_draws = 0;
        let mut n_defeats = 0;
        for _ in 0..n_games {
            let mut game: Game = Game {
                game_state: GameState::new(),
                game_display: Box::new(NoDisplay),
                controllers: [Box::new(bot1.clone()), Box::new(bot2.clone())],
                history: vec![],
            };
            let result = game.play();
            match result {
                GameResult::Winner(Color::White) => {
                    n_victories += 1;
                }
                GameResult::Draw => {
                    n_draws += 1;
                }
                GameResult::Winner(Color::Black) => {
                    // println!("Defeat:");
                    // AsciiDisplay.display_game(&game.game_state);
                    n_defeats += 1;
                }
            };
        }
        dbg!(n_victories);
        dbg!(n_draws);
        dbg!(n_defeats);
        return [n_victories, n_draws, n_defeats];
    }

    // fn join_timeout(handle: JoinHandle<()>, duration: Duration) -> Result<(),()> {
    //     let ini = Instant::now();
    //     while handle.try_join().is_err() {
    //         if Instant::now() - ini > duration {
    //             return Err(());
    //         }
    //         thread::sleep(Duration::from_millis(50));
    //     }
    //     Ok(())
    // }

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
        // join_timeout(handle1, Duration::from_secs(60)).unwrap();
        // join_timeout(handle2, Duration::from_secs(60)).unwrap();
        handle1.join().unwrap();
        handle2.join().unwrap();
        handle3.join().unwrap();
    }
}
