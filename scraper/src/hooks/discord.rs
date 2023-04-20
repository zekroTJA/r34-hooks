use std::path::Path;

use super::Hook;
use anyhow::Result;
use async_trait::async_trait;
use r34_wrapper::Post;
use serde::Serialize;

#[derive(Serialize, Default)]
struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

#[derive(Serialize, Default)]
struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
}

#[derive(Serialize, Default)]
struct EmbedAsset {
    pub url: String,
    pub height: Option<u64>,
    pub width: Option<u64>,
}

#[derive(Serialize, Default)]
struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub color: Option<u64>,
    pub fields: Option<Vec<EmbedField>>,
    pub image: Option<EmbedAsset>,
    pub video: Option<EmbedAsset>,
    pub footer: Option<EmbedFooter>,
}

#[derive(Serialize, Default)]
struct WebhookPayload {
    pub embeds: Vec<Embed>,
}

impl From<&Post> for WebhookPayload {
    fn from(post: &Post) -> Self {
        let mut emb = Embed::default();
        let mut fields = vec![];

        if !&post.source.is_empty() {
            fields.push(EmbedField {
                name: "Source".into(),
                value: post.source.clone(),
                ..Default::default()
            });
        }

        let asset = EmbedAsset {
            url: post.file_url.clone(),
            height: Some(post.height),
            width: Some(post.width),
        };

        if is_video_url(&post.file_url) {
            emb.video = Some(asset);
        } else {
            emb.image = Some(asset);
        }

        emb.title = Some("Post".into());
        emb.url = Some(format!(
            "https://rule34.xxx/index.php?page=post&s=view&id={}",
            &post.id
        ));

        emb.footer = Some(EmbedFooter {
            text: post.id.to_string(),
            ..Default::default()
        });

        emb.color = Some(0xaae5a4);

        emb.fields = Some(fields);

        Self { embeds: vec![emb] }
    }
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
            .json(&WebhookPayload::from(post))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

fn is_video_url(url: &str) -> bool {
    let Some(extension) = Path::new(url).extension().and_then(|e| e.to_str()) else {
        return false;
    };

    matches!(
        extension.to_lowercase().as_str(),
        "webm"
            | "flv"
            | "ogv"
            | "ogg"
            | "avi"
            | "mov"
            | "wmv"
            | "mpg"
            | "mp2"
            | "mpv"
            | "m4v"
            | "mp4"
    )
}
