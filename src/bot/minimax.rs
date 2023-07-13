use crate::move_generator::generate_movements;
use std::collections::BinaryHeap;

struct MinimaxTree {
    game_state: GameState,
    evaluation: f32,
    children: BinaryHeap<MinimaxTree>
}

fn chose_move(game_state: &GameState) {
    let possible_moves = generate_movements(game_state);
    let moves_heap: BinaryHeap<Movement> = BinaryHeap::new();
    for movement in possible_moves {
        let mut game_state2 = game_state.clone();
        game_state2.make_movement(movement);
        evaluate_state(game_state2);
    }
}

trait Bot {
    pub fn chose_move(game_state: &GameState);
}