use std::fs;

use serde::{Deserialize, Serialize};

use config::Args;

mod carddeck;
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
}
