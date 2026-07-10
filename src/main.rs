use std::io::{self, Write};

use flashcard::CardDeck;
use flashcard::config::Args;

fn main() {
    let args = Args::parse().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let mut deck = CardDeck::new(&args.file_path).unwrap_or_else(|e| {
        eprintln!("加载卡片失败: {}", e);
        std::process::exit(1);
    });

    loop {
        let card = deck.pick_random();
        let index = deck
            .cards
            .iter()
            .position(|c| std::ptr::eq(c, card))
            .unwrap();

        println!("\n? {}", card.question);
        print!("   （按回车查看答案）");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("\nA: {}", card.answer);
        print!("\n你记得这个答案吗？(y/n): ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let correct = input.trim().eq_ignore_ascii_case("y");
        deck.update_score(index, correct);

        if let Err(e) = deck.save() {
            eprintln!("保存失败: {}", e);
        }

        println!(
            "{}",
            if correct {
                "✓ 记住了"
            } else {
                "✗ 忘记了"
            }
        );
    }
}
