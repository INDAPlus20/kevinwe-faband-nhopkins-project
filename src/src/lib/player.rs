//! # Player
//!
//! The `Player` crate holds logic related to the players of the game

use crate::card::CType::*;
use crate::card::{CType, Card};
use crate::traits::{Effect, Target};
/**
 * Imports
 */
use crate::CardPosition;
//mod card;
//mod pile;
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
    pub health: isize,
    /// The player's hand of cards
    pub hand: Vec<Card>,
    /// The special ability or trait of the player
    pub special_ability: (Effect, isize),
}

// methods of Player are implemented here
impl Player {
    /// play plays a card in the player's hand, optionally supply targets
    ///
    /// card : the card
    /// card_targets is either a Card, Player, or ... which inherits the Target trait
    fn play(&self, cardindex: usize, target: Vec<CType>) {
        //put card in new position, or affect targets
        // This needs to be fixed, commenting it out for now
        /*
        if self.hand[cardindex].ctype.contains(Person){
            if target.contains.is_none(){
                //puts the card in the hole
                target.containes = Some(self.hand(cardindex));
            }
            // sacrifice the card to add mana to target
            else {
                target.containes.unwrap().mana += 1;
            }
        }
        else {
            if target.containes.is_none(){
                //this shouldn't work
                println!("Bruh you can't play that without a target");
            }
            else {
                for effect in self.hand(cardindex).effects{
                    target.apply_effect(effect.0, effect.1);
                }
            }
        }
        //remove card from hand
        self.hand.remove(cardindex);
        */
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
