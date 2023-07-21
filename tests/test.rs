use rand::seq::SliceRandom;
use rand::Rng;
use rust_chess::control::minimax::MinimaxBot;
use rust_chess::control::random_bot::RandomBot;
use rust_chess::game::Game;
use rust_chess::model::{Color, GameState};
use rust_chess::movement::Movement;
use rust_chess::rules::cmd_validator::{is_in_check, is_valid_movement};
use rust_chess::rules::game_over::{is_draw, is_in_check_mate};
use rust_chess::rules::move_generator::generate_movements;
use rust_chess::view::{GameDisplay, NoDisplay};

#[cfg(test)]
mod tests {
    use rust_chess::{
        game::GameResult,
        model::{load_game_state_from_json, Piece, PieceType},
        rules::cmd_validator::{king_castle_is_valid, queen_castle_is_valid},
        view::AsciiDisplay,
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

    #[test]
    fn minimax_vs_random() {
        const N_GAMES: u32 = 100;
        let mut n_minimax_victories = 0;
        let mut n_draws = 0;
        let mut n_defeats = 0;
        for _ in 0..N_GAMES {
            let mut game: Game = Game {
                game_state: GameState::new(),
                game_display: Box::new(NoDisplay),
                controllers: [Box::new(MinimaxBot::new(3)), Box::new(RandomBot)],
                history: vec![],
            };
            let game_result = game.play();
            match game_result {
                GameResult::Winner(Color::White) => {
                    n_minimax_victories += 1;
                }
                GameResult::Draw => {
                    n_draws += 1;
                }
                GameResult::Winner(Color::Black) => {
                    println!("Defeat:");
                    AsciiDisplay.display_game(&game.game_state);
                    n_defeats += 1;
                }
            }
        }
        dbg!(n_minimax_victories);
        dbg!(n_draws);
        dbg!(n_defeats);
        assert!((n_defeats as f32) < N_GAMES as f32 * 0.02);
        assert!(n_minimax_victories as f32 > N_GAMES as f32 * 0.96);
    }

    #[test]
    fn test_is_in_check() {
        let mut game_state = GameState::new_empty();
        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(1, 3);
        let y: usize = rng.gen_range(1, 3);
        game_state.board[x][y] = Some(Piece {
            piece_type: PieceType::King,
            color: Color::White,
        });
        let x2: usize = rng.gen_range(5, 7);
        let y2: usize = rng.gen_range(5, 7);
        game_state.board[x2][y2] = Some(Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        });
        assert_eq!(is_in_check(&game_state, Color::White), false);
        assert_eq!(is_in_check(&game_state, Color::Black), false);
        game_state.board[x + 1][y + 1] = Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        });
        assert_eq!(is_in_check(&game_state, Color::White), true);
        assert_eq!(is_in_check(&game_state, Color::Black), false);
    }

    #[test]
    fn test_rook() {
        let mut one_rook_state = GameState::new_empty();
        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(0, 7);
        let y: usize = rng.gen_range(0, 7);
        one_rook_state.board[x][y] = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        });
        let movements: Vec<Movement> = generate_movements(&one_rook_state);
        assert_eq!(movements.len(), 14);
    }

    #[test]
    fn test_bishop() {
        let mut one_bishop_state = GameState::new_empty();
        one_bishop_state.board[1][2] = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        });
        let movements: Vec<Movement> = generate_movements(&one_bishop_state);
        assert_eq!(movements.len(), 9);
    }

    #[test]
    fn test_king() {
        let mut one_king_state = GameState::new_empty();
        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(1, 6);
        let y: usize = rng.gen_range(1, 6);
        one_king_state.board[x][y] = Some(Piece {
            piece_type: PieceType::King,
            color: Color::White,
        });
        let movements: Vec<Movement> = generate_movements(&one_king_state);
        assert_eq!(movements.len(), 8);
        one_king_state.board[x + 1][y + 1] = Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        });
        let movements2: Vec<Movement> = generate_movements(&one_king_state);
        assert_eq!(movements2.len(), 3);
    }

    #[test]
    fn test_en_passant() {
        let en_passant_state =
            load_game_state_from_json("tests/boards/en_passant_board.json").unwrap();
        assert_eq!(
            is_valid_movement(
                &Movement::Normal {
                    from: [4, 3],
                    to: [5, 4]
                },
                &en_passant_state
            ),
            true
        );
    }

    #[test]
    fn test_castle() {
        let mut game_state = GameState::new();
        assert_eq!(king_castle_is_valid(&game_state), false);
        assert_eq!(queen_castle_is_valid(&game_state), false);
        game_state.board[0][5] = None;
        game_state.board[0][6] = None;
        assert_eq!(king_castle_is_valid(&game_state), true);
        game_state.make_movement(Movement::Normal {
            from: [0, 4],
            to: [0, 5],
        });
        game_state.make_movement(Movement::Normal {
            from: [0, 5],
            to: [0, 4],
        });
        assert_eq!(king_castle_is_valid(&game_state), false);
        game_state.board[7][5] = None;
        game_state.board[7][6] = None;
        game_state.player_to_move = Color::Black;
        assert_eq!(king_castle_is_valid(&game_state), true);
        assert_eq!(queen_castle_is_valid(&game_state), false);
        game_state.board[7][1] = None;
        game_state.board[7][2] = None;
        game_state.board[7][3] = None;
        assert_eq!(queen_castle_is_valid(&game_state), true);
        game_state.make_movement(Movement::Normal {
            from: [0, 0],
            to: [7, 0],
        }); // take the rook
        assert_eq!(queen_castle_is_valid(&game_state), false);
        game_state.board[0][1] = None;
        game_state.board[0][2] = None;
        game_state.board[0][3] = None;
        game_state.player_to_move = Color::White;
        assert_eq!(queen_castle_is_valid(&game_state), false);
    }
}
