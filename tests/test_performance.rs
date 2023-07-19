use rust_chess::control::random_bot::RandomBot;
use rust_chess::game::Game;
use rust_chess::model::GameState;
use rust_chess::view::NoDisplay;
mod boards;

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
            controller1: Box::new(MinimaxBot::new(4)),
            controller2: Box::new(MinimaxBot::new(4)),
        };
        let mut times = vec![];
        let mut n_turns = 0;
        for _ in 0..3 {
            let now = Instant::now();
            game.player_turn();
            times.push(now.elapsed().as_millis());
            //evaluate_state(&game.game_state, game.game_state.player_to_move);
            n_turns += 1;
        }
        let total_time : u128 = times.iter().sum();
        println!("total time: {}", total_time);
        dbg!(total_time);
        println!("average time: {}", total_time/n_turns);
        dbg!(total_time/n_turns);
    }
}
