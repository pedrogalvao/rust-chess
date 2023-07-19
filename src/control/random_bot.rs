use crate::control::control::Command;
use crate::control::control::Controller;
use crate::model::GameState;
use crate::rules::move_generator::generate_commands;
use rand::seq::SliceRandom;

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
