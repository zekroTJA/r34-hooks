use crate::Persistence;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{postgres, query, Pool, Row};

#[derive(Debug)]
pub struct Postgres {
    pool: Pool<sqlx::Postgres>,
}

impl Postgres {
    pub async fn new(conn_string: &str) -> Result<Self> {
        let pool = postgres::PgPool::connect(conn_string).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl Persistence for Postgres {
    async fn set_last_id(&self, uid: &str, last_id: i64) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let ok = query("SELECT (last_uid) FROM last_uids WHERE uid = $1")
            .bind(uid)
            .fetch_optional(&mut tx)
            .await?;

        if ok.is_some() {
            query("UPDATE last_uids SET last_uid = $1 WHERE uid = $2")
                .bind(last_id)
                .bind(uid)
                .execute(&mut tx)
                .await?;
        } else {
            query("INSERT INTO last_uids (uid, last_uid) VALUES ($1, $2)")
                .bind(uid)
                .bind(last_id)
                .execute(&mut tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn get_last_id(&self, uid: &str) -> Result<Option<i64>> {
        let res = query("SELECT last_uid FROM last_uids WHERE uid = $1")
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?;

        match res {
            None => Ok(None),
            Some(v) => v.try_get("last_uid").map(Result::Ok)?,
        }
    }
}
