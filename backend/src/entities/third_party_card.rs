use common::{card::Card, card_series::CardSeries};
use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::*;

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

impl ThirdPartyCard {
    pub fn into_card_model(self) -> Card {
        let series = CardSeries::from_string(self.series);
        let released = !self.is_token && series != CardSeries::None;

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
        }
    }
}
