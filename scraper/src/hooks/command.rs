use crate::hooks::Hook;
use anyhow::Result;
use async_trait::async_trait;
use r34_wrapper::Post;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;

#[derive(Clone, Debug, Deserialize)]
pub struct Command {
    run: String,
    args: Option<Vec<String>>,
    env: Option<HashMap<String, String>>,
    #[serde(default)]
    once_per_post: bool,
}

#[async_trait]
impl Hook for Command {
    async fn send(&self, posts: &[Post]) -> Result<()> {
        if self.once_per_post {
            for post in posts {
                tracing::info!("Running command for post {}: {}", &post.id, self);
                self.run(post).await?;
            }
        } else {
            let ids: Vec<String> = posts.iter().map(|p| p.id.to_string()).collect();
            tracing::info!("Running command for posts [{}]: {}", ids.join(", "), self);
            self.run(posts).await?;
        }
        Ok(())
    }
}

impl Command {
    async fn run(&self, payload: impl Serialize) -> Result<()> {
        let payload_vec = serde_json::to_vec(&payload)?;

        let mut cmd = tokio::process::Command::new(&self.run);
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .env_clear();

        if let Some(args) = &self.args {
            cmd.args(args);
        }

        if let Some(env) = &self.env {
            cmd.envs(env);
        }

        let mut child = cmd.spawn()?;

        {
            let mut stdin = child
                .stdin
                .take()
                .ok_or_else(|| anyhow::anyhow!("could not take stdin stream from process"))?;

            stdin.write_all(&payload_vec).await?;
            stdin.flush().await?;
        }

        let _ = child.wait().await?;

        Ok(())
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.run)?;
        if let Some(args) = &self.args {
            args.iter()
                .map(|a| write!(f, " \"{a}\""))
                .collect::<fmt::Result>()?;
        }
        Ok(())
    }
}
