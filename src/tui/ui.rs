use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::app::{App, AppState};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(4),
        ])
        .split(frame.area());

    draw_header(frame, app, chunks[0]);
    draw_card(frame, app, chunks[1]);
    draw_footer(frame, app, chunks[2]);
}

fn draw_header(frame: &mut Frame, app: &App, area: Rect) {
    let scores: Vec<Span> = app
        .deck
        .cards
        .iter()
        .enumerate()
        .map(|(_i, card)| {
            let color = match card.score {
                0..=3 => Color::Red,
                4..=6 => Color::Yellow,
                _ => Color::Green,
            };
            Span::styled(
                format!(" {} ", card.score),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )
        })
        .collect();

    let mut header_line = vec![Span::styled(
        " Scores: ",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )];
    header_line.extend(scores);

    let header = Paragraph::new(Line::from(header_line)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" Flashcard CLI "),
    );
    frame.render_widget(header, area);
}

fn draw_card(frame: &mut Frame, app: &App, area: Rect) {
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3)])
        .margin(1)
        .split(area);

    let title = match app.state {
        AppState::Question => "? Question",
        AppState::Answer => "A Answer",
        AppState::Finished => "Done",
    };

    let card_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .title(title)
        .style(Style::default().bg(Color::DarkGray));

    frame.render_widget(card_block, area);

    if app.state == AppState::Finished {
        let done = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled(
                "  All cards reviewed!",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  Press 'q' to quit",
                Style::default().fg(Color::Gray),
            )),
        ]);
        frame.render_widget(done, inner[0]);
        return;
    }

    let card = &app.deck.cards[app.current_card_index];

    let (label, content, color) = match app.state {
        AppState::Question => (
            "Q:",
            &card.question,
            Color::Yellow,
        ),
        AppState::Answer => (
            "A:",
            &card.answer,
            Color::Green,
        ),
        AppState::Finished => unreachable!(),
    };

    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("  {} ", label),
                Style::default()
                    .fg(color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                content.as_str(),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
    ]);
    frame.render_widget(content, inner[0]);
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let (left_text, _right_text) = match app.state {
        AppState::Question => (
            " [Enter] Show answer",
            "[q] Quit  ",
        ),
        AppState::Answer => (
            " [y] Remember  [n] Forget",
            "[q] Quit  ",
        ),
        AppState::Finished => (
            " [q] Quit",
            "",
        ),
    };

    let footer = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                left_text,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                format!(
                    " Cards: {}/{}  Score: {}",
                    app.current_card_index + 1,
                    app.deck.cards.len(),
                    app.deck.cards[app.current_card_index].score
                ),
                Style::default().fg(Color::Gray),
            ),
        ]),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Controls "),
    );
    frame.render_widget(footer, area);
}
