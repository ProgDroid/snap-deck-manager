use std::collections::HashMap;

use anyhow::Result;
use log::{debug, error};
use serde::Deserialize;
use strum::Display;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::{auth::Root, Resource},
    sql::Thing,
    Surreal,
};

use common::{
    card::Card, card_series::CardSeries, deck::Deck, game_mode::GameMode, package::Package,
};

use crate::{config::db::Db as DbConfig, services::share_code::encode_share_code_strings};

use crate::entities::third_party_card::ThirdPartyCard;

// TODO could be split into different files which all have a DB connection

#[derive(Deserialize, Debug)]
struct RecordCard {
    pub id: Thing,
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
    pub series: String,
    pub share_code: String,
}

impl RecordCard {
    fn into_card(self) -> Card {
        Card {
            id: (self.id.to_string()[5..]).to_owned(),
            name: self.name,
            description: self.description,
            abilities: self.abilities,
            cost: self.cost,
            power: self.power,
            category: self.category,
            connected_cards: self.connected_cards,
            is_token: self.is_token,
            released: self.released,
            primary_colour: self.primary_colour,
            secondary_colour: self.secondary_colour,
            ring_colour: self.ring_colour,
            series: CardSeries::from_string(self.series),
            share_code: self.share_code,
        }
    }
}

#[derive(Deserialize)]
struct RecordDeck {
    id: Thing,
    name: String,
    cards: Vec<String>,
    game_modes: Vec<GameMode>,
}

impl RecordDeck {
    fn into_deck(self, cards: Vec<Card>) -> Deck {
        let codes: Vec<String> = cards.iter().map(|card| card.share_code.clone()).collect();

        let share_code = encode_share_code_strings(&codes);

        Deck {
            id: Some((self.id.to_string()[5..]).to_owned()),
            name: self.name,
            cards,
            share_code,
            game_modes: self.game_modes,
        }
    }
}

#[derive(Deserialize)]
struct RecordPackage {
    id: Thing,
    name: String,
    cards: Vec<String>,
}

impl RecordPackage {
    fn into_package(self, cards: Vec<Card>) -> Package {
        Package {
            id: Some((self.id.to_string()[8..]).to_owned()),
            name: self.name,
            cards,
        }
    }
}

pub struct SurrealDbRepository {
    db: Surreal<Client>,
}

#[derive(Display, Debug)]
enum Operation {
    GetDeck(String),
    PostDeck(Deck),
    GetDecks,
    PatchDeck(Deck),
    GetPackage(String),
    PostPackage(Package),
    GetPackages,
    PatchPackage(Package),
    GetAllCards,
    GetSelectableCards,
    GetCards(Vec<String>),
    GetCardsFromShareCode(Vec<String>),
    UpdateAllCards,
}

impl SurrealDbRepository {
    pub async fn new(db_config: DbConfig) -> Result<Self> {
        let db = Surreal::new::<Ws>(db_config.to_address()).await.unwrap();

        db.signin(Root {
            username: db_config.user.as_str(),
            password: db_config.pass.as_str(),
        })
        .await?;

        db.use_ns(db_config.namespace)
            .use_db(db_config.database)
            .await?;

        db.query(
            "
            DEFINE TABLE IF NOT EXISTS card SCHEMAFULL;
            DEFINE FIELD IF NOT EXISTS name ON TABLE card TYPE string;
            DEFINE FIELD IF NOT EXISTS description ON TABLE card TYPE string;
            DEFINE FIELD IF NOT EXISTS abilities ON TABLE card TYPE array<string>;
            DEFINE FIELD IF NOT EXISTS cost ON TABLE card TYPE int;
            DEFINE FIELD IF NOT EXISTS power ON TABLE card TYPE int;
            DEFINE FIELD IF NOT EXISTS category ON TABLE card TYPE string;
            DEFINE FIELD IF NOT EXISTS connected_cards ON TABLE card TYPE array<string>;
            DEFINE FIELD IF NOT EXISTS is_token ON TABLE card TYPE bool READONLY;
            DEFINE FIELD IF NOT EXISTS released ON TABLE card TYPE bool;
            DEFINE FIELD IF NOT EXISTS primary_colour ON TABLE card TYPE string READONLY;
            DEFINE FIELD IF NOT EXISTS secondary_colour ON TABLE card TYPE string READONLY;
            DEFINE FIELD IF NOT EXISTS ring_colour ON TABLE card TYPE string READONLY;
            DEFINE FIELD IF NOT EXISTS series ON TABLE card TYPE string;
            DEFINE FIELD IF NOT EXISTS share_code ON TABLE card TYPE string;

            DEFINE TABLE IF NOT EXISTS game_mode SCHEMAFULL;
            DEFINE FIELD IF NOT EXISTS name ON TABLE game_mode TYPE string;
            DEFINE INDEX IF NOT EXISTS unique_name ON TABLE game_mode FIELDS name UNIQUE;

            DEFINE TABLE IF NOT EXISTS deck SCHEMAFULL;
            DEFINE FIELD IF NOT EXISTS name ON TABLE deck TYPE string;
            DEFINE FIELD IF NOT EXISTS cards ON TABLE deck TYPE array<string, 12>;
            DEFINE FIELD IF NOT EXISTS game_mode ON TABLE card TYPE array<record<game_mode>>;
            DEFINE INDEX IF NOT EXISTS unique_name ON TABLE deck FIELDS name UNIQUE;

            DEFINE TABLE IF NOT EXISTS package SCHEMAFULL;
            DEFINE FIELD IF NOT EXISTS name ON TABLE package TYPE string;
            DEFINE FIELD IF NOT EXISTS cards ON TABLE package TYPE array<string, 12>;
            DEFINE INDEX IF NOT EXISTS unique_name ON TABLE package FIELDS name UNIQUE;

            ",
        )
        .await?;

        Ok(Self { db })
    }

