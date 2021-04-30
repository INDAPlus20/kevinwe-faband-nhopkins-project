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
    special_ability: Vec<(String, int,)>,
}

// methods of Player are implemented here
impl Game {

    /// play plays a card in the player's hand, optionally supply targets
    ///
    /// card : the
    /// card_targets is either a Card, Player, or ... which inherits the Target trait
    fn play(card: Option<Card>, card_targets: Option<&Target>) -> Result<T, E> {
        // pass
    }


}


/**
 * main
 */

/// main holds logic for
/// settings and game setup as well as
/// the main game loop, where all objects are instantiated.
fn main() {

    let game = Game {
        current_player: (),
        in_play: (),
    };

    // main game loop
    loop {
        // do stuff with Game
        break;
    }

    println!("Hello, world!");
}