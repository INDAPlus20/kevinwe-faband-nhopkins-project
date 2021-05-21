//! # Player
//!
//! The `Player` crate holds logic related to the players of the game

use crate::board::Board;
use crate::card::CType::*;
use crate::card::{CType, Card};
use crate::traits::{Effect, Target};
use ggez::graphics;
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
    /// The health points of the player; zero or less means the player is dead and out of the game
    pub health: isize,
    /// The player's hand of cards
    pub hand: Vec<(Card)>,
    /// The special ability or trait of the player
    pub special_ability: (Effect, isize),
}

// methods of Player are implemented here
impl Player {
    /// play plays a card in the player's hand, optionally supply targets
    ///
    /// cardindex : the index of the card to be played from the hand
    ///
    /// target_index : the index of the position on the board where the card
    /// is to be played
    fn play(
        &self,
        cardindex: usize,
        mut board: Board,
        target_index: (usize, usize),
    ) -> Result<(usize, usize), &'static str> {
        //put card in new position, or affect targets
        // This needs to be fixed, commenting it out for now
        match self.hand[cardindex].ctype.0 {
            Person => {
                if board.field[target_index.0][target_index.1].is_none() {
                    //puts the card in the hole
                    board.field[target_index.0][target_index.1] = Some(self.hand[cardindex]);
                }
                // sacrifice the card to add mana to target
                else {
                    board.field[target_index.0][target_index.1]
                        .unwrap()
                        .apply_effect(Effect::ModMana, 1);
                }
                return Ok(target_index);
            }
            Event => {
                if board.field[target_index.0][target_index.1].is_none() {
                    //this shouldn't work
                    println!("Bruh you can't play that without a target");
                }
                // apply effect to card at position
                else {
                    board.field[target_index.0][target_index.1]
                        .unwrap()
                        .apply_effect(
                            self.hand[cardindex].effects.0,
                            self.hand[cardindex].effects.1,
                        );
                }
                return Ok(target_index);
            }
            _ => {
                return Err("This card doesn't have a primary type!");
            }
        }
        //remove card from hand
        self.hand.remove(cardindex);
    }
    //draws a card from a given pile
    // Think this can be used in the main method instead
    /*
    fn draw(&self, amount: isize, pile: Vec<Card>){
        self.hand.push(pile.pop());
    }*/
    // Commented out for now to make compiling version
    /*
    /// Changed name from use since it clashed with pre existing names
    fn target_use(&self, target: &impl Target) {
        target.apply_effect(self.special_ability.0, self.special_ability.1);
        self.used = true;
    }*/
}
impl Target for Player {
    fn apply_effect(&mut self, effect: Effect, value: isize) -> Result<isize, &'static str> {
        match effect {
            Effect::Damage => {
                self.health = self.health - value;
                return Ok(value);
            }
            _ => Err("NYI!"),
        }
    }
}
// Eventual tests
