use crate::config::{DynamicConfig, StaticConfig};
use anyhow::Result;
use argh::FromArgs;
use persistence::{Local, PersistenceImpl, Postgres};
use scraper::Scraper;
use std::path::{Path, PathBuf};
use tokio_cron_scheduler::{Job, JobScheduler};

mod config;

#[derive(FromArgs)]
/// r34-hooks runner CLI
struct Args {
    /// path to optional static config file
    #[argh(positional)]
    config_file: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let cfg: StaticConfig = match args.config_file {
        Some(ref p) => config::parse(p)?,
        None => config::parse_from_env("R34_")?,
    };

    let log_level: tracing::Level = cfg.log_level.parse()?;

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_writer(std::io::stdout)
        .init();

    let storage = match &cfg.storage {
        config::Storage::Local { storage_dir } => PersistenceImpl::Local(Local::new(storage_dir)),
        config::Storage::Postgres { dsn } => PersistenceImpl::Postgres(Postgres::new(dsn).await?),
    };

    let dynamic_config_path = cfg
        .dynamic_config_path
        .or_else(|| args.config_file.map(|v| v.into()))
        .ok_or_else(|| anyhow::anyhow!("No path to dynamic config has been specified"))?;

    let _ = parse_dynamic_config(&dynamic_config_path)?;

    if let Some(schedule) = cfg.schedule {
        let sched = JobScheduler::new().await?;

        let job = Job::new_async(&schedule, move |_uuid, _l| {
            let s = storage.clone();
            let p = dynamic_config_path.clone();
            Box::pin(async move {
                if let Err(err) = run(s, p).await {
                    tracing::error!("scraper run failed: {err}")
                };
            })
        })?;

        sched.add(job).await?;
        sched.shutdown_on_ctrl_c();
        sched.start().await?;

        tracing::info!("Running in scheduler mode ({schedule}) ...");
        tokio::signal::ctrl_c().await?;
    } else {
        tracing::info!("Running in one-shot mode");
        run(storage, dynamic_config_path).await?;
    }

    Ok(())
}

fn parse_dynamic_config(cfg_path: impl AsRef<Path>) -> Result<DynamicConfig> {
    config::parse(cfg_path)
}

async fn run(storage: PersistenceImpl, cfg_path: impl AsRef<Path>) -> Result<()> {
    let cfg = parse_dynamic_config(cfg_path)?;

    let sc = Scraper::new(
        storage,
        cfg.get_watch_map(),
        cfg.limit,
        cfg.default_tags,
        cfg.user_id,
        cfg.api_token,
    );

    sc.run().await
}
