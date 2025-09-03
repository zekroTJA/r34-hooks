use crate::{
    hooks::{discord::Discord, HookImpl},
    Scraper, WatchMap,
};
use anyhow::Result;
use persistence::{PersistenceImpl, Postgres};
use std::{collections::HashMap, env};

const ENV_PREFIX: &str = "R34_";

enum WatchPart {
    Tags(Vec<String>),
    Hook(HookImpl),
}

impl Scraper {
    pub async fn from_env() -> Result<Self> {
        let mut db = None;
        let mut limit = None;
        let mut default_tags = None;
        let mut user_id = None;
        let mut api_key = None;

        let mut tags: HashMap<String, Vec<String>> = HashMap::new();
        let mut hooks: HashMap<String, HookImpl> = HashMap::new();

        let vars: Vec<_> = env::vars()
            .map(|(k, v)| (k.strip_prefix(ENV_PREFIX).map(|s| s.to_owned()), v))
            .filter(|(k, _)| k.is_some())
            .map(|(k, v)| {
                (
                    k.unwrap()
                        .to_uppercase()
                        .split('_')
                        .map(|e| e.to_owned())
                        .collect::<Vec<_>>(),
                    v,
                )
            })
            .filter(|(k, _)| !k.is_empty())
            .collect();

        for (key, val) in vars {
            let Some(first) = key.first() else {
                continue;
            };

            match first.as_str() {
                "LIMIT" => limit = Some(val.parse()?),
                "DEFAULTTAGS" => default_tags = get_tags(&val),
                "DATABASE" => db = get_database(&key[1..], &val).await?,
                "WATCH" => {
                    let (uid, part) = get_watchpart(&key[1..], &val)?;
                    match part {
                        WatchPart::Tags(t) => {
                            tags.insert(uid, t);
                        }
                        WatchPart::Hook(h) => {
                            hooks.insert(uid, h);
                        }
                    }
                }
                "USERID" => user_id = Some(val.to_string()),
                "APIKEY" => api_key = Some(val.to_string()),
                _ => {}
            }
        }

        let Some(db) = db else {
            anyhow::bail!("No database has been configured");
        };

        let mut watchers = WatchMap::new();
        for (uid, t) in tags {
            let Some(hook) = hooks.get(&uid) else {
                anyhow::bail!("No hook defined for tags in ID {uid}");
            };
            watchers.insert(uid, (t, hook.clone()));
        }

        if watchers.is_empty() {
            anyhow::bail!("No watchers have been specified");
        }

        Ok(Self {
            db,
            watchers,
            default_tags,
            limit: limit.unwrap_or(100),
            user_id: user_id.ok_or_else(|| anyhow::anyhow!("USERID not set"))?,
            api_key: api_key.ok_or_else(|| anyhow::anyhow!("APIKEY not set"))?,
        })
    }
}

async fn get_database(keys: &[String], val: &str) -> Result<Option<PersistenceImpl>> {
    let Some(first) = keys.first() else {
        return Ok(None);
    };

    match first.as_str() {
        "POSTGRES" => {
            let pg = Postgres::new(val).await?;
            Ok(Some(PersistenceImpl::Postgres(pg)))
        }
        _ => Ok(None),
    }
}

fn get_watchpart(keys: &[String], val: &str) -> Result<(String, WatchPart)> {
    if keys.len() < 2 {
        anyhow::bail!(
            "WATCH environment key must be in format WATCH_<ID>_TAGS or WATCH_<ID>_HOOK_<HookImpl>"
        );
    }

    let uid = &keys[0];
    let typ = &keys[1];

    match typ.as_str() {
        "TAGS" => Ok((uid.to_owned(), WatchPart::Tags(get_watch_tags(val)))),
        "HOOK" => get_watch_hook(&keys[2..], val).map(|h| (uid.to_owned(), WatchPart::Hook(h))),
        _ => anyhow::bail!(
            "WATCH environment key must be in format WATCH_<ID>_TAGS or WATCH_<ID>_HOOK_<HookImpl>"
        ),
    }
}

fn get_watch_tags(val: &str) -> Vec<String> {
    val.split(',')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_owned())
        .collect()
}

fn get_watch_hook(keys: &[String], val: &str) -> Result<HookImpl> {
    let typ = keys.first().map(|v| v.as_str()).unwrap_or_default();
    match typ {
        "DISCORD" => Ok(HookImpl::Discord(Discord::new(val))),
        _ => anyhow::bail!("unsupported or invalid hook type"),
    }
}

fn get_tags(val: &str) -> Option<Vec<String>> {
    let val = val.trim();
    let tags: Vec<_> = val
        .split(',')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_owned())
        .collect();
    if tags.is_empty() {
        return None;
    }
    Some(tags)
}
