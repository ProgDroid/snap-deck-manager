use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Package {
    pub id: Option<String>,
    pub name: String,
    pub cards: Vec<String>,
}

// TODO distinguish between new package (without ID) and loaded package

impl Package {
    #[must_use]
    pub const fn new(name: String, cards: Vec<String>) -> Self {
        Self {
            id: None,
            name,
            cards,
        }
    }
}
