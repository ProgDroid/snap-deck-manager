use serde::{Deserialize, Serialize};

use crate::{card::Card, game_mode::GameMode};

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Deck {
    pub id: Option<String>,
    pub name: String,
    pub cards: Vec<Card>,
    pub share_code: String,
    pub game_modes: Vec<GameMode>,
}

// TODO distinguish between new deck (without ID) and loaded deck

impl Deck {
    #[must_use]
    pub const fn new(
        name: String,
        cards: Vec<Card>,
        share_code: String,
        game_modes: Vec<GameMode>,
    ) -> Self {
        Self {
            id: None,
            name,
            cards,
            share_code,
            game_modes,
        }
    }
}
