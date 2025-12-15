use crate::Persistence;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Local {
    path: PathBuf,
}

type Model = HashMap<String, i64>;

impl Local {
    pub fn new<P: Into<PathBuf>>(path: P) -> Local {
        Local { path: path.into() }
    }

    fn load(&self) -> Result<Model> {
        let f = match File::open(&self.path) {
            Ok(f) => f,
            Err(err) if matches!(err.kind(), ErrorKind::NotFound) => return Ok(Model::new()),
            Err(err) => return Err(err.into()),
        };
        Ok(serde_json::from_reader(f)?)
    }

    fn store(&self, model: &Model) -> Result<()> {
        let f = File::create(&self.path)?;
        serde_json::to_writer(f, model)?;
        Ok(())
    }
}

#[async_trait]
impl Persistence for Local {
    async fn set_last_id(&self, uid: &str, last_id: i64) -> Result<()> {
        let mut m = self.load()?;
        m.insert(uid.to_string(), last_id);
        self.store(&m)
    }

    async fn get_last_id(&self, uid: &str) -> Result<Option<i64>> {
        let m = self.load()?;
        Ok(m.get(uid).copied())
    }
}
