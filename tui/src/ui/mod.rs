pub mod activity;
pub mod board;
pub mod common;
pub mod header;
pub mod markdown;
pub mod resources;
pub mod task_detail;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};

use crate::app::{App, Overlay, View};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3), // header/tabs
        Constraint::Min(0),   // main content
        Constraint::Length(1), // status bar
    ])
    .split(f.area());

    header::render_header(f, app, chunks[0]);

    match app.view {
        View::Board => board::render_board(f, app, chunks[1]),
        View::Prompts => resources::render_list(f, app, chunks[1], crate::app::ResourceType::Prompt),
        View::Documents => {
            resources::render_list(f, app, chunks[1], crate::app::ResourceType::Document)
        }
        View::Activity => activity::render_activity(f, app, chunks[1]),
    }

    header::render_status_bar(f, app, chunks[2]);

    // Render overlay on top
    if let Some(overlay) = &app.overlay {
        match overlay {
            Overlay::TaskDetail { .. } => task_detail::render_task_detail(f, app),
            Overlay::ResourceDetail { .. } => resources::render_detail(f, app),
            Overlay::Help { .. } => common::render_help(f, app),
        }
    }
}
