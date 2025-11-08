use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::tui::app::{App, Tab, InputMode, NotificationLevel};

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();

    if size.height < 30 || size.width < 140 {
        draw_size_warning(f);
        return;
    }

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(20),
            Constraint::Length(1),
        ])
        .split(size);

    draw_titlebar(f, app, layout[0]);
    draw_main_content(f, app, layout[1]);
    draw_statusbar(f, app, layout[2]);
}

fn draw_titlebar(f: &mut Frame, app: &App, area: Rect) {
    let tabs = vec![
        ("💬 Chat", Tab::Chat),
        ("📝 Editor", Tab::Editor),
        ("⌨️ Terminal", Tab::Terminal),
        ("⚙️ Settings", Tab::Settings),
    ];

    let mut title_spans = Vec::new();

    for (label, tab) in tabs {
        let is_active = app.active_tab == tab;
        let style = if is_active {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .bold()
        } else {
            Style::default().fg(Color::DarkGray)
        };

        title_spans.push(Span::styled(format!(" {} ", label), style));
        title_spans.push(Span::raw("  "));
    }

    let title_line = Line::from(title_spans);
    let titlebar = Paragraph::new(title_line)
        .bg(Color::Black);

    f.render_widget(titlebar, area);
}

fn draw_main_content(f: &mut Frame, app: &App, area: Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(28), Constraint::Min(80)])
        .split(area);

    draw_sidebar(f, app, main_layout[0]);

    match app.active_tab {
        Tab::Chat => draw_chat_tab(f, app, main_layout[1]),
        Tab::Editor => draw_editor_tab(f, app, main_layout[1]),
        Tab::Terminal => draw_terminal_tab(f, app, main_layout[1]),
        Tab::Settings => draw_settings_tab(f, app, main_layout[1]),
    }

    if let Some(ref notif) = app.notification {
        draw_notification(f, notif);
    }

    if app.show_help {
        draw_help_modal(f, app);
    }
}

fn draw_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let sidebar_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(8),
        ])
        .split(area);

    draw_logo(f, sidebar_layout[0]);
    draw_file_tree(f, app, sidebar_layout[1]);
    draw_ollama_models(f, app, sidebar_layout[2]);
}

fn draw_logo(f: &mut Frame, area: Rect) {
    let logo = Paragraph::new(
        vec![
            Line::from(Span::styled("━━ CA ━━", Style::default().fg(Color::Cyan).bold())),
            Line::from(Span::styled(
                "CodeAgent",
                Style::default().fg(Color::Cyan).bold(),
            )),
        ]
    )
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::BOTTOM).border_style(
        Style::default().fg(Color::DarkGray)
    ));

    f.render_widget(logo, area);
}

fn draw_file_tree(f: &mut Frame, app: &App, area: Rect) {
    let mut items = vec!["📂 PROJECT FILES".to_string(), "".to_string()];

    for node in &app.file_tree {
        render_tree_node(&mut items, node, 0);
    }

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|s| ListItem::new(s.clone()).style(Style::default().fg(Color::White)))
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::RIGHT)
                .border_style(Style::default().fg(Color::DarkGray))
        );

    f.render_widget(list, area);
}

fn render_tree_node(items: &mut Vec<String>, node: &crate::tui::app::FileTreeNode, depth: usize) {
    let indent = "  ".repeat(depth);
    items.push(format!("{}{}", indent, node.name));

    if node.is_dir && node.expanded {
        for child in &node.children {
            render_tree_node(items, child, depth + 1);
        }
    }
}

fn draw_sidebar_stats(f: &mut Frame, area: Rect) {
    let stats = vec![
        Line::from(Span::styled("📊 STATS", Style::default().bold())),
        Line::from(""),
        Line::from("20 modules"),
        Line::from("35+ features"),
        Line::from("2,600+ lines"),
    ];

    let stats_widget = Paragraph::new(stats)
        .style(Style::default().fg(Color::Green))
        .block(
            Block::default()
                .borders(Borders::RIGHT | Borders::TOP)
                .border_style(Style::default().fg(Color::DarkGray))
        );

    f.render_widget(stats_widget, area);
}

