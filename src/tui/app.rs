use crate::carddeck::CardDeck;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Question,
    Answer,
    Finished,
}

pub struct App {
    pub deck: CardDeck,
    pub state: AppState,
    pub current_card_index: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new(deck: CardDeck) -> Self {
        let mut app = Self {
            deck,
            state: AppState::Question,
            current_card_index: 0,
            should_quit: false,
        };
        app.pick_next_card();
        app
    }

    pub fn pick_next_card(&mut self) {
        if self.deck.cards.is_empty() {
            self.state = AppState::Finished;
            return;
        }
        let card = self.deck.pick_random();
        self.current_card_index = self
            .deck
            .cards
            .iter()
            .position(|c| std::ptr::eq(c, card))
            .unwrap_or(0);
        self.state = AppState::Question;
    }

    pub fn show_answer(&mut self) {
        self.state = AppState::Answer;
    }

    pub fn mark_correct(&mut self) {
        self.deck.update_score(self.current_card_index, true);
        self.save();
        self.pick_next_card();
    }

    pub fn mark_incorrect(&mut self) {
        self.deck.update_score(self.current_card_index, false);
        self.save();
        self.pick_next_card();
    }

    fn save(&self) {
        if let Err(e) = self.deck.save() {
            eprintln!("Save failed: {}", e);
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
