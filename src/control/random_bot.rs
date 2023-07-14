use crate::model::GameState;
use crate::control::control::Controller;
use crate::move_generator::generate_movements;
use rand::seq::SliceRandom;

pub struct RandomBot;

impl Controller for RandomBot {
    fn control(&self, game_state: &mut GameState) {
        let movements = generate_movements(game_state);
        if let Some(chosen_move) = movements.choose(&mut rand::thread_rng()) {
            game_state.make_movement(chosen_move.clone());
        } else {
            //panic!();
            return;
        }
    }
}
