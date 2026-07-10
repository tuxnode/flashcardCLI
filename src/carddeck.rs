use crate::FlashCard;

pub struct CardDeck {
    pub cards: Vec<FlashCard>,
    pub file_path: String,
}

impl CardDeck {
    pub fn new(path: &str) {}
    pub fn save() {}
    pub fn pick_random() {}
    pub fn update_score(index: i32, correct: bool) {}
}
