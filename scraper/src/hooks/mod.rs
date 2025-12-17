use self::discord::Discord;
use crate::hooks::local::Local;
use anyhow::Result;
use async_trait::async_trait;
use r34_wrapper::Post;
use serde::Deserialize;
use std::ops::Deref;

pub mod discord;
pub mod local;

#[async_trait]
pub trait Hook {
    async fn send(&self, posts: &[Post]) -> Result<()>;
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HookImpl {
    Discord(Discord),
    Local(Local),
}

impl Deref for HookImpl {
    type Target = dyn Hook;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Discord(dc) => dc,
            Self::Local(local) => local,
        }
    }
}
