use crate::evaluation::evaluate_state;
use crate::model::{Color, GameState};
use crate::movement::Movement;

use crate::rules::move_generator::generate_movements;
use crate::view::{AsciiDisplay, GameDisplay};

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::control::{Command, Controller};

struct MinimaxTree {
    movement: Movement,
    game_state: GameState,
    score: f32,
    //parent: Option<MinimaxTree>,
    children: BinaryHeap<MinimaxTree>,
}

#[derive(Eq, PartialEq)]
struct MovementScore {
    movement: Movement,
    score: i32,
}

impl PartialOrd for MovementScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MovementScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

pub struct MinimaxBot;

impl MinimaxBot {
    fn choose_move(&self, game_state: &GameState) -> Movement {
        let mut possible_moves = generate_movements(game_state);
        let mut moves_heap: BinaryHeap<MovementScore> = BinaryHeap::new();

        let mut rng = thread_rng();
        possible_moves.shuffle(&mut rng);
        for movement in possible_moves {
            let mut game_state2 = game_state.clone();
            game_state2.make_movement(movement.clone());
            AsciiDisplay.display_game(&game_state2);
            let score = evaluate_state(&game_state2, game_state.player_to_move);
            moves_heap.push(MovementScore { movement, score });
        }
        return moves_heap.pop().unwrap().movement;
    }
}

impl Controller for MinimaxBot {
    fn choose_command(&self, game_state: &mut GameState) -> super::control::Command {
        return Command::Move(self.choose_move(game_state));
    }
}
