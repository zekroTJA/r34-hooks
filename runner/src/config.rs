use anyhow::Result;
use figment::Figment;
use figment::providers::{Format, Toml, Yaml};
use scraper::WatchMap;
use scraper::hooks::HookImpl;
use serde::Deserialize;
use std::ops::Deref;
use std::path::{Path, PathBuf};

fn default_storage_dir() -> PathBuf {
    "storage.json".into()
}

#[derive(Deserialize, Debug)]
pub struct Target {
    pub tags: Vec<String>,
    pub hook: HookImpl,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub user_id: String,
    pub api_token: String,
    #[serde(default = "default_storage_dir")]
    pub storage_dir: PathBuf,
    pub log_level: Option<String>,

    pub default_tags: Option<Vec<String>>,
    pub targets: Vec<Target>,
}

impl Config {
    pub fn parse<T: AsRef<Path>>(path: T) -> Result<Self> {
        let ext = path.as_ref().extension().unwrap_or_default();
        let mut figment = Figment::new();

        figment = match ext.to_string_lossy().deref() {
            "yml" | "yaml" => figment.merge(Yaml::file(path)),
            "toml" => figment.merge(Toml::file(path)),
            _ => anyhow::bail!("invalid config file type"),
        };

        Ok(figment.extract()?)
    }

    pub fn get_watch_map(&self) -> WatchMap {
        let mut map = WatchMap::new();

        for target in &self.targets {
            map.insert(
                target.tags.join(",").to_string(),
                (target.tags.clone(), target.hook.clone()),
            );
        }

        map
    }
}
