use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use crate::app::{App, Focus};
use crate::model::Task;
use crate::theme;

pub fn render_board(f: &mut Frame, app: &App, area: Rect) {
    let board = match &app.board {
        Some(b) => b,
        None => {
            let msg = if app.loading {
                "Loading..."
            } else {
                "No board data"
            };
            let p = Paragraph::new(msg)
                .style(Style::default().fg(theme::TEXT_DIM))
                .centered();
            f.render_widget(p, area);
            return;
        }
    };

    if board.columns.is_empty() {
        let p = Paragraph::new("No columns configured")
            .style(Style::default().fg(theme::TEXT_DIM))
            .centered();
        f.render_widget(p, area);
        return;
    }

    // Split area into equal columns
    let constraints: Vec<Constraint> = board
        .columns
        .iter()
        .map(|_| Constraint::Ratio(1, board.columns.len() as u32))
        .collect();

    let col_areas = Layout::horizontal(constraints).split(area);

    for (i, col) in board.columns.iter().enumerate() {
        let is_selected = i == app.board_col && app.overlay.is_none() && app.focus == Focus::Content;
        let col_color = theme::hex_to_color(&col.color);

        let border_style = if is_selected {
            Style::default().fg(col_color)
        } else {
            Style::default().fg(theme::BORDER_COLOR)
        };

        let label = if col.label.is_empty() {
            &col.name
        } else {
            &col.label
        };
        let title_line = Line::from(vec![
            Span::styled(
                format!(" {label} "),
                Style::default()
                    .fg(col_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{}", col.tasks.len()),
                Style::default().fg(theme::TEXT_DIM),
            ),
        ]);

        let block = Block::default()
            .title(title_line)
            .borders(Borders::ALL)
            .border_style(border_style)
            .padding(Padding::horizontal(1));

        let inner = block.inner(col_areas[i]);
        f.render_widget(block, col_areas[i]);

        if col.tasks.is_empty() {
            let empty = Paragraph::new("No tasks")
                .style(Style::default().fg(theme::TEXT_DIM));
            f.render_widget(empty, inner);
            continue;
        }

        let selected_row = app.board_row.get(i).copied().unwrap_or(0);

        // Render task cards
        render_task_list(f, &col.tasks, selected_row, is_selected, inner);
    }
}

fn render_task_list(
    f: &mut Frame,
    tasks: &[Task],
    selected: usize,
    col_is_active: bool,
    area: Rect,
) {
    // Each card takes 3 lines (title, meta, separator)
    let card_height = 3u16;
    let visible_cards = (area.height / card_height).max(1) as usize;

    // Scroll offset to keep selected visible
    let offset = if selected >= visible_cards {
        selected - visible_cards + 1
    } else {
        0
    };

    let mut y = area.y;
    for (i, task) in tasks.iter().enumerate().skip(offset) {
        if y + card_height > area.y + area.height {
            break;
        }

        let is_selected = i == selected && col_is_active;
        render_task_card(f, task, is_selected, Rect::new(area.x, y, area.width, card_height));
        y += card_height;
    }
}

fn render_task_card(f: &mut Frame, task: &Task, is_selected: bool, area: Rect) {
    if area.height < 2 {
        return;
    }

    let title = if task.meta.title.is_empty() {
        &task.filename
    } else {
        &task.meta.title
    };

    let indicator = if is_selected { "▌" } else { " " };

    // Line 1: indicator + title
    let title_style = if is_selected {
        Style::default()
            .fg(theme::TEXT_PRIMARY)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::TEXT_PRIMARY)
    };

    let title_line = Line::from(vec![
        Span::styled(indicator, Style::default().fg(theme::TAB_ACTIVE_FG)),
        Span::styled(truncate(title, area.width.saturating_sub(2) as usize), title_style),
    ]);
    f.render_widget(
        Paragraph::new(title_line),
        Rect::new(area.x, area.y, area.width, 1),
    );

    // Line 2: metadata (assignee, scopes, progress, due)
    if area.height >= 2 {
        let mut meta_spans = vec![Span::raw(" ")];

        // Assignee
        if !task.meta.assignee.is_empty() {
            meta_spans.push(Span::styled(
                format!("@{}", task.meta.assignee),
                Style::default().fg(theme::TEXT_SECONDARY),
            ));
            meta_spans.push(Span::raw(" "));
        }

        // Scopes
        let scopes = task.meta.scopes.as_vec();
        for scope in scopes.iter().take(2) {
            meta_spans.push(Span::styled(
                format!("[{scope}]"),
                Style::default().fg(theme::SCOPE_FG),
            ));
            meta_spans.push(Span::raw(" "));
        }

        // Checkbox progress
        let (checked, total) = count_checkboxes(&task.body);
        if total > 0 {
            meta_spans.push(Span::styled(
                format_progress(checked, total),
                Style::default().fg(if checked == total {
                    theme::GREEN
                } else {
                    theme::YELLOW
                }),
            ));
            meta_spans.push(Span::raw(" "));
        }

        // Due date
        if !task.meta.due.is_empty() {
            meta_spans.push(Span::styled(
                format!("due:{}", task.meta.due),
                Style::default().fg(theme::TEXT_DIM),
            ));
        }

        f.render_widget(
            Paragraph::new(Line::from(meta_spans)),
            Rect::new(area.x, area.y + 1, area.width, 1),
        );
    }

    // Line 3: separator
    if area.height >= 3 {
        let sep = "─".repeat(area.width as usize);
        f.render_widget(
            Paragraph::new(Span::styled(sep, Style::default().fg(theme::BORDER_COLOR))),
            Rect::new(area.x, area.y + 2, area.width, 1),
        );
    }
}

pub fn count_checkboxes(body: &str) -> (usize, usize) {
    let mut checked = 0;
    let mut total = 0;
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            checked += 1;
            total += 1;
        } else if trimmed.starts_with("- [ ]") {
            total += 1;
        }
    }
    (checked, total)
}

pub fn format_progress(checked: usize, total: usize) -> String {
    if total == 0 {
        return String::new();
    }
    let bar_width = 8;
    let filled = if total > 0 {
        (checked * bar_width) / total
    } else {
        0
    };
    let empty = bar_width - filled;
    format!(
        "[{}{}] {}/{}",
        "#".repeat(filled),
        "-".repeat(empty),
        checked,
        total
    )
}

fn truncate(s: &str, max_width: usize) -> String {
    if s.len() <= max_width {
        s.to_string()
    } else if max_width > 3 {
        format!("{}...", &s[..max_width - 3])
    } else {
        s[..max_width].to_string()
    }
}
