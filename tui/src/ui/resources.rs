use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph, Wrap};

use crate::app::{App, Focus, Overlay, ResourceType};
use crate::model::Resource;
use crate::theme;
use crate::ui::common::centered_rect;
use crate::ui::markdown::markdown_to_lines;

pub fn render_list(f: &mut Frame, app: &App, area: Rect, rtype: ResourceType) {
    let (resources, selected) = match rtype {
        ResourceType::Prompt => (&app.prompts, app.prompt_index),
        ResourceType::Document => (&app.documents, app.document_index),
    };

    let type_label = match rtype {
        ResourceType::Prompt => "Prompts",
        ResourceType::Document => "Documents",
    };

    if resources.is_empty() {
        let msg = if app.loading {
            "Loading...".to_string()
        } else {
            format!("No {}", type_label.to_lowercase())
        };
        let p = Paragraph::new(msg)
            .style(Style::default().fg(theme::TEXT_DIM))
            .centered();
        f.render_widget(p, area);
        return;
    }

    let items: Vec<ListItem> = resources
        .iter()
        .enumerate()
        .map(|(i, res)| make_list_item(res, i == selected && app.overlay.is_none() && app.focus == Focus::Content))
        .collect();

    let block = Block::default()
        .title(Line::from(Span::styled(
            format!(" {type_label} ({}) ", resources.len()),
            Style::default()
                .fg(theme::TEXT_PRIMARY)
                .add_modifier(Modifier::BOLD),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_COLOR))
        .padding(Padding::horizontal(1));

    let mut state = ListState::default().with_selected(Some(selected));

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().bg(theme::SURFACE_1));

    f.render_stateful_widget(list, area, &mut state);
}

fn make_list_item(res: &Resource, is_selected: bool) -> ListItem<'static> {
    let title = if res.meta.title.is_empty() {
        &res.dir_name
    } else {
        &res.meta.title
    };

    let indicator = if is_selected { "▌ " } else { "  " };

    let mut spans = vec![
        Span::styled(
            indicator.to_string(),
            Style::default().fg(theme::TAB_ACTIVE_FG),
        ),
        Span::styled(
            title.to_string(),
            Style::default()
                .fg(theme::TEXT_PRIMARY)
                .add_modifier(if is_selected {
                    Modifier::BOLD
                } else {
                    Modifier::empty()
                }),
        ),
    ];

    // Revision
    if let Some(rev) = res.meta.revision {
        spans.push(Span::styled(
            format!("  rev:{rev}"),
            Style::default().fg(theme::TEXT_DIM),
        ));
    }

    // Updated/created date
    let date = if !res.meta.updated.is_empty() {
        &res.meta.updated
    } else {
        &res.meta.created
    };
    if !date.is_empty() {
        spans.push(Span::styled(
            format!("  {date}"),
            Style::default().fg(theme::TEXT_DIM),
        ));
    }

    // Scopes
    let scopes = res.meta.scopes.as_vec();
    for scope in scopes.iter().take(3) {
        spans.push(Span::styled(
            format!("  [{scope}]"),
            Style::default().fg(theme::SCOPE_FG),
        ));
    }

    ListItem::new(Line::from(spans))
}

pub fn render_detail(f: &mut Frame, app: &App) {
    let (resource, revisions, current_rev, scroll, rtype) = match &app.overlay {
        Some(Overlay::ResourceDetail {
            resource,
            revisions,
            current_rev,
            scroll,
            resource_type,
        }) => (resource, revisions, current_rev, *scroll, *resource_type),
        _ => return,
    };

    let area = centered_rect(80, 85, f.area());
    f.render_widget(Clear, area);

    let title = if resource.meta.title.is_empty() {
        &resource.dir_name
    } else {
        &resource.meta.title
    };

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
    let mut meta_spans: Vec<Span<'static>> = Vec::new();

    if let Some(rev) = resource.meta.revision {
        meta_spans.push(Span::styled(
            format!("rev:{rev}"),
            Style::default().fg(theme::TEXT_SECONDARY),
        ));
        meta_spans.push(Span::raw("  "));
    }

    if !resource.meta.created.is_empty() {
        meta_spans.push(Span::styled(
            format!("created:{}", resource.meta.created),
            Style::default().fg(theme::TEXT_DIM),
        ));
        meta_spans.push(Span::raw("  "));
    }
    if !resource.meta.updated.is_empty() {
        meta_spans.push(Span::styled(
            format!("updated:{}", resource.meta.updated),
            Style::default().fg(theme::TEXT_DIM),
        ));
        meta_spans.push(Span::raw("  "));
    }

    let scopes = resource.meta.scopes.as_vec();
    for scope in &scopes {
        meta_spans.push(Span::styled(
            format!("[{scope}]"),
            Style::default().fg(theme::SCOPE_FG),
        ));
        meta_spans.push(Span::raw(" "));
    }

    if !meta_spans.is_empty() {
        lines.push(Line::from(meta_spans));
    }

    // Revision navigation hint
    if !revisions.is_empty() {
        let rev_info = match current_rev {
            Some(idx) => {
                let rev = &revisions[*idx];
                format!(
                    "Viewing revision {} of {} ([ ] to navigate, current = latest)",
                    rev.meta.revision.unwrap_or(0),
                    revisions.len()
                )
            }
            None => format!(
                "Viewing current (latest) — {} revisions available ([ ] to browse)",
                revisions.len()
            ),
        };
        lines.push(Line::from(Span::styled(
            rev_info,
            Style::default().fg(theme::TEXT_DIM),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "─".repeat(60),
        Style::default().fg(theme::BORDER_COLOR),
    )));
    lines.push(Line::from(""));

    // Body — if viewing a revision, show that revision's body
    let body = match current_rev {
        Some(idx) => &revisions[*idx].body,
        None => &resource.body,
    };

    let body_lines = markdown_to_lines(body);
    lines.extend(body_lines);

    let block = Block::default()
        .title(Line::from(Span::styled(
            format!(" {} — {} ", rtype.label(), resource.dir_name),
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
