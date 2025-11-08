use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::tui::app::{App, InputMode};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(4),
        ])
        .split(f.size());

    draw_header(f, chunks[0]);
    draw_messages(f, app, chunks[1]);
    draw_input(f, app, chunks[2]);
}

fn draw_header(f: &mut Frame, area: Rect) {
    let header = Paragraph::new("╔════════════════════════════════════════════════════════════════════════╗\n║         CodeAgent v0.3.0 - Interactive AI Coding Assistant             ║\n╚════════════════════════════════════════════════════════════════════════╝")
        .style(Style::default().fg(Color::Cyan).bold());
    f.render_widget(header, area);
}

fn draw_messages(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 📝 Conversation ")
        .title_alignment(Alignment::Left)
        .border_style(Style::default().fg(Color::DarkGray));

    let messages_text = app
        .messages
        .iter()
        .map(|m| Line::from(m.clone()))
        .collect::<Vec<_>>();

    let paragraph = Paragraph::new(messages_text)
        .block(block)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, area);
}

fn draw_input(f: &mut Frame, app: &App, area: Rect) {
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(match app.input_mode {
            InputMode::Normal => " 🤖 Command (press 'i' to edit) ",
            InputMode::Editing => " ✏️ Editing ",
        })
        .title_alignment(Alignment::Left)
        .border_style(match app.input_mode {
            InputMode::Normal => Style::default().fg(Color::Gray),
            InputMode::Editing => Style::default().fg(Color::Green),
        });

    let input_text = Paragraph::new(app.input.clone())
        .block(input_block)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(input_text, area);

    if app.input_mode == InputMode::Editing {
        f.set_cursor(
            area.x + app.input.len() as u16 + 1,
            area.y + 1,
        );
    }
}
