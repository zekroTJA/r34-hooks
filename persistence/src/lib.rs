mod postgrest;
pub use postgrest::*;

use anyhow::Result;
use async_trait::async_trait;
use std::ops::Deref;

#[async_trait]
pub trait Persistence {
    async fn set_last_id(&self, uid: &str, last_id: i64) -> Result<()>;
    async fn get_last_id(&self, uid: &str) -> Result<Option<i64>>;
}

#[derive(Debug)]
pub enum PersistenceImpl {
    Postgres(Postgres),
}

impl Deref for PersistenceImpl {
    type Target = dyn Persistence;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Postgres(pg) => pg,
        }
    }
}
