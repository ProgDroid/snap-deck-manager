use base64::prelude::*;
use serde::{Deserialize, Serialize};

use crate::card::Card;

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Deck {
    pub id: Option<String>,
    pub name: String,
    pub cards: Vec<Card>,
}

// TODO distinguish between new deck (without ID) and loaded deck

impl Deck {
    #[must_use]
    pub const fn new(name: String, cards: Vec<Card>) -> Self {
        Self {
            id: None,
            name,
            cards,
        }
    }

    #[must_use]
    pub fn share_code(&self) -> String {
        let card_codes: Vec<String> = self
            .cards
            .iter()
            .map(|card| card.share_code.clone())
            .collect();

        let deck_code = card_codes.join(",");

        BASE64_STANDARD.encode(deck_code)
    }
}
