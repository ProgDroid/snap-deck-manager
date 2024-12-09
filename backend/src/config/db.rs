use anyhow::Result;
use config_file::FromConfigFile;
use serde::Deserialize;
use std::env;

// TODO use secrets here
// TODO don't make these pub, work out better way
#[derive(Deserialize)]
pub struct Db {
    host: String,
    port: String,
    pub user: String,
    pub pass: String,
    pub namespace: String,
    pub database: String,
}

impl Db {
    pub fn to_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn new() -> Result<Self> {
        let path = match env::var("CONFIG_DB") {
            Ok(val) => val,
            Err(e) => {
                // TODO log e
                "database.toml".to_owned()
            }
        };

        Ok(Self::from_config_file(path)?)
    }
}
