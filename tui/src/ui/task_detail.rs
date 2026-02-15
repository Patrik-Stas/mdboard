use ratatui::Frame;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap};

use crate::app::{App, Overlay};
use crate::theme;
use crate::ui::board::{count_checkboxes, format_progress};
use crate::ui::common::centered_rect;
use crate::ui::markdown::markdown_to_lines;

pub fn render_task_detail(f: &mut Frame, app: &App) {
    let (task, comments, scroll) = match &app.overlay {
        Some(Overlay::TaskDetail {
            task,
            comments,
            scroll,
        }) => (task, comments, *scroll),
        _ => return,
    };

    let area = centered_rect(80, 85, f.area());
    f.render_widget(Clear, area);

    let title = if task.meta.title.is_empty() {
        &task.filename
    } else {
        &task.meta.title
    };

    // Build content lines
    let mut lines: Vec<Line<'static>> = Vec::new();

    // Title
    lines.push(Line::from(Span::styled(
        title.to_string(),
        Style::default()
            .fg(theme::TEXT_PRIMARY)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    // Metadata
    let mut meta_parts: Vec<Span<'static>> = Vec::new();

    if !task.meta.assignee.is_empty() {
        meta_parts.push(Span::styled(
            format!("@{}", task.meta.assignee),
            Style::default().fg(theme::TEXT_SECONDARY),
        ));
        meta_parts.push(Span::raw("  "));
    }

    if !task.column.is_empty() {
        meta_parts.push(Span::styled(
            format!("column:{}", task.column),
            Style::default().fg(theme::TEXT_SECONDARY),
        ));
        meta_parts.push(Span::raw("  "));
    }

    let scopes = task.meta.scopes.as_vec();
    if !scopes.is_empty() {
        for scope in &scopes {
            meta_parts.push(Span::styled(
                format!("[{scope}]"),
                Style::default().fg(theme::SCOPE_FG),
            ));
            meta_parts.push(Span::raw(" "));
        }
        meta_parts.push(Span::raw(" "));
    }

    if !task.meta.created.is_empty() {
        meta_parts.push(Span::styled(
            format!("created:{}", task.meta.created),
            Style::default().fg(theme::TEXT_DIM),
        ));
        meta_parts.push(Span::raw("  "));
    }

    if !task.meta.due.is_empty() {
        meta_parts.push(Span::styled(
            format!("due:{}", task.meta.due),
            Style::default().fg(theme::YELLOW),
        ));
        meta_parts.push(Span::raw("  "));
    }

    if !task.meta.completed.is_empty() {
        meta_parts.push(Span::styled(
            format!("completed:{}", task.meta.completed),
            Style::default().fg(theme::GREEN),
        ));
    }

    if !meta_parts.is_empty() {
        lines.push(Line::from(meta_parts));
    }

    // Progress bar
    let (checked, total) = count_checkboxes(&task.body);
    if total > 0 {
        lines.push(Line::from(Span::styled(
            format_progress(checked, total),
            Style::default().fg(if checked == total {
                theme::GREEN
            } else {
                theme::YELLOW
            }),
        )));
    }

    lines.push(Line::from(""));

    // Separator
    lines.push(Line::from(Span::styled(
        "─".repeat(60),
        Style::default().fg(theme::BORDER_COLOR),
    )));
    lines.push(Line::from(""));

    // Body
    let body_lines = markdown_to_lines(&task.body);
    lines.extend(body_lines);

    // Comments
    if !comments.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "─".repeat(60),
            Style::default().fg(theme::BORDER_COLOR),
        )));
        lines.push(Line::from(Span::styled(
            format!(" Comments ({})", comments.len()),
            Style::default()
                .fg(theme::TEXT_PRIMARY)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));

        for comment in comments {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("@{}", comment.meta.author),
                    Style::default()
                        .fg(theme::TAB_ACTIVE_FG)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  {}", comment.meta.created),
                    Style::default().fg(theme::TEXT_DIM),
                ),
            ]));
            let comment_lines = markdown_to_lines(&comment.body);
            lines.extend(comment_lines);
            lines.push(Line::from(""));
        }
    }

    let block = Block::default()
        .title(Line::from(Span::styled(
            format!(" {} ", task.filename),
            Style::default()
                .fg(theme::TEXT_SECONDARY)
                .add_modifier(Modifier::BOLD),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_HIGHLIGHT))
        .style(Style::default().bg(theme::OVERLAY_BG))
        .padding(Padding::new(2, 2, 1, 1));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll as u16, 0));

    f.render_widget(paragraph, area);
}
