use crate::model::GameState;
use crate::movement::Movement;
use crate::{evaluation::evaluate_state, movement};

use crate::rules::move_generator::generate_movements;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::mem;

use super::control::{Command, Controller};

//#[derive(Clone)]
struct MinimaxTree {
    movement: Option<Movement>,
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

#[derive(Eq, PartialEq)]
struct MovementScore {
    movement: Movement,
    score: i32,
}

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

pub struct MinimaxBot {
    tree: MinimaxTree,
}

impl MinimaxBot {
    pub fn new() -> Self {
        Self {
            tree: MinimaxTree {
                movement: None,
                score: 0,
                game_state: GameState::new(),
                children: BinaryHeap::new(),
            },
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

    fn expand_node(&mut self) {
        let mut possible_moves = generate_movements(&self.game_state);
        let mut rng = thread_rng();
        possible_moves.shuffle(&mut rng);
        for movement in possible_moves {
            let mut game_state2 = self.game_state.clone();
            game_state2.make_movement(movement.clone());
            let score = evaluate_state(&game_state2, self.game_state.player_to_move);
            self.children.push(MinimaxTree {
                movement: Some(movement),
                score,
                game_state: game_state2,
                children: BinaryHeap::new(),
            });
        }
        // update score
        if let Some(child) = self.children.peek() {
            self.score = -child.score;
        } else {
            // TODO: verify draw or check mate
            self.score = -100;
        }
    }

    fn expand_leaves(&mut self) {
        if self.children.len() == 0 {
            self.expand_node();
        } else {
            let mut reordered_children = BinaryHeap::new();
            let limit = 25;
            let mut i = 0;
            while let Some(mut child) = self.children.pop() {
                if i < limit {
                    child.expand_leaves();
                    i += 1;
                }
                reordered_children.push(child);
            }
            self.children = reordered_children;
            self.score = self.children.peek().unwrap().score;
        }
    }
}

impl MinimaxBot {
    fn update_tree(&mut self, game_state: &GameState) {
        if self.tree.children.len() == 0 {
            self.tree = MinimaxTree {
                movement: None,
                score: evaluate_state(game_state, game_state.player_to_move),
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

        // movement was not in the tree
        self.tree = MinimaxTree {
            movement: None,
            score: evaluate_state(game_state, game_state.player_to_move),
            game_state: game_state.clone(),
            children: BinaryHeap::new(),
        };
    }

    fn choose_move(&mut self, game_state: &GameState) -> Movement {
        if *game_state != self.tree.game_state {
            self.update_tree(game_state);
        }
        for _ in 0..2 {
            self.tree.expand_leaves();
        }
        // while self.tree.get_depth() < 5 {
        //     self.tree.expand_leaves();
        // }
        dbg!(self.tree.get_depth());
        let chosen_child = self.tree.children.pop().unwrap();
        let chosen_movement = chosen_child.movement.clone().unwrap();
        self.tree = chosen_child;
        return chosen_movement;
    }
}

impl Controller for MinimaxBot {
    fn choose_command(&mut self, game_state: &mut GameState) -> super::control::Command {
        return Command::Move(self.choose_move(game_state));
    }
}
