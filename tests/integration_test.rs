use flashcard::CardDeck;

#[test]
fn test_new() {
    let deck = CardDeck::new("cards.json").unwrap();
    assert!(!deck.cards.is_empty());
}

#[test]
fn test_new_invalid_path() {
    let result = CardDeck::new("nonexistent.json");
    assert!(result.is_err());
}

#[test]
fn test_save_and_reload() {
    let test_file = "test_save.json";
    let mut deck = CardDeck::new("cards.json").unwrap();
    deck.file_path = test_file.to_string();
    let original_count = deck.cards.len();
    deck.save().unwrap();

    let reloaded = CardDeck::new(test_file).unwrap();
    assert_eq!(original_count, reloaded.cards.len());

    let _ = std::fs::remove_file(test_file);
}

#[test]
fn test_pick_random() {
    let deck = CardDeck::new("cards.json").unwrap();
    let card = deck.pick_random();
    assert!(!card.question.is_empty());
    assert!(!card.answer.is_empty());
}

#[test]
fn test_update_score_correct() {
    let mut deck = CardDeck::new("cards.json").unwrap();
    let old_score = deck.cards[0].score;
    deck.update_score(0, true);
    assert_eq!(deck.cards[0].score, old_score + 1);
}

#[test]
fn test_update_score_incorrect() {
    let mut deck = CardDeck::new("cards.json").unwrap();
    deck.cards[0].score = 5;
    deck.update_score(0, false);
    assert_eq!(deck.cards[0].score, 4);
}

#[test]
fn test_score_clamp() {
    let mut deck = CardDeck::new("cards.json").unwrap();

    deck.cards[0].score = 10;
    deck.update_score(0, true);
    assert_eq!(deck.cards[0].score, 10);

    deck.cards[0].score = 0;
    deck.update_score(0, false);
    assert_eq!(deck.cards[0].score, 0);
}

#[test]
fn test_update_score_invalid_index() {
    let mut deck = CardDeck::new("cards.json").unwrap();
    let len = deck.cards.len();
    deck.update_score(len, true);
}

#[test]
fn test_flashcard_fields() {
    let deck = CardDeck::new("cards.json").unwrap();
    for card in &deck.cards {
        assert!(!card.question.is_empty());
        assert!(!card.answer.is_empty());
    }
}
