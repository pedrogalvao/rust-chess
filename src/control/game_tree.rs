use crate::evaluation::{evaluate_game_over, evaluate_material};
use crate::model::movement::Movement;
use crate::model::{game_state::GameState, piece::PieceType};

use crate::rules::game_over::is_game_over;
use crate::rules::move_generator::generate_movements_for_player_ignoring_check;
use crate::view::{AsciiDisplay, GameDisplay};

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
            let mut game_state2 = self.game_state.clone();
            game_state2.make_movement(movement);
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

    pub fn dfs(
        &mut self,
        depth_limit: u32,
        branch_limit: u32,
        mut alpha: i32,
        mut beta: i32,
    ) -> Result<(), ()> {
        // if -self.score < alpha || -self.score > beta {
        //     return Ok(());
        // }
        if depth_limit == 0 {
            self.score = -evaluate_material(&self.game_state, self.game_state.player_to_move);
            return Ok(());
        } else if depth_limit == 1 {
            return self.expand_node();
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
        let mut best_score = alpha;
        while let Some(mut child) = self.children.pop() {
            if branch_count < branch_limit {
                match child.dfs(depth_limit - 1, branch_limit, -beta, -alpha) {
                    Ok(()) => {
                        if child.children.len() == 0 {
                            if is_game_over(&child.game_state) {
                                child.score = -evaluate_game_over(
                                    &child.game_state,
                                    child.game_state.player_to_move,
                                );
                            }
                            // else {
                            //     //continue;
                            //     child.score = -evaluate_material(
                            //         &child.game_state,
                            //         child.game_state.player_to_move,
                            //     );
                            // }
                        }
                        best_score = best_score.max(child.score);
                        if best_score > beta {
                            self.score = best_score;
                            return Ok(());
                        }
                        alpha = alpha.max(best_score);
                        assert!(beta >= alpha);
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
}
