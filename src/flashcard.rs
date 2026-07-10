use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FlashCard {
    pub question: String,
    pub answer: String,
    pub score: i32,
}
