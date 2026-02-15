mod api;
mod app;
#[allow(dead_code)]
mod model;
mod poll;
mod theme;
mod ui;

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio::sync::mpsc;

use crate::api::ApiClient;
use crate::app::{App, ConnectionState, Focus, Overlay, ResourceType, View};
use crate::poll::{PollMessage, spawn_poller};

#[derive(Parser)]
#[command(name = "mdboard-tui", about = "Terminal UI for mdboard")]
struct Cli {
    /// Server URL (e.g. http://localhost:10600)
    #[arg(long)]
    url: Option<String>,

    /// Data directory (for port.json discovery)
    #[arg(long, default_value = ".mdboard")]
    dir: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let base_url = match cli.url {
        Some(url) => url,
        None => discover_url(&cli.dir)?,
    };

    let api = ApiClient::new(&base_url);

    // Set up terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, api).await;

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn discover_url(dir: &str) -> Result<String> {
    let port_file = PathBuf::from(dir).join("port.json");
    let content = std::fs::read_to_string(&port_file)
        .with_context(|| format!("Cannot read {port_file:?} — is the mdboard server running?\nStart it with: cd server && uv run mdboard --dir ../.mdboard\nOr specify --url manually."))?;
    let info: serde_json::Value =
        serde_json::from_str(&content).context("Invalid port.json")?;
    let port = info["port"]
        .as_u64()
        .context("port.json missing 'port' field")?;
    Ok(format!("http://localhost:{port}"))
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    api: ApiClient,
) -> Result<()> {
    let mut app = App::new();

    // Start background poller
    let (tx, mut rx) = mpsc::unbounded_channel::<PollMessage>();
    spawn_poller(api.clone(), tx);

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        // Multiplex terminal events and poll messages
        tokio::select! {
            // Check for terminal events (with short timeout to stay responsive)
            _ = tokio::time::sleep(Duration::from_millis(50)) => {
                while event::poll(Duration::ZERO)? {
                    if let Event::Key(key) = event::read()? {
                        handle_key(&mut app, &api, key).await;
                    }
                }
            }
            // Process poll messages
            msg = rx.recv() => {
                if let Some(msg) = msg {
                    handle_poll_message(&mut app, msg);
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn handle_poll_message(app: &mut App, msg: PollMessage) {
    match msg {
        PollMessage::InitialData {
            version,
            board,
            config,
            prompts,
            documents,
            activity,
        } => {
            app.version = Some(version);
            app.board = Some(board);
            app.config = Some(config);
            app.prompts = prompts;
            app.documents = documents;
            app.activity = activity;
            app.connection = ConnectionState::Connected;
            app.loading = false;
            app.ensure_board_row_vec();
            app.clamp_indices();
        }
        PollMessage::HashesChanged(hashes) => {
            app.poll_hashes = Some(hashes);
            app.last_poll = Some(std::time::Instant::now());
        }
        PollMessage::BoardUpdated(board) => {
            app.board = Some(board);
            app.ensure_board_row_vec();
            app.clamp_indices();
        }
        PollMessage::PromptsUpdated(prompts) => {
            app.prompts = prompts;
            app.clamp_indices();
        }
        PollMessage::DocumentsUpdated(documents) => {
            app.documents = documents;
            app.clamp_indices();
        }
        PollMessage::ActivityUpdated(activity) => {
            app.activity = activity;
            app.clamp_indices();
        }
        PollMessage::ConnectionLost => {
            app.connection = ConnectionState::Disconnected;
        }
        PollMessage::ConnectionRestored => {
            app.connection = ConnectionState::Connected;
        }
        PollMessage::Error(_) => {
            // Errors are reflected via ConnectionLost
        }
    }
}

async fn handle_key(app: &mut App, api: &ApiClient, key: KeyEvent) {
    // Global: Ctrl+C always quits
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.should_quit = true;
        return;
    }

    // Overlay key handling
    if app.overlay.is_some() {
        handle_overlay_key(app, api, key).await;
        return;
    }

    // Global keys that work regardless of focus
    match key.code {
        KeyCode::Char('q') => {
            app.should_quit = true;
            return;
        }
        KeyCode::Char('1') => { app.view = View::Board; app.focus = Focus::Content; return; }
        KeyCode::Char('2') => { app.view = View::Prompts; app.focus = Focus::Content; return; }
        KeyCode::Char('3') => { app.view = View::Documents; app.focus = Focus::Content; return; }
        KeyCode::Char('4') => { app.view = View::Activity; app.focus = Focus::Content; return; }
        KeyCode::Char('?') => {
            app.overlay = Some(Overlay::Help { scroll: 0 });
            return;
        }
        KeyCode::Char('r') => {
            refresh_current_view(app, api).await;
            return;
        }
        _ => {}
    }

    // Tab bar focus mode
    if app.focus == Focus::TabBar {
        match key.code {
            KeyCode::Char('h') | KeyCode::Left | KeyCode::BackTab => {
                app.view = app.view.prev();
            }
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Tab => {
                app.view = app.view.next();
            }
            KeyCode::Char('j') | KeyCode::Down | KeyCode::Enter | KeyCode::Char(' ') => {
                app.focus = Focus::Content;
            }
            KeyCode::Esc => {
                app.focus = Focus::Content;
            }
            _ => {}
        }
        return;
    }

    // Tab/BackTab always cycle views from content too
    match key.code {
        KeyCode::Tab => { app.view = app.view.next(); return; }
        KeyCode::BackTab => { app.view = app.view.prev(); return; }
        _ => {}
    }

    // Content focus — view-specific keys
    match app.view {
        View::Board => handle_board_key(app, api, key).await,
        View::Prompts => handle_list_key(app, api, key, ResourceType::Prompt).await,
        View::Documents => handle_list_key(app, api, key, ResourceType::Document).await,
        View::Activity => handle_activity_key(app, api, key).await,
    }
}

async fn handle_board_key(app: &mut App, api: &ApiClient, key: KeyEvent) {
    let ncols = app.column_count();
    if ncols == 0 {
        return;
    }

    match key.code {
        KeyCode::Char('h') | KeyCode::Left => {
            if app.board_col > 0 {
                app.board_col -= 1;
            }
        }
        KeyCode::Char('l') | KeyCode::Right => {
            if app.board_col + 1 < ncols {
                app.board_col += 1;
            }
        }
        KeyCode::Char('j') | KeyCode::Down => {
            let tasks_len = app.current_column_tasks().len();
            if tasks_len > 0 {
                let row = app.current_board_row();
                if row + 1 < tasks_len {
                    app.set_board_row(row + 1);
                }
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let row = app.current_board_row();
            if row > 0 {
                app.set_board_row(row - 1);
            } else {
                app.focus = Focus::TabBar;
            }
        }
        KeyCode::Char('g') => {
            app.set_board_row(0);
        }
        KeyCode::Char('G') => {
            let tasks_len = app.current_column_tasks().len();
            if tasks_len > 0 {
                app.set_board_row(tasks_len - 1);
            }
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            if let Some(task) = app.selected_task().cloned() {
                // Fetch full task detail + comments
                let task_id = task
                    .meta
                    .id
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_default();
                let full_task = api
                    .get_task(&task.column, &task.filename)
                    .await
                    .unwrap_or(task.clone());
                let comments = if !task_id.is_empty() {
                    api.get_comments(&task_id).await.unwrap_or_default()
                } else {
                    vec![]
                };
                app.overlay = Some(Overlay::TaskDetail {
                    task: full_task,
                    comments,
                    scroll: 0,
                });
            }
        }
        _ => {}
    }
}

async fn handle_list_key(app: &mut App, api: &ApiClient, key: KeyEvent, rtype: ResourceType) {
    let (len, index) = match rtype {
        ResourceType::Prompt => (app.prompts.len(), &mut app.prompt_index),
        ResourceType::Document => (app.documents.len(), &mut app.document_index),
    };

    if len == 0 {
        // Empty list — up goes to tab bar
        if matches!(key.code, KeyCode::Char('k') | KeyCode::Up) {
            app.focus = Focus::TabBar;
        }
        return;
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            if *index + 1 < len {
                *index += 1;
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if *index > 0 {
                *index -= 1;
            } else {
                app.focus = Focus::TabBar;
            }
        }
        KeyCode::Char('g') => {
            *index = 0;
        }
        KeyCode::Char('G') => {
            *index = len - 1;
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            let (resources, idx) = match rtype {
                ResourceType::Prompt => (&app.prompts, app.prompt_index),
                ResourceType::Document => (&app.documents, app.document_index),
            };
            if let Some(res) = resources.get(idx).cloned() {
                let dir_name = res.dir_name.clone();
                // Fetch full resource + revisions
                let full_res = match rtype {
                    ResourceType::Prompt => api.get_prompt(&dir_name).await.unwrap_or(res),
                    ResourceType::Document => api.get_document(&dir_name).await.unwrap_or(res),
                };
                let revisions = match rtype {
                    ResourceType::Prompt => {
                        api.list_prompt_revisions(&dir_name).await.unwrap_or_default()
                    }
                    ResourceType::Document => {
                        api.list_document_revisions(&dir_name)
                            .await
                            .unwrap_or_default()
                    }
                };
                app.overlay = Some(Overlay::ResourceDetail {
                    resource: full_res,
                    revisions,
                    current_rev: None,
                    scroll: 0,
                    resource_type: rtype,
                });
            }
        }
        _ => {}
    }
}

async fn handle_activity_key(app: &mut App, api: &ApiClient, key: KeyEvent) {
    let len = app.activity.len();
    if len == 0 {
        if matches!(key.code, KeyCode::Char('k') | KeyCode::Up) {
            app.focus = Focus::TabBar;
        }
        return;
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            if app.activity_index + 1 < len {
                app.activity_index += 1;
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if app.activity_index > 0 {
                app.activity_index -= 1;
            } else {
                app.focus = Focus::TabBar;
            }
        }
        KeyCode::Char('g') => {
            app.activity_index = 0;
        }
        KeyCode::Char('G') => {
            app.activity_index = len - 1;
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            if let Some(entry) = app.activity.get(app.activity_index).cloned() {
                open_activity_entry(app, api, &entry).await;
            }
        }
        _ => {}
    }
}

async fn open_activity_entry(app: &mut App, api: &ApiClient, entry: &model::ActivityEntry) {
    match entry.entry_type.as_str() {
        "task" => {
            if let (Some(col), Some(filename)) = (&entry.column, &entry.filename) {
                let task_id = entry
                    .id
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_default();
                if let Ok(task) = api.get_task(col, filename).await {
                    let comments = if !task_id.is_empty() {
                        api.get_comments(&task_id).await.unwrap_or_default()
                    } else {
                        vec![]
                    };
                    app.overlay = Some(Overlay::TaskDetail {
                        task,
                        comments,
                        scroll: 0,
                    });
                }
            }
        }
        "prompt" => {
            if let Some(dir_name) = &entry.dir_name {
                if let Ok(resource) = api.get_prompt(dir_name).await {
                    let revisions = api
                        .list_prompt_revisions(dir_name)
                        .await
                        .unwrap_or_default();
                    app.overlay = Some(Overlay::ResourceDetail {
                        resource,
                        revisions,
                        current_rev: None,
                        scroll: 0,
                        resource_type: ResourceType::Prompt,
                    });
                }
            }
        }
        "document" => {
            if let Some(dir_name) = &entry.dir_name {
                if let Ok(resource) = api.get_document(dir_name).await {
                    let revisions = api
                        .list_document_revisions(dir_name)
                        .await
                        .unwrap_or_default();
                    app.overlay = Some(Overlay::ResourceDetail {
                        resource,
                        revisions,
                        current_rev: None,
                        scroll: 0,
                        resource_type: ResourceType::Document,
                    });
                }
            }
        }
        _ => {}
    }
}

async fn handle_overlay_key(app: &mut App, _api: &ApiClient, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.overlay = None;
        }
        KeyCode::Char('q') => {
            app.overlay = None;
        }
        KeyCode::Char('j') | KeyCode::Down => {
            scroll_overlay(app, 1);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            scroll_overlay(app, -1);
        }
        KeyCode::Char(' ') => {
            scroll_overlay(app, 15);
        }
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            scroll_overlay(app, 15);
        }
        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            scroll_overlay(app, -15);
        }
        KeyCode::Char('g') => {
            set_overlay_scroll(app, 0);
        }
        KeyCode::Char('G') => {
            scroll_overlay(app, 1000); // large number, effectively bottom
        }
        KeyCode::Char('[') => {
            navigate_revision(app, -1);
        }
        KeyCode::Char(']') => {
            navigate_revision(app, 1);
        }
        _ => {}
    }
}

fn scroll_overlay(app: &mut App, delta: i32) {
    match &mut app.overlay {
        Some(Overlay::TaskDetail { scroll, .. }) => {
            *scroll = (*scroll as i32 + delta).max(0) as usize;
        }
        Some(Overlay::ResourceDetail { scroll, .. }) => {
            *scroll = (*scroll as i32 + delta).max(0) as usize;
        }
        Some(Overlay::Help { scroll }) => {
            *scroll = (*scroll as i32 + delta).max(0) as usize;
        }
        None => {}
    }
}

fn set_overlay_scroll(app: &mut App, value: usize) {
    match &mut app.overlay {
        Some(Overlay::TaskDetail { scroll, .. }) => *scroll = value,
        Some(Overlay::ResourceDetail { scroll, .. }) => *scroll = value,
        Some(Overlay::Help { scroll }) => *scroll = value,
        None => {}
    }
}

fn navigate_revision(app: &mut App, delta: i32) {
    if let Some(Overlay::ResourceDetail {
        revisions,
        current_rev,
        scroll,
        ..
    }) = &mut app.overlay
    {
        if revisions.is_empty() {
            return;
        }
        let new_rev = match current_rev {
            None => {
                if delta < 0 {
                    // Go to latest revision
                    Some(revisions.len() - 1)
                } else {
                    return; // already at current
                }
            }
            Some(idx) => {
                let new_idx = *idx as i32 + delta;
                if new_idx < 0 || new_idx >= revisions.len() as i32 {
                    // Back to current
                    None
                } else {
                    Some(new_idx as usize)
                }
            }
        };
        *current_rev = new_rev;
        *scroll = 0;
    }
}

async fn refresh_current_view(app: &mut App, api: &ApiClient) {
    match app.view {
        View::Board => {
            if let Ok(board) = api.board().await {
                app.board = Some(board);
                app.ensure_board_row_vec();
                app.clamp_indices();
            }
        }
        View::Prompts => {
            if let Ok(prompts) = api.list_prompts().await {
                app.prompts = prompts;
                app.clamp_indices();
            }
        }
        View::Documents => {
            if let Ok(docs) = api.list_documents().await {
                app.documents = docs;
                app.clamp_indices();
            }
        }
        View::Activity => {
            if let Ok(activity) = api.activity().await {
                app.activity = activity;
                app.clamp_indices();
            }
        }
    }
}
