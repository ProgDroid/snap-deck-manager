use anyhow::Result;
use config_file::FromConfigFile;
use serde::Deserialize;

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
        Ok(Self::from_config_file("backend/database.toml")?)
    }
}
