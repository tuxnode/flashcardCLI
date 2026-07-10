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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let deck = CardDeck::new("cards.json").unwrap();
        assert!(!deck.cards.is_empty());
    }

    #[test]
    fn test_save() {
        let test_file = "test_save_unit.json";
        let mut deck = CardDeck::new("cards.json").unwrap();
        deck.file_path = test_file.to_string();
        deck.save().unwrap();
        let _ = std::fs::remove_file(test_file);
    }

    #[test]
    fn test_pick_random() {
        let deck = CardDeck::new("cards.json").unwrap();
        let card = deck.pick_random();
        assert!(!card.question.is_empty());
    }

    #[test]
    fn test_update_score() {
        let mut deck = CardDeck::new("cards.json").unwrap();
        let old_score = deck.cards[0].score;
        deck.update_score(0, true);
        assert_eq!(deck.cards[0].score, old_score + 1);
    }
}
