//! # Board
//!
//! This module contains the Board struct and related impl
use crate::card::Card;
use ggez::graphics;

/// ## Board
/// Layout of the board that the entire game plays by. This holds the logic for where cards are
/// stored and it is necessary for generation of graphics.
const CELL_AMOUNT: (i32, i32) = (8, 20); //this is a very scuffed way to solve this issue, TODO: FIX

#[derive(Copy, Clone)]
pub struct Board {
    pub field: [[Option<Card>; CELL_AMOUNT.0 as usize]; CELL_AMOUNT.1 as usize],
    //cards: Vec<Card>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            field: [[None; CELL_AMOUNT.0 as usize]; CELL_AMOUNT.1 as usize],
        }
    }
}
