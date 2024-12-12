use serde::{Deserialize, Serialize};

use crate::card::Card;

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Deck {
    pub id: Option<String>,
    pub name: String,
    pub cards: Vec<Card>,
    pub share_code: String,
}

// TODO distinguish between new deck (without ID) and loaded deck

impl Deck {
    #[must_use]
    pub const fn new(name: String, cards: Vec<Card>, share_code: String) -> Self {
        Self {
            id: None,
            name,
            cards,
            share_code,
        }
    }
}