    // ! Cards

    pub async fn update_all_cards(&self) -> Result<()> {
        log(&Operation::UpdateAllCards);

        let response: HashMap<String, ThirdPartyCard> =
            reqwest::get("https://static2.marvelsnap.pro/snap/do.php?cmd=getcards")
                .await?
                .json::<HashMap<String, ThirdPartyCard>>()
                .await?;

        let third_party_cards: Vec<ThirdPartyCard> = response.values().cloned().collect();

        for card in third_party_cards {
            // TODO can I just let these run without awaiting?
            let result: surrealdb::Result<Option<RecordCard>> = self
                .db
                .upsert(("card", card.id.clone()))
                .content(card.into_card_model())
                .await;

            match result {
                Ok(_) => {}
                Err(_e) => {
                    // TODO log e
                }
            }
        }

        Ok(())
    }

    pub async fn get_all_cards(&self) -> Vec<Card> {
        log(&Operation::GetAllCards);

        let records: Result<Vec<RecordCard>, _> = self.db.select("card").await;

        // TODO there must be a better way, way too much processing
        records.map_or(Vec::default(), |list| {
            list.into_iter()
                .map(RecordCard::into_card)
                .collect::<Vec<Card>>()
        })
    }

    pub async fn get_all_selectable_cards(&self) -> Vec<Card> {
        log(&Operation::GetSelectableCards);

        let sql = "
            SELECT * FROM card
            WHERE is_token = false AND series != \"None\"
        ";

        let result = self.db.query(sql).await;

        let records: Vec<RecordCard> = result.unwrap().take(0).unwrap(); // TODO fix

        // TODO there must be a better way, way too much processing
        records
            .into_iter()
            .map(RecordCard::into_card)
            .collect::<Vec<Card>>()
    }

    pub async fn get_cards(&self, cards: Vec<String>) -> Vec<Card> {
        log(&Operation::GetCards(cards.clone()));

        let sql = "
            SELECT * FROM card
            WHERE id in $cards
        ";

        let card_filter = cards
            .iter()
            .map(|card_id| Thing::from(("card", card_id.as_str())))
            .collect::<Vec<Thing>>();

        let result = self.db.query(sql).bind(("cards", card_filter)).await;

        let records: Vec<RecordCard> = result.unwrap().take(0).unwrap(); // TODO fix

        // TODO there must be a better way, way too much processing
        records
            .into_iter()
            .map(RecordCard::into_card)
            .collect::<Vec<Card>>()
    }

    pub async fn get_cards_from_share_codes(&self, codes: Vec<String>) -> Vec<Card> {
        log(&Operation::GetCardsFromShareCode(codes.clone()));

        let sql = "
            SELECT * FROM card
            WHERE share_code in $codes
        ";

        let result = self.db.query(sql).bind(("codes", codes)).await;

        let records: Vec<RecordCard> = result.unwrap().take(0).unwrap(); // TODO fix

        // TODO there must be a better way, way too much processing
        records
            .into_iter()
            .map(RecordCard::into_card)
            .collect::<Vec<Card>>()
    }

