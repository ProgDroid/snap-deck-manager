use serde::{Deserialize, Serialize};

use crate::card_series::CardSeries;

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub description: String,
    pub abilities: Vec<String>,
    pub cost: i8,
    pub power: i8,
    pub category: String,
    pub connected_cards: Vec<String>,
    pub is_token: bool,
    pub released: bool,
    pub primary_colour: String,
    pub secondary_colour: String,
    pub ring_colour: String,
    pub series: CardSeries,
    pub share_code: String,
}

impl Card {
    #[must_use]
    pub fn art(&self) -> String {
        format!("https://static.marvelsnap.pro/cards/{}.webp", self.id)
    }
}
