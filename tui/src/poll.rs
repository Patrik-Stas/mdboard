use std::time::Duration;

use futures::StreamExt;
use tokio::sync::mpsc;

use crate::api::ApiClient;
use crate::model::PollHashes;

#[derive(Debug)]
pub enum PollMessage {
    InitialData {
        version: crate::model::VersionInfo,
        board: crate::model::Board,
        config: crate::model::Config,
        prompts: Vec<crate::model::Resource>,
        documents: Vec<crate::model::Resource>,
        activity: Vec<crate::model::ActivityEntry>,
    },
    HashesChanged(PollHashes),
    BoardUpdated(crate::model::Board),
    PromptsUpdated(Vec<crate::model::Resource>),
    DocumentsUpdated(Vec<crate::model::Resource>),
    ActivityUpdated(Vec<crate::model::ActivityEntry>),
    ConnectionLost,
    ConnectionRestored,
    #[allow(dead_code)]
    Error(String),
}

pub fn spawn_poller(api: ApiClient, tx: mpsc::UnboundedSender<PollMessage>) {
    tokio::spawn(async move {
        // Initial data fetch
        match fetch_all(&api).await {
            Ok(msg) => {
                let _ = tx.send(msg);
            }
            Err(e) => {
                let _ = tx.send(PollMessage::Error(format!("Initial fetch failed: {e}")));
                let _ = tx.send(PollMessage::ConnectionLost);
            }
        }

        let mut was_connected = true;

        loop {
            match connect_sse(&api, &tx, &mut was_connected).await {
                Ok(()) => {
                    // Stream ended cleanly (server closed connection)
                }
                Err(_) => {
                    // Connection failed or broke
                }
            }

            if was_connected {
                was_connected = false;
                let _ = tx.send(PollMessage::ConnectionLost);
            }

            // Back off before reconnecting
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });
}

/// Connect to SSE stream and process events until disconnect.
async fn connect_sse(
    api: &ApiClient,
    tx: &mpsc::UnboundedSender<PollMessage>,
    was_connected: &mut bool,
) -> anyhow::Result<()> {
    let resp = api
        .client()
        .get(api.events_url())
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("SSE endpoint returned {}", resp.status());
    }

    if !*was_connected {
        *was_connected = true;
        let _ = tx.send(PollMessage::ConnectionRestored);
        // Full refresh on reconnect
        if let Ok(msg) = fetch_all(api).await {
            let _ = tx.send(msg);
        }
    }

    let mut stream = resp.bytes_stream();
    let mut buf = String::new();
    let mut last_hashes: Option<PollHashes> = None;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        buf.push_str(&String::from_utf8_lossy(&chunk));

        // SSE messages are terminated by a blank line (\n\n)
        while let Some(boundary) = buf.find("\n\n") {
            let message = buf[..boundary].to_string();
            buf = buf[boundary + 2..].to_string();

            if let Some(hashes) = parse_sse_message(&message) {
                // On hash change, selectively re-fetch changed data
                if let Some(prev) = &last_hashes {
                    let mut changed = false;
                    if prev.board != hashes.board {
                        changed = true;
                        if let Ok(board) = api.board().await {
                            let _ = tx.send(PollMessage::BoardUpdated(board));
                        }
                    }
                    if prev.prompts != hashes.prompts {
                        changed = true;
                        if let Ok(prompts) = api.list_prompts().await {
                            let _ = tx.send(PollMessage::PromptsUpdated(prompts));
                        }
                    }
                    if prev.documents != hashes.documents {
                        changed = true;
                        if let Ok(docs) = api.list_documents().await {
                            let _ = tx.send(PollMessage::DocumentsUpdated(docs));
                        }
                    }
                    if changed {
                        if let Ok(activity) = api.activity().await {
                            let _ = tx.send(PollMessage::ActivityUpdated(activity));
                        }
                        let _ = tx.send(PollMessage::HashesChanged(hashes.clone()));
                    }
                }
                last_hashes = Some(hashes);
            }
            // else: heartbeat comment or unparseable — ignore
        }
    }

    Ok(())
}

/// Parse an SSE message block. Returns hashes for both `init` and `changed` events.
fn parse_sse_message(message: &str) -> Option<PollHashes> {
    let mut event_type = None;
    let mut data = None;

    for line in message.lines() {
        if let Some(rest) = line.strip_prefix("event: ") {
            event_type = Some(rest.trim());
        } else if let Some(rest) = line.strip_prefix("data: ") {
            data = Some(rest.trim());
        }
        // Lines starting with ":" are comments (heartbeat) — skip
    }

    match (event_type, data) {
        (Some("init" | "changed"), Some(json_str)) => {
            serde_json::from_str::<PollHashes>(json_str).ok()
        }
        _ => None,
    }
}

async fn fetch_all(api: &ApiClient) -> anyhow::Result<PollMessage> {
    let (version, board, config, prompts, documents, activity) = tokio::try_join!(
        api.version(),
        api.board(),
        api.config(),
        api.list_prompts(),
        api.list_documents(),
        api.activity(),
    )?;
    Ok(PollMessage::InitialData {
        version,
        board,
        config,
        prompts,
        documents,
        activity,
    })
}
