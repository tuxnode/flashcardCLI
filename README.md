# Flashcard CLI

Rust TUI flashcard app with spaced repetition scoring.

## Quick Start

```bash
cargo build --release
cargo run -- cards.json
```

## Usage

```bash
cargo run -- <cards.json>           # random mode
cargo run -- random <cards.json>    # explicit random
cargo run -- ordered <cards.json>   # ordered mode
```

## Controls

| Key | Action |
|-----|--------|
| Enter | Show answer |
| y | Mark correct (+1) |
| n | Mark incorrect (-1) |
| q / Ctrl+C | Quit |

## Card Format

```json
[
  { "question": "What is ownership?", "answer": "A set of rules for managing memory", "score": 0 }
]
```

## License

MIT
