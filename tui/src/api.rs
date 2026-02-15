use anyhow::{Context, Result};
use reqwest::Client;

use crate::model::*;

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
    client: Client,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
        }
    }

    pub async fn version(&self) -> Result<VersionInfo> {
        let resp = self
            .client
            .get(format!("{}/api/version", self.base_url))
            .send()
            .await
            .context("Failed to connect to mdboard server")?;
        resp.json().await.context("Invalid version response")
    }

    pub async fn config(&self) -> Result<Config> {
        let resp = self
            .client
            .get(format!("{}/api/config", self.base_url))
            .send()
            .await?;
        resp.json().await.context("Invalid config response")
    }

    pub async fn board(&self) -> Result<Board> {
        let resp = self
            .client
            .get(format!("{}/api/board", self.base_url))
            .send()
            .await?;
        resp.json().await.context("Invalid board response")
    }

    pub async fn get_task(&self, column: &str, filename: &str) -> Result<Task> {
        let resp = self
            .client
            .get(format!("{}/api/task/{}/{}", self.base_url, column, filename))
            .send()
            .await?;
        resp.json().await.context("Invalid task response")
    }

    pub async fn get_comments(&self, task_id: &str) -> Result<Vec<Comment>> {
        let resp = self
            .client
            .get(format!("{}/api/comments/{}", self.base_url, task_id))
            .send()
            .await?;
        resp.json().await.context("Invalid comments response")
    }

    pub async fn list_prompts(&self) -> Result<Vec<Resource>> {
        let resp = self
            .client
            .get(format!("{}/api/prompts", self.base_url))
            .send()
            .await?;
        resp.json().await.context("Invalid prompts response")
    }

    pub async fn get_prompt(&self, dir_name: &str) -> Result<Resource> {
        let resp = self
            .client
            .get(format!("{}/api/prompts/{}", self.base_url, dir_name))
            .send()
            .await?;
        resp.json().await.context("Invalid prompt response")
    }

    pub async fn list_prompt_revisions(&self, dir_name: &str) -> Result<Vec<Revision>> {
        let resp = self
            .client
            .get(format!(
                "{}/api/prompts/{}/revisions",
                self.base_url, dir_name
            ))
            .send()
            .await?;
        resp.json().await.context("Invalid revisions response")
    }

    pub async fn list_documents(&self) -> Result<Vec<Resource>> {
        let resp = self
            .client
            .get(format!("{}/api/documents", self.base_url))
            .send()
            .await?;
        resp.json().await.context("Invalid documents response")
    }

    pub async fn get_document(&self, dir_name: &str) -> Result<Resource> {
        let resp = self
            .client
            .get(format!("{}/api/documents/{}", self.base_url, dir_name))
            .send()
            .await?;
        resp.json().await.context("Invalid document response")
    }

    pub async fn list_document_revisions(&self, dir_name: &str) -> Result<Vec<Revision>> {
        let resp = self
            .client
            .get(format!(
                "{}/api/documents/{}/revisions",
                self.base_url, dir_name
            ))
            .send()
            .await?;
        resp.json().await.context("Invalid revisions response")
    }

    pub async fn activity(&self) -> Result<Vec<ActivityEntry>> {
        let resp = self
            .client
            .get(format!("{}/api/activity", self.base_url))
            .send()
            .await?;
        resp.json().await.context("Invalid activity response")
    }

    pub fn events_url(&self) -> String {
        format!("{}/api/events", self.base_url)
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
