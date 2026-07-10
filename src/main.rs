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

fn main() {
    let args = Args::parse().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    match load_cards(&args.file_path) {
        Ok(cards) => {},
        Err(e) => eprintln!("Failed to load: {}", e),
    } 
}

fn load_cards(path: &str) -> Result<Vec<FlashCard>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cards: Vec<FlashCard> = serde_json::from_str(&content)?;
    Ok(cards)
}
