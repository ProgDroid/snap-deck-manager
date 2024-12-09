use anyhow::Result;
use config_file::FromConfigFile;
use serde::Deserialize;
use std::env;

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
        let path = match env::var("CONFIG_API") {
            Ok(val) => val,
            Err(e) => {
                // TODO log e
                "api.toml".to_owned()
            }
        };

        Ok(Self::from_config_file(path)?)
    }
}
