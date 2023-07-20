use rust_chess::game::Game;
use rust_chess::model::GameState;
use rust_chess::view::NoDisplay;

#[cfg(test)]
mod test {
    use std::time::Instant;

    use rust_chess::control::minimax::MinimaxBot;

    use super::*;

    #[test]
    fn test_minimax_time() {
        let mut game: Game = Game {
            game_state: GameState::new(),
            game_display: Box::new(NoDisplay),
            controllers: [Box::new(MinimaxBot::new(5)), Box::new(MinimaxBot::new(5))],
            history: vec![],
        };
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
}
