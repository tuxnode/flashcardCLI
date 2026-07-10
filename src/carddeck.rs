use std::fs::File;
use std::io::BufReader;

use crate::flashcard::FlashCard;

pub struct CardDeck {
    pub cards: Vec<FlashCard>,
    pub file_path: String,
}

impl CardDeck {
    // Load cards from JSON file, parsing to Vector
    // Return cards and file_path
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let cards: Vec<FlashCard> = serde_json::from_reader(reader)?;
        Ok(CardDeck {
            cards: cards,
            file_path: path.to_string(),
        })
    }

    // Write cards information back to JSON file
    pub fn save() -> Result<(), Box<dyn std::error::Error>> {}

    // Return a random number
    pub fn pick_random(&self) -> &FlashCard {}

    // Update cords scros
    pub fn update_score(index: usize, correct: bool) {}
}
