use std::time::UNIX_EPOCH;

use common::{card::Card, card_series::CardSeries};
use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::*;

use super::{series_start_date::SeriesStartDate, share_code::ShareCode};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ThirdPartyCard {
    #[serde(rename = "CardDefId")]
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "deserialise_encoded")]
    pub description: String,
    #[serde(deserialize_with = "deserialise_abilities")]
    pub abilities: Vec<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub cost: i8,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub power: i8,
    pub category: String,
    #[serde(deserialize_with = "deserialise_abilities")]
    pub connected_cards: Vec<String>,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub collectible: bool,
    #[serde(
        rename = "is_Token",
        deserialize_with = "deserialize_bool_from_anything"
    )]
    pub is_token: bool,
    #[serde(rename = "PrimaryColor")]
    pub primary_colour: String,
    #[serde(rename = "SecondaryColor")]
    pub secondary_colour: String,
    #[serde(rename = "RingColor")]
    pub ring_colour: String,
    #[serde(rename = "CardSeriesDefId")]
    pub series: String,
    #[serde(
        rename = "SeriesStartDates",
        deserialize_with = "deserialise_series_start_dates"
    )]
    pub series_start_dates: Vec<SeriesStartDate>,
}

fn deserialise_abilities<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MultipleTypes {
        Vec(Vec<String>),
        Str(String),
        None(()),
    }

    Ok(match MultipleTypes::deserialize(deserializer)? {
        MultipleTypes::Str(v) => serde_json::from_str(v.as_str()).unwrap_or_default(),
        MultipleTypes::Vec(v) => v,
        MultipleTypes::None(()) => Vec::default(),
    })
}

fn deserialise_encoded<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    Ok(s.replace('\\', ""))
}

fn deserialise_series_start_dates<'de, D>(deserializer: D) -> Result<Vec<SeriesStartDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let str: String = Deserialize::deserialize(deserializer)?;

    let vec: Vec<SeriesStartDate> = serde_json::from_str(str.as_str()).unwrap_or_default();

    Ok(vec)
}

impl ThirdPartyCard {
    pub fn into_card_model(self) -> Card {
        let mut series = CardSeries::from_string(self.series);
        let now = std::time::SystemTime::now();
        let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs(); // TODO handle

        let mut released = false;

        if !self.series_start_dates.is_empty() {
            let start_date = self.series_start_dates.first().unwrap();

            // TODO fix
            if series == CardSeries::None {
                series = CardSeries::from_string(start_date.data.clone());
            }

            #[allow(clippy::cast_sign_loss)]
            if start_date.start_date < 0 {
                released = true;
            } else {
                released = start_date.start_date as u64 <= timestamp;
            }
        }

        let released = !self.is_token && released;
        let share_code = ShareCode::new(self.id.clone());

        Card {
            id: self.id,
            name: self.name,
            description: self.description,
            abilities: self.abilities,
            cost: self.cost,
            power: self.power,
            category: self.category,
            connected_cards: self.connected_cards,
            is_token: self.is_token,
            released,
            primary_colour: self.primary_colour,
            secondary_colour: self.secondary_colour,
            ring_colour: self.ring_colour,
            series,
            share_code: share_code.processed,
        }
    }
}
