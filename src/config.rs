// CLI Config Code

use std::path::Path;

pub struct Args {
    pub mode: String,
    pub file_path: String,
}

impl Args {
    pub fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let args: Vec<String> = std::env::args().collect();

        match args.len() {
            1 => Err(String::from("Usage: flashcard <mode> <cards.json>").into()),
            2 => {
                let file_path = args[1].clone();
                if !Path::new(&file_path).exists() {
                    return Err(format!("Invalid path: {}", file_path).into());
                }
                Ok(Self {
                    mode: String::from("random"),
                    file_path,
                })
            }
            3 => {
                let mode = args[1].clone();
                let file_path = args[2].clone();
                if mode != "random" && mode != "ordered" {
                    return Err(format!("Invalid mode {}", mode).into());
                }
                if !Path::new(&file_path).exists() {
                    return Err(format!("Invalid path: {}", file_path).into());
                }
                Ok(Self {
                    mode,
                    file_path,
                })
            }
            _ => Err(String::from("Too many arguments").into()),
        }
    }
}