fn draw_ollama_models(f: &mut Frame, app: &App, area: Rect) {
    let mut model_lines = vec![
        Line::from(Span::styled("🤖 OLLAMA", Style::default().bold())),
        Line::from(""),
    ];

    if app.available_models.is_empty() {
        model_lines.push(Line::from(Span::styled(
            "No models",
            Style::default().fg(Color::Red),
        )));
    } else {
        model_lines.push(Line::from(format!("Active: {}", app.ollama_model)));
        model_lines.push(Line::from(""));
        for model in &app.available_models {
            let style = if model == &app.ollama_model {
                Style::default().fg(Color::Green).bold()
            } else {
                Style::default().fg(Color::DarkGray)
            };
            model_lines.push(Line::from(Span::styled(format!("▪ {}", model), style)));
        }
    }

    let models_widget = Paragraph::new(model_lines)
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::RIGHT | Borders::TOP)
                .border_style(Style::default().fg(Color::DarkGray))
        );

    f.render_widget(models_widget, area);
}

fn draw_chat_tab(f: &mut Frame, app: &App, area: Rect) {
    let chat_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(4)])
        .split(area);

    draw_messages(f, app, chat_layout[0]);
    draw_input_box(f, app, chat_layout[1]);
}

fn draw_messages(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();

    for msg in &app.chat_messages {
        let author_style = if msg.author == "You" {
            Style::default().fg(Color::Cyan).bold()
        } else if msg.author == "CodeAgent" {
            Style::default().fg(Color::Green).bold()
        } else {
            Style::default().fg(Color::Yellow).bold()
        };

        lines.push(Line::from(vec![
            Span::styled(&msg.author, author_style),
            Span::raw(" "),
            Span::styled("●", Style::default().fg(Color::DarkGray)),
        ]));

        for content_line in msg.content.lines() {
            lines.push(Line::from(format!("  {}", content_line)));
        }
        lines.push(Line::from(""));
    }

    let messages = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" 💬 Messages ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(messages, area);
}

fn draw_input_box(f: &mut Frame, app: &App, area: Rect) {
    let spinner = app.get_spinner();
    let border_color = match app.input_mode {
        InputMode::Insert => Color::Green,
        InputMode::Command => Color::Yellow,
        InputMode::Search => Color::Magenta,
        InputMode::Normal => Color::Gray,
    };

    let input_text = if app.input.is_empty() {
        " press i to chat • ? for help ".to_string()
    } else {
        app.input.clone()
    };

    let input = Paragraph::new(input_text)
        .block(
            Block::default()
                .title(format!(" {} Input ", spinner))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
        )
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(input, area);

    if app.input_mode == InputMode::Insert {
        f.set_cursor(
            area.x + app.input.len().min(area.width as usize - 4) as u16 + 2,
            area.y + 1,
        );
    }
}

fn draw_editor_tab(f: &mut Frame, app: &App, area: Rect) {
    let editor_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(10)])
        .split(area);

    draw_editor_tabs(f, app, editor_layout[0]);
    draw_code_editor(f, app, editor_layout[1]);
}

