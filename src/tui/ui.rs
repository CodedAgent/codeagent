use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::tui::app::{App, Panel, InputMode};

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();

    if size.height < 24 || size.width < 120 {
        draw_minimal(f, app);
        return;
    }

    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(50)])
        .split(size);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(10),
        ])
        .split(main_layout[1]);

    draw_sidebar(f, app, main_layout[0]);
    draw_header(f, app, right_layout[0]);
    draw_chat_panel(f, app, right_layout[1]);
    draw_input_bar(f, app, right_layout[2]);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let title = format!(" ⚡ CodeAgent v0.3.0 | {} ", app.project_path);
    let block = Block::default()
        .borders(Borders::BOTTOM)
        .title(title)
        .title_alignment(Alignment::Left)
        .style(Style::default().fg(Color::Cyan));

    let header = Paragraph::new("")
        .block(block)
        .style(Style::default().bg(Color::Black));

    f.render_widget(header, area);
}

fn draw_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let sidebar_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(10),
            Constraint::Length(5),
        ])
        .split(area);

    draw_logo(f, sidebar_layout[0]);
    draw_file_explorer(f, app, sidebar_layout[1]);
    draw_stats(f, sidebar_layout[2]);
}

fn draw_logo(f: &mut Frame, area: Rect) {
    let logo = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  CA",
            Style::default().fg(Color::Cyan).bold(),
        )),
        Line::from("CodeAgent"),
        Line::from(""),
    ];

    let paragraph = Paragraph::new(logo)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));

    f.render_widget(paragraph, area);
}

fn draw_file_explorer(f: &mut Frame, app: &App, area: Rect) {
    let mut items = Vec::new();
    items.push(format!("  📁 Project Files"));
    items.push(String::new());

    for (idx, file) in app.file_tree.iter().enumerate() {
        let prefix = if idx == app.selected_file { "▶ " } else { "  " };
        let icon = if file.is_dir { "📁" } else { "📄" };
        let indent = "  ".repeat(file.depth);

        items.push(format!(
            "{}{}{}{}",
            prefix,
            indent,
            icon,
            file.name
        ));
    }

    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == 0 {
                Style::default().fg(Color::Yellow).bold()
            } else if i - 2 == app.selected_file {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(item.clone()).style(style)
        })
        .collect();

    let block = Block::default()
        .title(" 📂 Files ")
        .borders(Borders::RIGHT)
        .border_style(Style::default().fg(Color::DarkGray));

    let list = List::new(list_items).block(block);
    f.render_widget(list, area);
}

fn draw_stats(f: &mut Frame, area: Rect) {
    let stats = vec![
        Line::from("📊 Stats"),
        Line::from(""),
        Line::from("Modules: 20"),
        Line::from("Features: 35+"),
        Line::from("LOC: 2,600+"),
    ];

    let paragraph = Paragraph::new(stats)
        .style(Style::default().fg(Color::Green))
        .block(
            Block::default()
                .borders(Borders::RIGHT)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

    f.render_widget(paragraph, area);
}

fn draw_chat_panel(f: &mut Frame, app: &App, area: Rect) {
    let mut messages = Vec::new();

    for msg in &app.chat_messages {
        let author_style = if msg.author == "You" {
            Style::default().fg(Color::Cyan).bold()
        } else {
            Style::default().fg(Color::Green).bold()
        };

        messages.push(Line::from(vec![
            Span::styled(&msg.author, author_style),
            Span::raw(": "),
        ]));

        for line in msg.content.lines() {
            messages.push(Line::from(format!("  {}", line)));
        }
        messages.push(Line::from(""));
    }

    let block = Block::default()
        .title(format!(
            " {} Messages ",
            if app.input_mode == InputMode::Editing {
                "💬"
            } else {
                "📝"
            }
        ))
        .borders(Borders::ALL)
        .border_style(if app.input_mode == InputMode::Editing {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        });

    let paragraph = Paragraph::new(messages)
        .block(block)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, area);
}

fn draw_input_bar(f: &mut Frame, app: &App, area: Rect) {
    let input_text = if app.input.is_empty() {
        "Type your message (Tab to switch panels, ? for help)".to_string()
    } else {
        app.input.clone()
    };

    let spinner = if app.input_mode == InputMode::Editing {
        app.get_spinner()
    } else {
        "•"
    };

    let border_color = match app.input_mode {
        InputMode::Editing => Color::Green,
        InputMode::CommandPalette => Color::Yellow,
        InputMode::Normal => Color::Gray,
    };

    let block = Block::default()
        .title(format!(" {} Input ", spinner))
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let input_widget = Paragraph::new(input_text)
        .block(block)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(input_widget, area);

    if app.input_mode == InputMode::Editing {
        f.set_cursor(
            area.x + app.input.len().min(area.width as usize - 4) as u16 + 2,
            area.y + 1,
        );
    }
}

fn draw_minimal(f: &mut Frame, app: &App) {
    let block = Block::default()
        .title(" CodeAgent ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let mut lines = Vec::new();
    lines.push(Line::from("Terminal too small. Resize to at least 120x24."));
    lines.push(Line::from(""));
    lines.push(Line::from(format!("Current: {}x{}", f.size().width, f.size().height)));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .style(Style::default().fg(Color::White));

    f.render_widget(paragraph, f.size());
}
