use crate::evaluation::evaluate_material;
use crate::model::{game_state::GameState, movement::Movement};
use crate::view::{AsciiDisplay, GameDisplay};

use std::collections::BinaryHeap;

use super::controller::{Command, Controller};
use super::game_tree::GameTree;

#[derive(Clone)]
pub struct MinimaxBot {
    depth: u32,
    tree: GameTree,
}

impl MinimaxBot {
    pub fn new(depth: u32) -> Self {
        Self {
            tree: GameTree {
                score: 0,
                game_state: GameState::new(),
                children: BinaryHeap::new(),
            },
            depth,
        }
    }
}

impl MinimaxBot {
    fn update_tree(&mut self, game_state: &GameState) {
        if self.tree.children.len() == 0 {
            self.tree = GameTree {
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
        self.tree = GameTree {
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
