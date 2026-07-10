use std::env;
use std::fs;
use std::io;

use serde::{Deserialize, Serialize};

use crate::config::print_args;

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
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
      print_args();
    }
}

fn load_cards(path: &str) -> Result<Vec<FlashCard>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cards: Vec<FlashCard> = serde_json::from_str(&content)?;
    Ok(cards)
}
