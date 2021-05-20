//! # Player
//!
//! The `Player` crate holds logic related to the players of the game

/**
 * Imports
 */

// Import crates
// use std::io::{self, Read, Write};
// ...

// Import foreign function interfaces (FFI)
extern "C" {
    fn rand() -> usize;
}

/**
 * Structs
 */

/// Player represents a player in the game
///
/// Keeps track of player stats, hand, special ability as well as player related methods.
pub struct Player {
    /// The health points of the player; zero means the player is dead and out of the game
    health: int,

    /// The player's hand of cards
    hand: Vec<Card>,

    /// The special ability or trait of the player
    special_ability: Vec<(String, int)>,
}
/**
 * main
 */

/// main method for testing
fn main() {
    println!("Hello, world!");
}
