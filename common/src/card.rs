use serde::{Deserialize, Deserializer, Serialize};
use serde_aux::prelude::*;

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Card {
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
    pub source: String, // ? is this even needed, especially if I make series an enum
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
    pub series: String, // ? enum
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

impl Card {
    #[must_use]
    pub fn art(&self) -> String {
        format!("https://static.marvelsnap.pro/cards/{}.webp", self.id)
    }
}
