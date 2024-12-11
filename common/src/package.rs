use serde::{Deserialize, Serialize};

use crate::card::Card;

#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Package {
    pub id: Option<String>,
    pub name: String,
    pub cards: Vec<Card>,
}

// TODO distinguish between new package (without ID) and loaded package

impl Package {
    #[must_use]
    pub const fn new(name: String, cards: Vec<Card>) -> Self {
        Self {
            id: None,
            name,
            cards,
        }
    }
}