fn draw_editor_tabs(f: &mut Frame, app: &App, area: Rect) {
    let mut tabs_spans = Vec::new();

    for (idx, tab) in app.editor_tabs.iter().enumerate() {
        let is_active = idx == app.active_editor_tab;
        let modified = if tab.modified { "●" } else { "" };

        let style = if is_active {
            Style::default().fg(Color::Black).bg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        tabs_spans.push(Span::styled(
            format!(" {} {} ", tab.name, modified),
            style,
        ));
        tabs_spans.push(Span::raw(" "));
    }

    let tabs_line = Line::from(tabs_spans);
    let tabs_widget = Paragraph::new(tabs_line);

    f.render_widget(tabs_widget, area);
}

fn draw_code_editor(f: &mut Frame, app: &App, area: Rect) {
    let editor = &app.editor_tabs[app.active_editor_tab];

    let mut lines = Vec::new();
    for (idx, code_line) in editor.content.iter().enumerate() {
        let line_num = Span::styled(
            format!("{:>3} ", idx + 1),
            Style::default().fg(Color::DarkGray),
        );
        let content = Span::raw(&code_line.content);
        lines.push(Line::from(vec![line_num, content]));
    }

    let code = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" 📝 Editor ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(code, area);
}

fn draw_terminal_tab(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();

    for terminal_line in &app.terminal_output {
        let line_style = match terminal_line.style {
            crate::tui::app::LineStyle::Error => Style::default().fg(Color::Red),
            crate::tui::app::LineStyle::Success => Style::default().fg(Color::Green),
            crate::tui::app::LineStyle::Warning => Style::default().fg(Color::Yellow),
            crate::tui::app::LineStyle::Normal => Style::default().fg(Color::White),
        };
        lines.push(Line::from(Span::styled(&terminal_line.content, line_style)));
    }

    let terminal = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" ⌨️ Terminal ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        );

    f.render_widget(terminal, area);
}

fn draw_settings_tab(f: &mut Frame, _app: &App, area: Rect) {
    let settings = vec![
        Line::from(Span::styled("⚙️ SETTINGS", Style::default().bold())),
        Line::from(""),
        Line::from("Theme: Dark"),
        Line::from("Font Size: 12"),
        Line::from("Language: Rust"),
        Line::from("Format On Save: true"),
        Line::from("Show Line Numbers: true"),
    ];

    let settings_widget = Paragraph::new(settings)
        .block(
            Block::default()
                .title(" ⚙️ Settings ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        );

    f.render_widget(settings_widget, area);
}

fn draw_statusbar(f: &mut Frame, app: &App, area: Rect) {
    let left = Span::raw(&app.status_bar.file_info);
    let center = Span::styled(&app.status_bar.mode, Style::default().bold());
    let right = Span::raw(format!(
        "{} | {} | {}",
        app.status_bar.position, app.status_bar.git_branch, app.status_bar.diagnostics
    ));

    let statusbar = Paragraph::new(Line::from(vec![left, center, right]))
        .style(Style::default().fg(Color::Black).bg(Color::DarkGray));

    f.render_widget(statusbar, area);
}

fn draw_notification(f: &mut Frame, notif: &crate::tui::app::Notification) {
    let color = match notif.level {
        NotificationLevel::Info => Color::Cyan,
        NotificationLevel::Warning => Color::Yellow,
        NotificationLevel::Error => Color::Red,
        NotificationLevel::Success => Color::Green,
    };

    let content = vec![
        Line::from(Span::styled(&notif.title, Style::default().fg(color).bold())),
        Line::from(notif.message.clone()),
    ];

    let width = notif.message.len().max(20) as u16 + 4;
    let height = 4u16;
    let x = (f.size().width.saturating_sub(width)) / 2;
    let y = 2;

    let popup_area = Rect::new(x, y, width, height);

    let popup = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color))
        )
        .style(Style::default().fg(Color::White).bg(Color::Black));

    f.render_widget(Clear, popup_area);
    f.render_widget(popup, popup_area);
}

fn draw_help_modal(f: &mut Frame, _app: &App) {
    let content = vec![
        Line::from(Span::styled("❓ HELP", Style::default().fg(Color::Yellow).bold())),
        Line::from(""),
        Line::from("i        - Insert mode"),
        Line::from(":        - Command palette"),
        Line::from("/        - Search"),
        Line::from("Tab      - Next tab"),
        Line::from("Shift+Tab - Prev tab"),
        Line::from("Ctrl+s   - Save"),
        Line::from("Ctrl+p   - Find file"),
        Line::from("q        - Quit"),
    ];

    let width = 40u16;
    let height = 12u16;
    let x = (f.size().width.saturating_sub(width)) / 2;
    let y = (f.size().height.saturating_sub(height)) / 2;

    let modal_area = Rect::new(x, y, width, height);

    let modal = Paragraph::new(content)
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow).bold())
        )
        .style(Style::default().fg(Color::White).bg(Color::Black));

    f.render_widget(Clear, modal_area);
    f.render_widget(modal, modal_area);
}

fn draw_size_warning(f: &mut Frame) {
    let block = Block::default()
        .title(" Size Warning ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Red));

    let content = Paragraph::new(vec![
        Line::from("Terminal too small!"),
        Line::from(""),
        Line::from("Required: 140x30"),
        Line::from(format!("Current: {}x{}", f.size().width, f.size().height)),
    ])
    .block(block)
    .alignment(Alignment::Center);

    f.render_widget(content, f.size());
}
