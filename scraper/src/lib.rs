pub mod env;
pub mod hooks;

use anyhow::Result;
use hooks::HookImpl;
use persistence::PersistenceImpl;
use r34_wrapper::{Client, Post};
use std::collections::HashMap;

pub type WatchMap = HashMap<String, (Vec<String>, HookImpl)>;

#[derive(Debug)]
pub struct Scraper {
    db: PersistenceImpl,
    watchers: WatchMap,
    limit: usize,
    default_tags: Option<Vec<String>>,
}

impl Scraper {
    pub fn new(
        db: PersistenceImpl,
        watch: WatchMap,
        limit: usize,
        default_tags: Option<Vec<String>>,
    ) -> Self {
        Self {
            db,
            watchers: watch,
            limit,
            default_tags,
        }
    }

    pub async fn run(&self) -> Result<()> {
        for (uid, (tags, hook)) in &self.watchers {
            let new = self.get_new(uid, tags).await?;
            if let Some(new) = new {
                hook.send(&new).await?;
            }
        }

        Ok(())
    }

    pub async fn get_new(&self, uid: &str, tags: &[String]) -> Result<Option<Vec<Post>>> {
        let client = Client::default();

        let Some(last_uid) = self.db.get_last_id(uid).await? else {
            self.set_latest_id(&client, uid, tags).await?;
            return Ok(None);
        };

        if client.get_post(last_uid as u64).await?.is_none() {
            self.set_latest_id(&client, uid, tags).await?;
            return Ok(None);
        }

        let mut page = 0;
        let mut new = vec![];

        loop {
            let default = self.default_tags.clone().unwrap_or_default();
            let tags = [tags, default.as_slice()].concat();
            let res = client.list_posts(&tags, Some(page), Some(10)).await?;
            if res.count == 0 {
                break;
            }

            let mut found = false;

            let mut posts: Vec<_> = res
                .posts
                .iter()
                .take_while(|p| {
                    if p.id == last_uid as u64 {
                        found = true;
                        false
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();

            new.append(&mut posts);

            if found || new.len() >= self.limit {
                break;
            }

            page += 1;
        }

        new.reverse();

        if let Some(last) = new.last() {
            self.db.set_last_id(uid, last.id as i64).await?;
        }

        Ok(Some(new))
    }

    async fn set_latest_id(&self, client: &Client, uid: &str, tags: &[String]) -> Result<()> {
        let res = client.list_posts(tags, Some(0), Some(1)).await?;
        let Some(post) = res.posts.first() else {
                return Ok(());
            };
        self.db.set_last_id(uid, post.id as i64).await?;
        Ok(())
    }
}
