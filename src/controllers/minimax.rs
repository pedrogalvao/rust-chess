use crate::evaluation::{evaluate_game_over, evaluate_material};
use crate::model::{game_state::GameState, movement::Movement, piece::PieceType};
use crate::rules::move_generator::generate_movements_for_player_ignoring_check;
use crate::view::{AsciiDisplay, GameDisplay};

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::controller::{Command, Controller};

#[derive(Clone)]
struct MinimaxTree {
    game_state: GameState,
    score: i32,
    children: BinaryHeap<MinimaxTree>,
}

impl PartialEq for MinimaxTree {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl Eq for MinimaxTree {}

impl PartialOrd for MinimaxTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MinimaxTree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

#[derive(Clone)]
pub struct MinimaxBot {
    depth: u32,
    tree: MinimaxTree,
}

impl MinimaxBot {
    pub fn new(depth: u32) -> Self {
        Self {
            tree: MinimaxTree {
                score: 0,
                game_state: GameState::new(),
                children: BinaryHeap::new(),
            },
            depth,
        }
    }
}

impl MinimaxTree {
    fn get_depth(&self) -> u32 {
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
            if let Some(Movement::Normal {
                from: _,
                to: [x, y],
            }) = game_state2.last_move
            {
                match self.game_state.board[x][y] {
                    Some(piece) if piece.piece_type == PieceType::King => {
                        // Captured the king
                        return Err(());
                    }
                    _ => {
                        self.children.push(MinimaxTree {
                            score,
                            game_state: game_state2,
                            children: BinaryHeap::new(),
                        });
                    }
                }
            } else {
                // castling movements
                self.children.push(MinimaxTree {
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

    fn expand_leaves(&mut self, branch_limit: u32) -> Result<(), ()> {
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
}

impl MinimaxBot {
    fn update_tree(&mut self, game_state: &GameState) {
        if self.tree.children.len() == 0 {
            self.tree = MinimaxTree {
                score: evaluate_material(game_state, game_state.player_to_move),
                game_state: game_state.clone(),
                children: BinaryHeap::new(),
            };
            return;
        }

        // look for corresponding tree node
        while let Some(child) = self.tree.children.pop() {
            if child.game_state == *game_state {
                self.tree = child;
                return;
            }
        }
        AsciiDisplay.display_game(game_state);
        dbg!(&game_state.last_move);
        println!("Unexpected movement");
        // movement was not in the tree
        self.tree = MinimaxTree {
            score: evaluate_material(game_state, game_state.player_to_move),
            game_state: game_state.clone(),
            children: BinaryHeap::new(),
        };
    }

    fn choose_move(&mut self, game_state: &GameState) -> Movement {
        if *game_state != self.tree.game_state {
            self.update_tree(game_state);
        }
        for _ in 0..2 {
            match self.tree.expand_leaves(255) {
                Ok(_) => {}
                Err(_) => {
                    println!("Invalid state:");
                    AsciiDisplay.display_game(game_state);
                }
            };
        }
        while self.tree.get_depth() < self.depth {
            match self.tree.expand_leaves(25) {
                Ok(_) => {}
                Err(_) => {
                    println!("Invalid state:");
                    AsciiDisplay.display_game(game_state);
                }
            };
        }
        let chosen_child = self.tree.children.pop().unwrap();
        let chosen_movement = chosen_child.game_state.last_move.clone().unwrap();
        self.tree = chosen_child;
        return chosen_movement;
    }
}

impl Controller for MinimaxBot {
    fn choose_command(&mut self, game_state: &mut GameState) -> super::controller::Command {
        return Command::Move(self.choose_move(game_state));
    }
}
