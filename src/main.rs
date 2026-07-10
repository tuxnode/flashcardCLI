use std::fs;
use std::io;

use serde::{Deserialize, Serialize};

use config::Args;

mod config;

#[derive(Serialize, Deserialize, Debug)]
pub struct FlashCard {
    pub question: String,
    pub answer: String,
    pub score: i32,
}

pub struct CardDeck {
    pub cards: Vec<FlashCard>,
    pub file_path: String,
}

fn main() {}

fn load_cards(path: &str) -> Result<Vec<FlashCard>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cards: Vec<FlashCard> = serde_json::from_str(&content)?;
    Ok(cards)
}
