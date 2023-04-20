use super::Hook;
use anyhow::Result;
use async_trait::async_trait;
use r34_wrapper::Post;
use serde::Serialize;

#[derive(Serialize)]
struct WebhookPayload {
    pub content: String,
}

#[derive(Clone, Debug)]
pub struct Discord {
    webhook_url: String,
}

#[async_trait]
impl Hook for Discord {
    async fn send(&self, posts: &[Post]) -> Result<()> {
        for post in posts {
            self.send_one(post).await?;
        }
        Ok(())
    }
}

impl Discord {
    pub fn new(webhook_url: &str) -> Self {
        Self {
            webhook_url: webhook_url.into(),
        }
    }

    async fn send_one(&self, post: &Post) -> Result<()> {
        reqwest::Client::default()
            .post(&self.webhook_url)
            .json(&WebhookPayload {
                content: post.file_url.clone(),
            })
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
