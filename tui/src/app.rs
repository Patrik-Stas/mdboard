use crate::model::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Board,
    Prompts,
    Documents,
    Activity,
}

impl View {
    pub const ALL: [View; 4] = [View::Board, View::Prompts, View::Documents, View::Activity];

    pub fn label(self) -> &'static str {
        match self {
            View::Board => "Board",
            View::Prompts => "Prompts",
            View::Documents => "Documents",
            View::Activity => "Activity",
        }
    }

    pub fn index(self) -> usize {
        match self {
            View::Board => 0,
            View::Prompts => 1,
            View::Documents => 2,
            View::Activity => 3,
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => View::Board,
            1 => View::Prompts,
            2 => View::Documents,
            3 => View::Activity,
            _ => View::Board,
        }
    }

    pub fn next(self) -> Self {
        Self::from_index((self.index() + 1) % Self::ALL.len())
    }

    pub fn prev(self) -> Self {
        Self::from_index((self.index() + Self::ALL.len() - 1) % Self::ALL.len())
    }
}

#[derive(Debug, Clone)]
pub enum Overlay {
    TaskDetail {
        task: Task,
        comments: Vec<Comment>,
        scroll: usize,
    },
    ResourceDetail {
        resource: Resource,
        revisions: Vec<Revision>,
        current_rev: Option<usize>, // None = current, Some(idx) = viewing revision
        scroll: usize,
        resource_type: ResourceType,
    },
    Help {
        scroll: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Prompt,
    Document,
}

impl ResourceType {
    pub fn label(self) -> &'static str {
        match self {
            ResourceType::Prompt => "Prompt",
            ResourceType::Document => "Document",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connected,
    Disconnected,
    Connecting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    TabBar,
    Content,
}

pub struct App {
    pub view: View,
    pub overlay: Option<Overlay>,
    pub should_quit: bool,
    pub focus: Focus,

    // Data
    pub version: Option<VersionInfo>,
    pub board: Option<Board>,
    pub config: Option<Config>,
    pub prompts: Vec<Resource>,
    pub documents: Vec<Resource>,
    pub activity: Vec<ActivityEntry>,

    // Navigation state
    pub board_col: usize,
    pub board_row: Vec<usize>, // per-column selected row
    pub prompt_index: usize,
    pub document_index: usize,
    pub activity_index: usize,

    // Connection
    pub connection: ConnectionState,
    pub last_poll: Option<std::time::Instant>,
    pub poll_hashes: Option<PollHashes>,

    // Loading
    pub loading: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            view: View::Board,
            overlay: None,
            should_quit: false,
            focus: Focus::Content,
            version: None,
            board: None,
            config: None,
            prompts: vec![],
            documents: vec![],
            activity: vec![],
            board_col: 0,
            board_row: vec![],
            prompt_index: 0,
            document_index: 0,
            activity_index: 0,
            connection: ConnectionState::Connecting,
            last_poll: None,
            poll_hashes: None,
            loading: true,
        }
    }

    pub fn column_count(&self) -> usize {
        self.board
            .as_ref()
            .map(|b| b.columns.len())
            .unwrap_or(0)
    }

    pub fn current_column_tasks(&self) -> &[Task] {
        self.board
            .as_ref()
            .and_then(|b| b.columns.get(self.board_col))
            .map(|c| c.tasks.as_slice())
            .unwrap_or(&[])
    }

    pub fn current_board_row(&self) -> usize {
        self.board_row
            .get(self.board_col)
            .copied()
            .unwrap_or(0)
    }

    pub fn set_board_row(&mut self, row: usize) {
        while self.board_row.len() <= self.board_col {
            self.board_row.push(0);
        }
        self.board_row[self.board_col] = row;
    }

    pub fn selected_task(&self) -> Option<&Task> {
        let tasks = self.current_column_tasks();
        let row = self.current_board_row();
        tasks.get(row)
    }

    pub fn ensure_board_row_vec(&mut self) {
        let ncols = self.column_count();
        if self.board_row.len() < ncols {
            self.board_row.resize(ncols, 0);
        }
    }

    /// Clamp all navigation indices to valid ranges.
    pub fn clamp_indices(&mut self) {
        let ncols = self.column_count();
        if ncols > 0 && self.board_col >= ncols {
            self.board_col = ncols - 1;
        }
        self.ensure_board_row_vec();
        if let Some(board) = &self.board {
            for (i, col) in board.columns.iter().enumerate() {
                if let Some(row) = self.board_row.get_mut(i) {
                    if !col.tasks.is_empty() && *row >= col.tasks.len() {
                        *row = col.tasks.len() - 1;
                    }
                }
            }
        }
        if !self.prompts.is_empty() && self.prompt_index >= self.prompts.len() {
            self.prompt_index = self.prompts.len() - 1;
        }
        if !self.documents.is_empty() && self.document_index >= self.documents.len() {
            self.document_index = self.documents.len() - 1;
        }
        if !self.activity.is_empty() && self.activity_index >= self.activity.len() {
            self.activity_index = self.activity.len() - 1;
        }
    }
}
