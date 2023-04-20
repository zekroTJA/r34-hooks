pub mod discord;

use std::ops::Deref;

use self::discord::Discord;
use anyhow::Result;
use async_trait::async_trait;
use r34_wrapper::Post;

#[async_trait]
pub trait Hook {
    async fn send(&self, posts: &[Post]) -> Result<()>;
}

#[derive(Clone, Debug)]
pub enum HookImpl {
    Discord(Discord),
}

impl Deref for HookImpl {
    type Target = dyn Hook;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Discord(dc) => dc,
        }
    }
}
