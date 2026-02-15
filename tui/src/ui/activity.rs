use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Padding};

use crate::app::{App, Focus};
use crate::model::ActivityEntry;
use crate::theme;

pub fn render_activity(f: &mut Frame, app: &App, area: Rect) {
    if app.activity.is_empty() {
        let msg = if app.loading {
            "Loading..."
        } else {
            "No activity"
        };
        let p = ratatui::widgets::Paragraph::new(msg)
            .style(Style::default().fg(theme::TEXT_DIM))
            .centered();
        f.render_widget(p, area);
        return;
    }

    let items: Vec<ListItem> = app
        .activity
        .iter()
        .enumerate()
        .map(|(i, entry)| make_activity_item(entry, i == app.activity_index && app.overlay.is_none() && app.focus == Focus::Content))
        .collect();

    let block = Block::default()
        .title(Line::from(Span::styled(
            format!(" Activity ({}) ", app.activity.len()),
            Style::default()
                .fg(theme::TEXT_PRIMARY)
                .add_modifier(Modifier::BOLD),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_COLOR))
        .padding(Padding::horizontal(1));

    let mut state = ListState::default().with_selected(Some(app.activity_index));

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().bg(theme::SURFACE_1));

    f.render_stateful_widget(list, area, &mut state);
}

fn make_activity_item(entry: &ActivityEntry, is_selected: bool) -> ListItem<'static> {
    let indicator = if is_selected { "▌" } else { " " };

    let type_color = match entry.entry_type.as_str() {
        "task" => theme::TAB_ACTIVE_FG,
        "prompt" => theme::GREEN,
        "document" => theme::YELLOW,
        _ => theme::TEXT_SECONDARY,
    };

    let type_label = match entry.entry_type.as_str() {
        "task" => "task",
        "prompt" => "prompt",
        "document" => "doc",
        other => other,
    };

    let mut spans = vec![
        Span::styled(
            indicator.to_string(),
            Style::default().fg(theme::TAB_ACTIVE_FG),
        ),
        Span::styled(
            format!(" {type_label:<8}"),
            Style::default().fg(type_color),
        ),
        Span::styled(
            entry.title.clone(),
            Style::default()
                .fg(theme::TEXT_PRIMARY)
                .add_modifier(if is_selected {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        ),
    ];

    // Column for tasks
    if let Some(col) = &entry.column {
        spans.push(Span::styled(
            format!("  [{col}]"),
            Style::default().fg(theme::TEXT_DIM),
        ));
    }

    // Revision for resources
    if let Some(rev) = entry.revision {
        spans.push(Span::styled(
            format!("  rev:{rev}"),
            Style::default().fg(theme::TEXT_DIM),
        ));
    }

    // Relative time
    let time_str = relative_time(entry.mtime);
    spans.push(Span::styled(
        format!("  {time_str}"),
        Style::default().fg(theme::TEXT_DIM),
    ));

    ListItem::new(Line::from(spans))
}

fn relative_time(mtime: f64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    let diff = (now - mtime).max(0.0) as u64;

    if diff < 60 {
        "just now".to_string()
    } else if diff < 3600 {
        let m = diff / 60;
        format!("{m}m ago")
    } else if diff < 86400 {
        let h = diff / 3600;
        format!("{h}h ago")
    } else {
        let d = diff / 86400;
        format!("{d}d ago")
    }
}
