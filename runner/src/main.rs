use anyhow::Result;
use persistence::{Local, PersistenceImpl};
use scraper::Scraper;
use std::env;

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let config_file = env::args()
        .nth(1)
        .map(|a| a.to_string())
        .unwrap_or_else(|| "config.yaml".to_string());

    let cfg = config::Config::parse(&config_file)?;

    let log_level = cfg
        .log_level
        .as_ref()
        .map(|l| l.parse())
        .transpose()?
        .unwrap_or(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_writer(std::io::stdout)
        .init();

    let storage = PersistenceImpl::Local(Local::new(&cfg.storage_dir));

    let sc = Scraper::new(
        storage,
        cfg.get_watch_map(),
        100,
        cfg.default_tags,
        cfg.user_id,
        cfg.api_token,
    );

    sc.run().await?;

    Ok(())
}
