use anyhow::Result;
use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Api {
    host: String,
    port: u16,
}

impl Api {
    pub fn to_address(&self) -> (&str, u16) {
        (self.host.as_str(), self.port)
    }

    pub fn new() -> Result<Self> {
        Ok(Self::from_config_file("backend/api.toml")?)
    }
}
