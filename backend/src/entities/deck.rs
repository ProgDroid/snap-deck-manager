use common::deck::Deck as DeckModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Deck {
    pub id: Option<String>,
    pub name: String,
    pub cards: Vec<String>,
}

impl Deck {
    #[must_use]
    pub fn from_model(deck: &DeckModel) -> Self {
        let cards: Vec<String> = deck.cards.iter().map(|card| card.id.clone()).collect();

        Self {
            id: deck.id.clone(),
            name: deck.name.clone(),
            cards,
        }
    }
}
