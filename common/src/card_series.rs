use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Default, Display, EnumIter, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum CardSeries {
    #[default]
    None,
    Starter,
    Recruit,
    Series0,
    Series1,
    Series2,
    Series3,
    Series4,
    Series5,
    LimitedTimeEvent,
    SeasonPass,
}

impl CardSeries {
    pub fn from_string<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        match value.into().as_str() {
            "Starter" => Self::Starter,
            "Recruit" => Self::Recruit,
            "Series0" => Self::Series0,
            "Series1" => Self::Series1,
            "Series2" => Self::Series2,
            "Series3" => Self::Series3,
            "Series4" => Self::Series4,
            "Series5" => Self::Series5,
            "LimitedTimeEvent" => Self::LimitedTimeEvent,
            "SeasonPass" => Self::SeasonPass,
            _ => Self::None,
        }
    }
}
