//! # Main

/**
 * Imports
 */

// Import crates
// ...
// use std::io::{self, Read, Write};
// ...

// Import foreign function interfaces (FFI)
extern "C" {
    fn rand() -> usize;
}


/**
 * Structs
 */

/// `Game` is instantiated in main and changes over the course of the main game loop
pub struct Game {
    /// `current_player` holds the `player_id` (`int`) of current player
    pub current_player: *Player,

    /// `in_play` holds the `Cards` that are currently in play on the board
    pub in_play: *Pile, 

    /// `on_stack` holds the stack of `Cards` on the board that are currently resolving their effects
    pub on_stack: *Pile,

    // more fields...
}

// methods of Game are implemented here
impl Game {
    // methods ...
}


/**
 * Main
 */

/// The entry point of the program
///
/// `main` holds logic for settings and game setup as well as
/// the main game loop, where all objects are instantiated.
fn main() {

    let game = Game {
        current_player: (),
        in_play: (),
        on_stack: ()
    };

    // main game loop
    loop {
        // do stuff with Game
        break;
    }

    println!("Hello, world!");
}