use std::fs;

use config::Args;

mod carddeck;
mod config;
mod flashcard;

fn main() {
    let args = Args::parse().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
}
