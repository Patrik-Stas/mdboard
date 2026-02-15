use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph, Wrap};

use crate::app::{App, Overlay};
use crate::theme;

/// Create a centered overlay area.
pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(area);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

pub fn render_help(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 70, f.area());
    f.render_widget(Clear, area);

    let scroll = match &app.overlay {
        Some(Overlay::Help { scroll }) => *scroll,
        _ => 0,
    };

    let help_text = vec![
        Line::from(Span::styled(
            "Key Bindings",
            Style::default()
                .fg(theme::TEXT_PRIMARY)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )),
        Line::from(""),
        Line::from(Span::styled("Global", Style::default().fg(theme::TAB_ACTIVE_FG).add_modifier(Modifier::BOLD))),
        make_help_line("q / Ctrl+C", "Quit"),
        make_help_line("1-4", "Switch view"),
        make_help_line("Tab / Shift+Tab", "Cycle views"),
        make_help_line("r", "Force refresh"),
        make_help_line("?", "Toggle this help"),
        Line::from(""),
        Line::from(Span::styled("Navigation", Style::default().fg(theme::TAB_ACTIVE_FG).add_modifier(Modifier::BOLD))),
        make_help_line("↑ at top of list", "Focus tab bar"),
        make_help_line("←/→ in tab bar", "Switch views"),
        make_help_line("↓/Enter in tab bar", "Focus content"),
        Line::from(""),
        Line::from(Span::styled("Board View", Style::default().fg(theme::TAB_ACTIVE_FG).add_modifier(Modifier::BOLD))),
        make_help_line("h/l / ←/→", "Move between columns"),
        make_help_line("j/k / ↓/↑", "Move between tasks"),
        make_help_line("Space / Enter", "Open task detail"),
        make_help_line("g / G", "Jump to top/bottom"),
        Line::from(""),
        Line::from(Span::styled("List Views (Prompts/Documents/Activity)", Style::default().fg(theme::TAB_ACTIVE_FG).add_modifier(Modifier::BOLD))),
        make_help_line("j/k / ↓/↑", "Move between items"),
        make_help_line("Space / Enter", "Open detail"),
        make_help_line("g / G", "Jump to top/bottom"),
        Line::from(""),
        Line::from(Span::styled("Overlays", Style::default().fg(theme::TAB_ACTIVE_FG).add_modifier(Modifier::BOLD))),
        make_help_line("Esc", "Close overlay"),
        make_help_line("j/k / ↓/↑", "Scroll content"),
        make_help_line("Space / Ctrl+d", "Page down"),
        make_help_line("Ctrl+u", "Page up"),
        make_help_line("g / G", "Jump to top/bottom"),
        make_help_line("[ / ]", "Browse revisions (prompts/docs)"),
    ];

    let block = Block::default()
        .title(Line::from(Span::styled(
            " Help ",
            Style::default()
                .fg(theme::TEXT_PRIMARY)
                .add_modifier(Modifier::BOLD),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_HIGHLIGHT))
        .style(Style::default().bg(theme::OVERLAY_BG))
        .padding(Padding::new(2, 2, 1, 1));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll as u16, 0));

    f.render_widget(paragraph, area);
}

fn make_help_line(key: &str, desc: &str) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("  {key:.<24}"),
            Style::default().fg(theme::YELLOW),
        ),
        Span::styled(
            desc.to_string(),
            Style::default().fg(theme::TEXT_PRIMARY),
        ),
    ])
}
