use crate::hooks::Hook;
use anyhow::Result;
use async_trait::async_trait;
use futures_util::StreamExt;
use r34_wrapper::Post;
use serde::Deserialize;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Clone, Deserialize, Debug)]
pub struct Local {
    directory: PathBuf,
    #[serde(default)]
    store_metadata: bool,
}

#[async_trait]
impl Hook for Local {
    async fn send(&self, posts: &[Post]) -> Result<()> {
        if tokio::fs::metadata(&self.directory).await.is_err() {
            tracing::info!("Creating output directory {:?}", &self.directory);
            tokio::fs::create_dir_all(&self.directory).await?;
        }

        for post in posts {
            self.store_post(post).await?;
        }

        Ok(())
    }
}

impl Local {
    async fn store_post(&self, post: &Post) -> Result<()> {
        let file_name = post.file_url.rsplit('/').next().ok_or_else(|| {
            anyhow::anyhow!("could not get file name from URL {}", &post.file_url)
        })?;

        let file_path = self.directory.join(file_name);

        tracing::info!("Downloading post {} to {:?}", post.id, &file_path);

        let mut stream = reqwest::get(&post.file_url)
            .await?
            .error_for_status()?
            .bytes_stream();

        let mut file = File::create(&file_path).await?;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }

        if self.store_metadata {
            let meta_file_name = self.directory.join(format!("{}.meta.json", file_name));
            tracing::info!("Storing post {} metadata to {:?}", post.id, &meta_file_name);
            let json_str = serde_json::to_string_pretty(post)?;
            tokio::fs::write(meta_file_name, json_str).await?;
        }

        Ok(())
    }
}
