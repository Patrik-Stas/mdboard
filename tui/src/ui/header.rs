use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, ConnectionState, Focus, View};
use crate::theme;

pub fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let tab_focused = app.focus == Focus::TabBar && app.overlay.is_none();

    let mut spans = vec![Span::styled("  mdboard", Style::default().fg(theme::HEADER_FG).add_modifier(Modifier::BOLD))];
    spans.push(Span::raw("  "));

    for (i, view) in View::ALL.iter().enumerate() {
        let num = format!("{}", i + 1);
        let is_active = *view == app.view;

        if is_active && tab_focused {
            // Focused + active: highlighted background to show cursor is here
            spans.push(Span::styled(
                format!(" {num} {} ", view.label()),
                Style::default()
                    .fg(theme::HEADER_BG)
                    .bg(theme::TAB_ACTIVE_FG)
                    .add_modifier(Modifier::BOLD),
            ));
        } else if is_active {
            spans.push(Span::styled(
                format!(" {num} {}", view.label()),
                Style::default()
                    .fg(theme::TAB_ACTIVE_FG)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::raw(" "));
        } else {
            spans.push(Span::styled(
                format!(" {num} {} ", view.label()),
                Style::default().fg(theme::TAB_INACTIVE_FG),
            ));
        }
    }

    let border_color = if tab_focused {
        theme::BORDER_HIGHLIGHT
    } else {
        theme::BORDER_COLOR
    };

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(theme::HEADER_BG));

    let paragraph = Paragraph::new(Line::from(spans)).block(block);
    f.render_widget(paragraph, area);
}

pub fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let mut spans = vec![];

    // Connection indicator
    match app.connection {
        ConnectionState::Connected => {
            spans.push(Span::styled(" ● ", Style::default().fg(theme::GREEN)));
        }
        ConnectionState::Disconnected => {
            spans.push(Span::styled(" ● disconnected ", Style::default().fg(theme::RED)));
        }
        ConnectionState::Connecting => {
            spans.push(Span::styled(" ◌ connecting ", Style::default().fg(theme::YELLOW)));
        }
    }

    // Project name + version
    if let Some(ver) = &app.version {
        spans.push(Span::styled(
            format!("{} v{}", ver.project, ver.version),
            Style::default().fg(theme::TEXT_SECONDARY),
        ));
    }

    // Right side: URL + help hint
    let right_text = " ?=help  q=quit ";
    let left_len: usize = spans.iter().map(|s| s.width()).sum();
    let padding = area.width as usize - left_len.min(area.width as usize) - right_text.len().min(area.width as usize);
    spans.push(Span::raw(" ".repeat(padding)));
    spans.push(Span::styled(
        right_text,
        Style::default().fg(theme::TEXT_DIM),
    ));

    let paragraph = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(theme::SURFACE_1));
    f.render_widget(paragraph, area);
}
