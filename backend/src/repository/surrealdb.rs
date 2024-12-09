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

use common::{deck::Deck, package::Package};

use crate::config::db::Db as DbConfig;

#[derive(Deserialize)]
struct Record {
    id: Thing,
    name: String,
    cards: Vec<String>,
}

impl Record {
    fn to_deck(&self) -> Deck {
        Deck {
            id: Some(String::from(&self.id.to_string()[5..])),
            name: self.name.clone(),
            cards: self.cards.clone(),
        }
    }

    fn to_package(&self) -> Package {
        Package {
            id: Some(String::from(&self.id.to_string()[8..])),
            name: self.name.clone(),
            cards: self.cards.clone(),
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

        Ok(Self { db })
    }

    // ! Decks

    pub async fn create_deck(&self, deck: Deck) -> Option<Thing> {
        log(&Operation::PostDeck(deck.clone()));

        let records: Result<Option<Record>, _> = self.db.create("deck").content(deck).await;

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

        let record: Result<Option<Record>, _> = self.db.select(("deck", deck_id.as_str())).await;

        record.map_or(None, |rec| rec.map(|record| record.to_deck()))
    }

    pub async fn get_all_decks(&self) -> Vec<Deck> {
        log(&Operation::GetDecks);

        let records: Result<Vec<Record>, _> = self.db.select("deck").await;

        // TODO there must be a better way, way too much processing
        records.map_or(Vec::default(), |list| {
            list.iter().map(Record::to_deck).collect::<Vec<Deck>>()
        })
    }

    pub async fn update_deck(&self, deck: Deck) -> Result<()> {
        log(&Operation::PatchDeck(deck.clone()));

        let _ = self
            .db
            .update(Resource::from(("deck", deck.id.clone().unwrap()))) // TODO fix
            .content(deck)
            .await?;

        Ok(())
    }

    // ! Packages

    pub async fn create_package(&self, package: Package) -> Option<Thing> {
        log(&Operation::PostPackage(package.clone()));

        let records: Result<Option<Record>, _> = self.db.create("package").content(package).await;

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

        let record: Result<Option<Record>, _> =
            self.db.select(("package", package_id.as_str())).await;

        record.map_or(None, |rec| rec.map(|record| record.to_package()))
    }

    pub async fn get_all_packages(&self) -> Vec<Package> {
        log(&Operation::GetPackages);

        let records: Result<Vec<Record>, _> = self.db.select("package").await;

        // TODO there must be a better way, way too much processing
        records.map_or(Vec::default(), |list| {
            list.iter()
                .map(Record::to_package)
                .collect::<Vec<Package>>()
        })
    }

    pub async fn update_package(&self, package: Package) -> Result<()> {
        log(&Operation::PatchPackage(package.clone()));

        let _ = self
            .db
            .update(Resource::from(("package", package.id.clone().unwrap()))) // TODO fix
            .content(package)
            .await?;

        Ok(())
    }
}

// ? Is this logging needed or even good? is it inefficient?
fn log(operation: &Operation) {
    debug!("{operation:?}");
}
