use crate::evaluation::{evaluate_game_over, evaluate_material};
use crate::model::movement::Movement;
use crate::model::{game_state::GameState, piece::PieceType};

use crate::rules::game_over::is_game_over;
use crate::rules::move_generator::generate_movements_for_player_ignoring_check;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct GameTree {
    pub game_state: GameState,
    pub score: i32,
    pub children: BinaryHeap<GameTree>,
}

impl PartialEq for GameTree {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl Eq for GameTree {}

impl PartialOrd for GameTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GameTree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl GameTree {
    pub fn get_depth(&self) -> u32 {
        let mut depth_heap = BinaryHeap::new();
        for child in &self.children {
            depth_heap.push(child.get_depth());
        }
        if let Some(n) = depth_heap.pop() {
            return n + 1;
        } else {
            return 0;
        }
    }

    fn is_king_capture(movement: &Option<Movement>, game_state: &GameState) -> bool {
        if let Some(Movement::Normal {
            from: _,
            to: [x, y],
        }) = movement
        {
            match game_state.board[*x][*y] {
                Some(piece) if piece.piece_type == PieceType::King => {
                    // Captured the king
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        false
    }

    fn expand_node(&mut self) -> Result<(), ()> {
        let mut possible_movements = generate_movements_for_player_ignoring_check(
            &self.game_state,
            self.game_state.player_to_move,
        );
        let mut rng = thread_rng();
        possible_movements.shuffle(&mut rng);
        for movement in possible_movements {
            let game_state2 = self.game_state.clone_and_move(movement);
            let score = evaluate_material(&game_state2, self.game_state.player_to_move);
            if Self::is_king_capture(&game_state2.last_move, &self.game_state) {
                return Err(());
            } else {
                self.children.push(GameTree {
                    score,
                    game_state: game_state2,
                    children: BinaryHeap::new(),
                });
            }
        }
        // update score
        if let Some(child) = self.children.peek() {
            self.score = -child.score;
        }
        return Ok(());
    }

    pub fn dfs(&mut self, depth_limit: u32, branch_limit: u32) -> Result<(), ()> {
        if depth_limit == 0 {
            self.score = -evaluate_material(&self.game_state, self.game_state.player_to_move);
            return Ok(());
        } else if depth_limit == 1 {
            match self.expand_node() {
                Err(()) => {
                    return Err(());
                }
                Ok(()) => {
                    if let Some(child) = self.children.peek() {
                        self.score = -child.score;
                    }
                    return Ok(());
                }
            }
        } else if self.children.len() == 0 {
            match self.expand_node() {
                Ok(()) => {}
                Err(()) => {
                    return Err(());
                }
            }
        }
        let mut reordered_children = BinaryHeap::new();
        let mut branch_count = 0;
        dbg!(self.children.peek().unwrap().score);
        while let Some(mut child) = self.children.pop() {
            if branch_count < branch_limit {
                match child.dfs(depth_limit - 1, branch_limit) {
                    Ok(()) => {
                        if child.children.len() == 0 {
                            if depth_limit > 1 && is_game_over(&child.game_state) {
                                child.score = -evaluate_game_over(
                                    &child.game_state,
                                    child.game_state.player_to_move,
                                );
                            }
                        }
                        reordered_children.push(child);
                    }
                    Err(()) => {
                        // invalid child node
                        continue;
                    }
                }
                branch_count += 1;
            } else {
                reordered_children.push(child);
            }
        }
        self.children = reordered_children;
        // update score
        if let Some(child) = self.children.peek() {
            self.score = -child.score;
        } else {
            self.score = -evaluate_game_over(&self.game_state, self.game_state.player_to_move);
        }
        return Ok(());
    }

    pub fn expand_leaves(&mut self, branch_limit: u32) -> Result<(), ()> {
        if self.children.len() == 0 {
            return self.expand_node();
        } else {
            let mut reordered_children = BinaryHeap::new();
            let mut branch_count = 0;
            while let Some(mut child) = self.children.pop() {
                if branch_count < branch_limit {
                    match child.expand_leaves(25) {
                        Ok(()) => {
                            if child.children.len() == 0 {
                                child.score = -evaluate_game_over(
                                    &child.game_state,
                                    child.game_state.player_to_move,
                                );
                            }
                            reordered_children.push(child);
                        }
                        Err(()) => {
                            // invalid child node
                            continue;
                        }
                    }
                    branch_count += 1;
                } else {
                    reordered_children.push(child);
                }
            }
            self.children = reordered_children;
            // update score
            if let Some(child) = self.children.peek() {
                self.score = -child.score;
            }
            return Ok(());
        }
    }

    pub fn alphabeta_search(
        &mut self,
        depth_limit: u32,
        branch_limit: u32,
        alpha: i32,
        beta: i32,
    ) -> Result<i32, ()> {
        if depth_limit == 0 {
            // If we've reached the maximum depth or a leaf node, evaluate the node and return its score.
            self.score = -evaluate_material(&self.game_state, self.game_state.player_to_move);
            return Ok(self.score);
        }
        self.children = BinaryHeap::new();
        match self.expand_node() {
            Ok(()) => {}
            Err(()) => {
                return Err(());
            }
        }
        if self.children.is_empty() {
            // No possibilities for next moves (game over)
            self.score = -evaluate_game_over(&self.game_state, self.game_state.player_to_move);
            return Ok(self.score);
        }

        let mut best_score = alpha;

        // Create a new BinaryHeap to hold the updated child nodes
        let mut updated_children = BinaryHeap::new();

        while let Some(mut child) = self.children.pop() {
            // Recursively call alphabeta on the child nodes with negated alpha and beta for the opposite player
            let Ok(_) = child.alphabeta_search(depth_limit - 1, branch_limit, -beta, -best_score) else {
                continue;
            };

            // Update the best_score using the maximum value (`.max()`)
            best_score = best_score.max(child.score);

            // Push the updated child back into the new BinaryHeap
            updated_children.push(child);
            if best_score >= beta {
                // Prune the remaining nodes as the current player has already found a better move.
                break;
            }
        }

        // Replace the children with the updated BinaryHeap
        self.children = updated_children;

        self.score = -best_score;
        Ok(best_score)
    }
}
