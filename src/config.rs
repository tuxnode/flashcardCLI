// CLI Config Code

pub struct Args {
    pub mode: String,
    pub file_path: String,
}

impl Args {
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = std::env::args().collect();

        match args.len() {
            1 => Err(String::from("Usage: flashcard <mode> filepath")),
            2 => Ok(Self {
                mode: String::from("random"),
                file_path: args[1].clone(),
            }),
            3 => {
                let mode = args[1].clone();
                if mode != "random" && mode != "ordered" {
                    return Err(format!("Invalid mode {}: random, ordered", mode));
                }
                Ok(Self {
                    mode: mode,
                    file_path: args[2].clone(),
                })
            }
            _ => Err(String::from("Too many arguments")),
        }
    }
}
