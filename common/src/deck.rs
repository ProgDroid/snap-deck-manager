use base64::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Deck {
    pub id: Option<String>,
    pub name: String,
    pub cards: Vec<String>,
}

// TODO distinguish between new deck (without ID) and loaded deck

struct CardId {
    _source: String,
    processed: String,
}

impl CardId {
    /// Share code card names are as follows:
    /// Start with Card ID e.g. `SilverSable`
    /// Count characters e.g. 11
    /// Remove vowels e.g. `SlvrSbl`
    /// Add character in front for how many characters, from 1 to 9 and starting at A for 10 e.g. `SlvrSblB`
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_lossless)]
    pub fn new(id: String) -> Self {
        let count = id.chars().count();

        let vowels_removed: String = id
            .chars()
            .filter(|c| !['a', 'e', 'i', 'o', 'u', 'y'].contains(c))
            .collect();

        let processed = format!("{vowels_removed}{count:X}");

        Self {
            _source: id,
            processed,
        }
    }
}

impl Deck {
    #[must_use]
    pub const fn new(name: String, cards: Vec<String>) -> Self {
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
            .map(|card| CardId::new(card.clone()).processed)
            .collect();

        let deck_code = card_codes.join(",");

        BASE64_STANDARD.encode(deck_code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_id() {
        let card_id = CardId::new(String::from("SilverSable"));

        assert_eq!(card_id.processed, String::from("SlvrSblB"));

        let card_id = CardId::new(String::from("Bast"));

        assert_eq!(card_id.processed, String::from("Bst4"));

        let card_id = CardId::new(String::from("Abomination"));

        assert_eq!(card_id.processed, String::from("AbmntnB"));

        let card_id = CardId::new(String::from("RavonnaRenslayer"));

        assert_eq!(card_id.processed, String::from("RvnnRnslr10"));

        let card_id = CardId::new(String::from("HighEvolutionary"));

        assert_eq!(card_id.processed, String::from("HghEvltnr10"));

        let card_id = CardId::new(String::from("JeffTheBabyLandShark"));

        assert_eq!(card_id.processed, String::from("JffThBbLndShrk14"));

        let card_id = CardId::new(String::from("LadyDeathstrike"));

        assert_eq!(card_id.processed, String::from("LdDthstrkF"));

        let card_id = CardId::new(String::from("NegasonicTeenageWarhead"));

        assert_eq!(card_id.processed, String::from("NgsncTngWrhd17"));

        let card_id = CardId::new(String::from("SpiderMan2099"));

        assert_eq!(card_id.processed, String::from("SpdrMn2099D"));

        let card_id = CardId::new(String::from("SymbioteSpiderMan"));

        assert_eq!(card_id.processed, String::from("SmbtSpdrMn11"));
    }
}
// TODO test with deck s7qy0od3v5qhsij2y5gg
