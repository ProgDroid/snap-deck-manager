use common::package::Package as PackageModel;
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
    pub fn from_model(package: PackageModel) -> Self {
        let cards: Vec<String> = package.cards.iter().map(|card| card.id.clone()).collect();

        Self {
            id: package.id,
            name: package.name,
            cards,
        }
    }
}
