// CLI Config Code

use std::{fmt::format, path::Path};

pub struct Args {
    pub mode: String,
    pub file_path: String,
}

impl Args {
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = std::env::args().collect();

        match args.len() {
            1 => Err(String::from("Usage: flashcard <mode> filepath")),
            2 => {
                let file_path = args[1].clone();
                if !Path::new(&file_path).exists() {
                    return Err(format!("Invalid path: {}", file_path));
                }
                Ok(Self {
                    mode: String::from("random"),
                    file_path: file_path,
                })
            }
            3 => {
                let mode = args[1].clone();
                let file_path = args[2].clone();
                if mode != "random" && mode != "ordered" {
                    return Err(format!("Invalid mode {}: random, ordered", mode));
                }
                if !Path::new(&file_path).exists() {
                    return Err(format!("Invalid path: {}", file_path));
                }
                Ok(Self {
                    mode: mode,
                    file_path: file_path,
                })
            }
            _ => Err(String::from("Too many arguments")),
        }
    }
}
