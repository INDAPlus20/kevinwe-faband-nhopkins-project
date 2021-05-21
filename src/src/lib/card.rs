/**
 * Imports
 */
use crate::player::Player;
use crate::traits::{Effect, PlayerType, Target};

/// Card positions
#[derive(Clone, Copy)]
pub enum CardPosition {
    Hand,
    Deck,
    Board,
}

/// Card types
#[derive(Clone, Copy)]
pub enum CType {
    Event,
    Person,
    Professor,
    Phoes,
    EECS,
    ABE,
    ITM,
    SCI,
    CCH,
}

/// holds all relevant data for a card
#[derive(Clone, Copy)]
pub struct Card {
    position: CardPosition,
    owner: Option<PlayerType>,
    used: bool,
    mana: isize,
    strength: isize,
    health: isize,
    // Until we grow smart, only 2 types ;_;
    pub ctype: (CType, CType),
    // For now, we are limited to one effect ;_;
    effects: (Effect, isize),
    // For now, I am not sure about text please god help
    //text: String,
}

impl Card {
    /// creates a new card, should use a helper function to not need to type as much.
    pub fn new(
        position: CardPosition,
        //owner: PlayerType, Consider assigning this later
        strength: isize,
        health: isize,
        ctype: (CType, CType),
        effects: (Effect, isize),
        /*text: String,*/
    ) -> Card {
        Card {
            position: position,
            owner: None,
            used: false,
            mana: 0,
            strength: strength,
            health: health,
            ctype: ctype,
            effects: effects,
            //text: text,
        }
    }
    /// given a target, applies its effects to that target
    fn play(&mut self, target: &mut impl Target) {
        // removed for loop here to make things function
        target.apply_effect(self.effects.0, self.effects.1);
        self.used = true;
    }
    /// Sets used to false, should be called at start of turn
    fn refresh(&mut self) {
        self.used = false;
    }
}

impl Target for Card {
    /// applies an instance of an effect with a scalar value to self
    fn apply_effect(&mut self, effect: Effect, value: isize) -> Result<isize, &'static str> {
        match effect {
            Effect::Damage => {
                self.health -= value;
                return Ok(value);
            }
            Effect::ModStrength => {
                self.strength += value;
                return Ok(value);
            }
            _ => return Err("The effect isn't implemented yet for this target type!"),
        }
    }
}