    // ! Decks

    pub async fn create_deck(&self, deck: Deck) -> Option<Thing> {
        log(&Operation::PostDeck(deck.clone()));

        let deck_to_save = crate::entities::deck::Deck::from_model(&deck);

        let records: Result<Option<RecordDeck>, _> =
            self.db.create("deck").content(deck_to_save).await;

        match records {
            Ok(inner) => inner.map(|record| record.id),
            Err(e) => {
                error!("{e}");
                None
            }
        }
    }

    pub async fn get_deck(&self, deck_id: String) -> Option<Deck> {
        log(&Operation::GetDeck(deck_id.clone()));

        let record: Result<Option<RecordDeck>, _> =
            self.db.select(("deck", deck_id.as_str())).await;

        // TODO there must be a better way, way too much processing
        match record {
            Ok(val) => match val {
                Some(inner) => {
                    let cards = self.get_cards(inner.cards.clone()).await;

                    Some(RecordDeck::into_deck(inner, cards))
                }
                None => None,
            },
            Err(_e) => {
                // TODO log e
                None
            }
        }
    }

    pub async fn get_all_decks(&self) -> Vec<Deck> {
        log(&Operation::GetDecks);

        let records: Result<Vec<RecordDeck>, _> = self.db.select("deck").await;

        // TODO there must be a better way, way too much processing
        match records {
            Ok(val) => {
                let mut decks = Vec::default();

                for record in val {
                    let cards = self.get_cards(record.cards.clone()).await;

                    decks.push(RecordDeck::into_deck(record, cards));
                }

                decks
            }
            Err(_e) => {
                // TODO log e
                Vec::default()
            }
        }
    }

    pub async fn update_deck(&self, deck: Deck) -> Result<()> {
        log(&Operation::PatchDeck(deck.clone()));

        let deck_to_save = crate::entities::deck::Deck::from_model(&deck);

        let _ = self
            .db
            .update(Resource::from(("deck", deck.id.clone().unwrap()))) // TODO fix
            .content(deck_to_save)
            .await?;

        Ok(())
    }

    // ! Packages

    pub async fn create_package(&self, package: Package) -> Option<Thing> {
        log(&Operation::PostPackage(package.clone()));

        let package_to_save = crate::entities::package::Package::from_model(&package);

        let records: Result<Option<RecordPackage>, _> =
            self.db.create("package").content(package_to_save).await;

        match records {
            Ok(inner) => inner.map(|record| record.id),
            Err(e) => {
                error!("{e}");
                None
            }
        }
    }

    pub async fn get_package(&self, package_id: String) -> Option<Package> {
        log(&Operation::GetPackage(package_id.clone()));

        let record: Result<Option<RecordPackage>, _> =
            self.db.select(("package", package_id.as_str())).await;

        // TODO there must be a better way, way too much processing
        match record {
            Ok(val) => match val {
                Some(inner) => {
                    let cards = self.get_cards(inner.cards.clone()).await;

                    Some(RecordPackage::into_package(inner, cards))
                }
                None => None,
            },
            Err(_e) => {
                // TODO log e
                None
            }
        }
    }

    pub async fn get_all_packages(&self) -> Vec<Package> {
        log(&Operation::GetPackages);

        let records: Result<Vec<RecordPackage>, _> = self.db.select("package").await;

        // TODO there must be a better way, way too much processing
        match records {
            Ok(val) => {
                let mut packages = Vec::default();

                for record in val {
                    let cards = self.get_cards(record.cards.clone()).await;

                    packages.push(RecordPackage::into_package(record, cards));
                }

                packages
            }
            Err(_e) => {
                // TODO log e
                Vec::default()
            }
        }
    }

    pub async fn update_package(&self, package: Package) -> Result<()> {
        log(&Operation::PatchPackage(package.clone()));

        let package_to_save = crate::entities::package::Package::from_model(&package);

        let _ = self
            .db
            .update(Resource::from(("package", package.id.clone().unwrap()))) // TODO fix
            .content(package_to_save)
            .await?;

        Ok(())
    }
}

// ? Is this logging needed or even good? is it inefficient?
fn log(operation: &Operation) {
    debug!("{operation:?}");
}
