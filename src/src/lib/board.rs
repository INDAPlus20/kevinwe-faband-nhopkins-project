//! # Board
//! 
//! This module contains the Board struct and related impl
use crate::card::Card;

/// ## Board
/// Layout of the board that the entire game plays by. This holds the logic for where cards are
/// stored and it is necessary for generation of graphics.
struct Board {
    field: [[Option<Card>; CELL_AMOUNT.0 as usize]; CELL_AMOUNT.1 as usize],
    //cards: Vec<Card>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            field: [[None; CELL_AMOUNT.0 as usize]; CELL_AMOUNT.1 as usize],
        }
    }
}