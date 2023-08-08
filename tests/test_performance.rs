use rust_chess::game::Game;
use rust_chess::model::game_state::GameState;
use rust_chess::view::NoDisplay;

#[cfg(test)]
mod test {
    use std::time::Instant;

    use rust_chess::controllers::{alphabeta::AlphaBetaBot, minimax::MinimaxBot};

    use super::*;

    #[test]
    fn test_minimax_time() {
        let mut game: Game = Game::new(
            GameState::new(),
            Box::new(NoDisplay),
            [Box::new(MinimaxBot::new(5)), Box::new(MinimaxBot::new(5))],
        );

        let mut times = vec![];
        let mut n_turns = 0;
        for _ in 0..5 {
            let now = Instant::now();
            game.player_turn();
            times.push(now.elapsed().as_millis());
            //evaluate_state(&game.game_state, game.game_state.player_to_move);
            n_turns += 1;
        }
        let total_time: u128 = times.iter().sum();
        println!("Minimax performance");
        println!("total time: {} ms", total_time);
        println!("average time: {} ms", total_time / n_turns);
        assert!(total_time / n_turns < 5000)
    }

    #[test]
    fn test_alphabeta_time() {
        let depth = 7;
        let mut game: Game = Game::new(
            GameState::new(),
            Box::new(NoDisplay),
            [
                Box::new(AlphaBetaBot::new(depth)),
                Box::new(AlphaBetaBot::new(depth)),
            ],
        );

        let mut times = vec![];
        let mut n_turns = 0;
        for _ in 0..5 {
            let now = Instant::now();
            game.player_turn();
            times.push(now.elapsed().as_millis());
            //evaluate_state(&game.game_state, game.game_state.player_to_move);
            n_turns += 1;
        }
        let total_time: u128 = times.iter().sum();
        println!("AlphaBeta performance depth");
        println!("depth: {}", depth);
        println!("total time: {} ms", total_time);
        println!("average time: {} ms", total_time / n_turns);
        assert!(total_time / n_turns < 5000)
    }
}
