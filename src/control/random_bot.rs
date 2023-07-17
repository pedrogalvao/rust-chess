use crate::control::control::Command;
use crate::control::control::Controller;
use crate::model::GameState;
use crate::rules::move_generator::generate_movements;
use rand::seq::SliceRandom;

pub struct RandomBot;

impl Controller for RandomBot {
    fn choose_command(&self, game_state: &mut GameState) -> super::control::Command {
        let movements = generate_movements(game_state);
        if let Some(chosen_move) = movements.choose(&mut rand::thread_rng()) {
            return Command::Move(chosen_move.clone());
        } else {
            // no moves are possible
            panic!();
        }
    }
}
