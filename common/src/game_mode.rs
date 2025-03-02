use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum GameMode {
    #[default]
    Ranked,
    Conquest,
    DeadpoolsDiner,
    HighVoltage,
    SanctumShowdown,
}

impl GameMode {
    pub fn from_string<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        match value.into().as_str() {
            "Conquest" => Self::Conquest,
            "DeadpoolsDiner" => Self::DeadpoolsDiner,
            "HighVoltage" => Self::HighVoltage,
            "SanctumShowdown" => Self::SanctumShowdown,
            _ => Self::Ranked,
        }
    }
}
