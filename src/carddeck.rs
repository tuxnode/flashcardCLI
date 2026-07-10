use std::fs::File;
use std::io::BufReader;

use serde_json::value::Serializer;

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
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string_pretty(&self.cards)?;
        std::fs::write(&self.file_path, json_data)?;
        Ok(())
    }

    // Return a random number
    pub fn pick_random(&self) -> &FlashCard {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.cards.len());
        // Return random FlashCard according to random usize
        &self.cards[index]
    }

    // Update cords scros
    pub fn update_score(&mut self, index: usize, correct: bool) {
        if index < self.cards.len() {
            if correct {
                self.cards[index].score += 1;
            } else {
                self.cards[index].score -= 1;
            }
            self.cards[index].score = self.cards[index].score.clamp(0, 10);
        }
    }
}
