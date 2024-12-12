use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct SeriesStartDate {
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "StartDate")]
    pub start_date: i64,
}
