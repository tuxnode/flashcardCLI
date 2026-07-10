use std::fs;
use std::io;

use crate::flashcard::FlashCard;

pub struct CardDeck {
    pub cards: Vec<FlashCard>,
    pub file_path: String,
}

impl CardDeck {
    // Load cards from JSON file, parsing to Vector
    // Return cards and file_path
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {}

    // Write cards information back to JSON file
    pub fn save() -> Result<(), Box<dyn std::error::Error>> {}

    // Return a random number
    pub fn pick_random(&self) -> &FlashCard {}

    // Update cords scros
    pub fn update_score(index: usize, correct: bool) {}
}
