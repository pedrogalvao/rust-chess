use super::controller::{Command, Controller};
use super::game_tree::GameTree;
use crate::evaluation::evaluate_material;
use crate::model::game_state::GameState;
use crate::model::movement::Movement;
use crate::view::{AsciiDisplay, GameDisplay};

use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct AlphaBetaBot {
    depth: u32,
    tree: GameTree,
}

impl AlphaBetaBot {
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

impl AlphaBetaBot {
    fn update_tree(&mut self, game_state: &GameState) {
        // if self.tree.children.len() == 0 {
        //     self.tree = AlphaBetaTree {
        //         score: evaluate_material(game_state, game_state.player_to_move),
        //         game_state: game_state.clone(),
        //         children: BinaryHeap::new(),
        //     };
        //     return;
        // }

        // // look for corresponding tree node
        // while let Some(child) = self.tree.children.pop() {
        //     if child.game_state == *game_state {
        //         self.tree = child;
        //         return;
        //     }
        // }
        // AsciiDisplay.display_game(game_state);
        // dbg!(&game_state.last_move);
        // println!("Unexpected movement");
        // movement was not in the tree
        self.tree = GameTree {
            score: -evaluate_material(game_state, game_state.player_to_move),
            game_state: game_state.clone(),
            children: BinaryHeap::new(),
        };
    }

    fn choose_move(&mut self, game_state: &GameState) -> Movement {
        if *game_state != self.tree.game_state {
            self.update_tree(game_state);
        }
        self.tree.alphabeta_search(self.depth, 255, -20000, 20000).unwrap();
        let c = self.tree.children.pop();
        match c {
            None => {
                // dbg!(game_state);
                AsciiDisplay.display_game(game_state);
                panic!();
            }
            _ => {}
        }
        let chosen_child = c.unwrap();
        let chosen_movement = chosen_child.game_state.last_move.clone().unwrap();
        self.tree = chosen_child;
        return chosen_movement;
    }
}

impl Controller for AlphaBetaBot {
    fn choose_command(&mut self, game_state: &mut GameState) -> super::controller::Command {
        return Command::Move(self.choose_move(game_state));
    }
}
