# Flashcard CLI

Rust CLI flashcard app with spaced repetition scoring.

## Build & Run

```bash
cargo build              # debug build
cargo build --release    # optimized build
cargo run -- <cards.json>           # default: random mode
cargo run -- random <cards.json>    # explicit random mode
cargo run -- ordered <cards.json>   # ordered mode (not yet implemented in main loop)
```

## Tests

```bash
cargo test               # run all unit tests
cargo test -- --nocapture  # show println! output in tests
```

Tests require `cards.json` in working directory (used by `carddeck::tests`).

## Project Structure

```
src/
├── lib.rs           # public API re-exports
├── main.rs          # CLI entry point, TUI event loop
├── carddeck.rs      # CardDeck struct: new, save, pick_random, update_score
├── flashcard.rs     # FlashCard struct (question, answer, score)
├── config.rs        # CLI arg parsing (manual, no clap)
└── tui/
    ├── mod.rs       # module declarations
    ├── app.rs       # App state machine (Question/Answer/Finished)
    ├── event.rs     # crossterm event handler (key + tick)
    └── ui.rs        # ratatui rendering (header, card, footer)
```

## Key Details

- **Dependencies**: `serde` (derive), `serde_json`, `rand 0.8`, `ratatui 0.29`, `crossterm 0.28`
- **Score range**: clamped 0–10 (see `carddeck.rs:49`)
- **CLI args**: `<file>` (random mode) or `<mode> <file>` where mode = `random` | `ordered`
- **ordered mode**: parsed in config.rs but `pick_random()` always uses random selection regardless
- **Card lookup trick**: `app.rs:33-36` uses `std::ptr::eq` to find index of card returned by `pick_random`
- **save()**: overwrites original file with pretty-printed JSON on every card review
- **TUI**: uses alternate screen + raw mode; `Ctrl+C` or `q` to quit

## Conventions

- English UI text in TUI (user-facing messages)
- English code comments
- No external CLI framework (manual arg parsing in config.rs)
- Score incremented +1 on correct, -1 on incorrect
