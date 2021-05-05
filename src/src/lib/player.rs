//! # Player
//!
//! The `Player` crate holds logic related to the players of the game

/**
 * Imports
 */

// Import crates
// use std::io::{self, Read, Write};
use Card

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
    health: isize,

    /// The player's hand of cards
    hand: Vec<Card>,

    /// The special ability or trait of the player
    special_ability: (Ability, isize)
}

/// 
enum Ability{
    Damage,
    Healing,
}

// methods of Player are implemented here
impl Player {

    /// play plays a card in the player's hand, optionally supply targets
    ///
    /// card : the card
    /// card_targets is either a Player or Position that implements Target
    fn play(card: Option<Card>, target: Option<&Target>) -> Result<T, E> {
        
    }
    /// pass() ends the current player's turn
    fn pass() {

    }
    /// use_special() uses the player's special ability
    /// target: a Player or Position that implements Target
    /// 
    /// basically a giant match statement
    fn use_special(target: Option<&Target>) -> Result<T, E> {
        match ability {
            Damage => target.modifyField("health", -1)
            Healing => target.modifyField("health", 1)
        }
    }
}


/**
 * main
 */

/// main method for testing
fn main() {
    
    println!("Hello, world!");
}