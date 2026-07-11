use std::io;
use std::time::Duration;

use crossterm::{
    event::{KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Terminal};

use flashcard::config::Args;
use flashcard::tui::app::{App, AppState};
use flashcard::tui::event::{AppEvent, EventHandler};
use flashcard::tui::ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse().unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });

    let deck = flashcard::CardDeck::new(&args.file_path).unwrap_or_else(|e| {
        eprintln!("Failed to load cards: {}", e);
        std::process::exit(1);
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(deck);
    let events = EventHandler::new(Duration::from_millis(250));

    let result = run_app(&mut terminal, app, &events);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: App,
    events: &EventHandler,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        match events.next()? {
            AppEvent::Tick => {}
            AppEvent::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.quit()
                    }
                    _ => {}
                }

                if app.state == AppState::Question && key.code == KeyCode::Enter {
                    app.show_answer();
                } else if app.state == AppState::Answer {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => app.mark_correct(),
                        KeyCode::Char('n') | KeyCode::Char('N') => app.mark_incorrect(),
                        _ => {}
                    }
                }

                if app.should_quit {
                    return Ok(());
                }
            }
        }
    }
}
