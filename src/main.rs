// External Libs
use std::io::{stdin, Read, Write}; // for user input

use game::GameState;

// Internal Libs
mod game;
mod input;

fn main() {
    let mut state: GameState = GameState::new(2);
}
