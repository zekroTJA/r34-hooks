use anyhow::Result;
use figment::Figment;
use figment::providers::{Env, Format, Toml, Yaml};
use scraper::WatchMap;
use scraper::hooks::HookImpl;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::ops::Deref;
use std::path::{Path, PathBuf};

fn default_storage_dir() -> PathBuf {
    "storage.json".into()
}

fn default_log_level() -> String {
    "info".into()
}

fn default_limit() -> usize {
    100
}

#[derive(Deserialize, Debug)]
pub struct Target {
    pub id: Option<String>,
    pub tags: Vec<String>,
    pub hook: HookImpl,
}

#[derive(Deserialize, Debug)]
pub struct StaticConfig {
    #[serde(default = "default_storage_dir")]
    pub storage_dir: PathBuf,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    pub schedule: Option<String>,
    pub dynamic_config_path: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
pub struct DynamicConfig {
    pub user_id: String,
    pub api_token: String,
    pub default_tags: Option<Vec<String>>,
    pub targets: Vec<Target>,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

impl DynamicConfig {
    pub fn get_watch_map(&self) -> WatchMap {
        let mut map = WatchMap::new();

        for target in &self.targets {
            let id = target
                .id
                .clone()
                .unwrap_or_else(|| target.tags.join(",").to_string());
            map.insert(id, (target.tags.clone(), target.hook.clone()));
        }

        map
    }
}

pub fn parse_from_env<C>(prefix: &str) -> Result<C>
where
    C: DeserializeOwned,
{
    Ok(Figment::new().merge(Env::prefixed(prefix)).extract()?)
}

pub fn parse<P, C>(path: P) -> Result<C>
where
    P: AsRef<Path>,
    C: DeserializeOwned,
{
    let ext = path.as_ref().extension().unwrap_or_default();
    let mut figment = Figment::new();

    figment = match ext.to_string_lossy().deref() {
        "yml" | "yaml" => figment.merge(Yaml::file(path)),
        "toml" => figment.merge(Toml::file(path)),
        _ => anyhow::bail!("invalid config file type"),
    };

    Ok(figment.extract()?)
}
