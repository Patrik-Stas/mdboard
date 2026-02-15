use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::theme;

/// Convert markdown text to a list of styled Lines for ratatui rendering.
/// Handles: headers, checkboxes, bold, italic, inline code, bullet lists.
pub fn markdown_to_lines(text: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    for raw_line in text.lines() {
        let trimmed = raw_line.trim();

        // Headers
        if trimmed.starts_with("### ") {
            lines.push(Line::from(Span::styled(
                trimmed[4..].to_string(),
                Style::default()
                    .fg(theme::TEXT_PRIMARY)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )));
            continue;
        }
        if trimmed.starts_with("## ") {
            lines.push(Line::from(Span::styled(
                trimmed[3..].to_string(),
                Style::default()
                    .fg(theme::TEXT_PRIMARY)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )));
            continue;
        }
        if trimmed.starts_with("# ") {
            lines.push(Line::from(Span::styled(
                trimmed[2..].to_string(),
                Style::default()
                    .fg(theme::TEXT_PRIMARY)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )));
            continue;
        }

        // Checkboxes
        if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            let rest = trimmed[5..].to_string();
            lines.push(Line::from(vec![
                Span::styled("  ✓ ", Style::default().fg(theme::GREEN)),
                Span::styled(
                    rest,
                    Style::default()
                        .fg(theme::TEXT_DIM)
                        .add_modifier(Modifier::CROSSED_OUT),
                ),
            ]));
            continue;
        }
        if trimmed.starts_with("- [ ]") {
            let rest = trimmed[5..].to_string();
            lines.push(Line::from(vec![
                Span::styled("  ○ ", Style::default().fg(theme::TEXT_DIM)),
                Span::styled(rest, Style::default().fg(theme::TEXT_PRIMARY)),
            ]));
            continue;
        }

        // Bullet lists
        if trimmed.starts_with("- ") {
            let rest = trimmed[2..].to_string();
            let spans = parse_inline_formatting(&format!("  • {rest}"));
            lines.push(Line::from(spans));
            continue;
        }

        // Horizontal rule
        if trimmed == "---" || trimmed == "***" || trimmed == "___" {
            lines.push(Line::from(Span::styled(
                "─".repeat(40),
                Style::default().fg(theme::BORDER_COLOR),
            )));
            continue;
        }

        // Empty line
        if trimmed.is_empty() {
            lines.push(Line::from(""));
            continue;
        }

        // Regular text with inline formatting
        let spans = parse_inline_formatting(raw_line);
        lines.push(Line::from(spans));
    }

    lines
}

/// Parse inline markdown formatting: **bold**, *italic*, `code`.
fn parse_inline_formatting(text: &str) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut remaining = text.to_string();

    while !remaining.is_empty() {
        // Bold **text**
        if let Some(start) = remaining.find("**") {
            if let Some(end) = remaining[start + 2..].find("**") {
                if start > 0 {
                    spans.push(Span::styled(
                        remaining[..start].to_string(),
                        Style::default().fg(theme::TEXT_PRIMARY),
                    ));
                }
                spans.push(Span::styled(
                    remaining[start + 2..start + 2 + end].to_string(),
                    Style::default()
                        .fg(theme::TEXT_PRIMARY)
                        .add_modifier(Modifier::BOLD),
                ));
                remaining = remaining[start + 2 + end + 2..].to_string();
                continue;
            }
        }

        // Inline code `code`
        if let Some(start) = remaining.find('`') {
            if let Some(end) = remaining[start + 1..].find('`') {
                if start > 0 {
                    spans.push(Span::styled(
                        remaining[..start].to_string(),
                        Style::default().fg(theme::TEXT_PRIMARY),
                    ));
                }
                spans.push(Span::styled(
                    remaining[start + 1..start + 1 + end].to_string(),
                    Style::default()
                        .fg(theme::YELLOW)
                        .bg(theme::SURFACE_1),
                ));
                remaining = remaining[start + 1 + end + 1..].to_string();
                continue;
            }
        }

        // *italic* (single asterisk, not inside bold)
        if let Some(start) = remaining.find('*') {
            if let Some(end) = remaining[start + 1..].find('*') {
                if start > 0 {
                    spans.push(Span::styled(
                        remaining[..start].to_string(),
                        Style::default().fg(theme::TEXT_PRIMARY),
                    ));
                }
                spans.push(Span::styled(
                    remaining[start + 1..start + 1 + end].to_string(),
                    Style::default()
                        .fg(theme::TEXT_PRIMARY)
                        .add_modifier(Modifier::ITALIC),
                ));
                remaining = remaining[start + 1 + end + 1..].to_string();
                continue;
            }
        }

        // No more formatting — emit remainder
        spans.push(Span::styled(
            remaining.clone(),
            Style::default().fg(theme::TEXT_PRIMARY),
        ));
        break;
    }

    if spans.is_empty() {
        spans.push(Span::styled(
            text.to_string(),
            Style::default().fg(theme::TEXT_PRIMARY),
        ));
    }

    spans
}
