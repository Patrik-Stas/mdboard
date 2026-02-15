use serde::Deserialize;
use std::collections::HashMap;

// /api/version
#[derive(Debug, Clone, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub project: String,
}

// /api/config
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub columns: Vec<ColumnDef>,
    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColumnDef {
    pub name: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub color: String,
}

// /api/board
#[derive(Debug, Clone, Deserialize)]
pub struct Board {
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Column {
    pub name: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub tasks: Vec<Task>,
}

// /api/task/{col}/{file}
#[derive(Debug, Clone, Deserialize)]
pub struct Task {
    pub filename: String,
    #[serde(default)]
    pub column: String,
    #[serde(default)]
    pub meta: TaskMeta,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TaskMeta {
    #[serde(default)]
    pub id: Option<serde_json::Value>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub assignee: String,
    #[serde(default)]
    pub scopes: ScopesOrString,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub due: String,
    #[serde(default)]
    pub branch: String,
    #[serde(default)]
    pub completed: String,
}

/// Scopes can be either a list of strings or a single string from YAML parsing.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(untagged)]
pub enum ScopesOrString {
    List(Vec<String>),
    Single(String),
    #[default]
    Empty,
}

impl ScopesOrString {
    pub fn as_vec(&self) -> Vec<&str> {
        match self {
            ScopesOrString::List(v) => v.iter().map(|s| s.as_str()).collect(),
            ScopesOrString::Single(s) if !s.is_empty() => vec![s.as_str()],
            _ => vec![],
        }
    }
}

// /api/comments/{id}
#[derive(Debug, Clone, Deserialize)]
pub struct Comment {
    pub filename: String,
    #[serde(default)]
    pub meta: CommentMeta,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CommentMeta {
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub created: String,
}

// /api/prompts, /api/documents
#[derive(Debug, Clone, Deserialize)]
pub struct Resource {
    pub dir_name: String,
    #[serde(default)]
    pub meta: ResourceMeta,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ResourceMeta {
    #[serde(default)]
    pub id: Option<serde_json::Value>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub updated: String,
    #[serde(default)]
    pub revision: Option<i64>,
    #[serde(default)]
    pub scopes: ScopesOrString,
}

// /api/{type}/{dir}/revisions
#[derive(Debug, Clone, Deserialize)]
pub struct Revision {
    pub filename: String,
    #[serde(default)]
    pub meta: RevisionMeta,
    #[serde(default)]
    pub body: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RevisionMeta {
    #[serde(default)]
    pub revision: Option<i64>,
    #[serde(default)]
    pub created: String,
}

// /api/activity
#[derive(Debug, Clone, Deserialize)]
pub struct ActivityEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub id: Option<serde_json::Value>,
    #[serde(default)]
    pub column: Option<String>,
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(default)]
    pub dir_name: Option<String>,
    #[serde(default)]
    pub mtime: f64,
    #[serde(default)]
    pub revision: Option<i64>,
}

// /api/poll
#[derive(Debug, Clone, Deserialize)]
pub struct PollHashes {
    pub board: String,
    pub prompts: String,
    pub documents: String,
}
