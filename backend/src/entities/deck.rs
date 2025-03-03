use common::deck::Deck as DeckModel;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Deck {
    pub id: Option<String>,
    pub name: String,
    pub cards: Vec<RecordId>,
    pub game_modes: Vec<RecordId>,
}

impl Deck {
    #[must_use]
    pub fn from_model(deck: &DeckModel) -> Self {
        let cards: Vec<RecordId> = deck
            .cards
            .iter()
            .map(|card| RecordId::from(("card", card.id.clone())))
            .collect();

        let game_modes: Vec<RecordId> = deck
            .game_modes
            .iter()
            .map(|mode| RecordId::from(("game_mode", mode.id.clone())))
            .collect();

        Self {
            id: deck.id.clone(),
            name: deck.name.clone(),
            cards,
            game_modes,
        }
    }
}
