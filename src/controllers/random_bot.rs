use crate::controllers::controller::Command;
use crate::controllers::controller::Controller;
use crate::model::game_state::GameState;
use crate::rules::move_generator::generate_commands;
use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct RandomBot;

impl Controller for RandomBot {
    fn choose_command(&mut self, game_state: &mut GameState) -> Command {
        let commands = generate_commands(game_state);
        if let Some(chosen_command) = commands.choose(&mut rand::thread_rng()) {
            return chosen_command.clone();
        } else {
            // no moves are possible
            panic!();
        }
    }
}
