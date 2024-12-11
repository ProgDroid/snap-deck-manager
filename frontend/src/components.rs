use common::{card::Card, deck::Deck, package::Package};

use crate::{
    api::{
        decks::{create as create_deck, update as update_deck},
        packages::{create as create_package, update as update_package},
    },
    route::Route,
};

pub mod button;
pub mod card_list;
pub mod card_picker;
pub mod cards;
pub mod clickable;
pub mod deck;
pub mod edit_form;
pub mod filters;
pub mod footer;
pub mod form;
pub mod header;
pub mod icon_link;
pub mod label;
pub mod logo;
pub mod nav_link;
pub mod packages;
pub mod share_code_input;
pub mod submit;
pub mod textbox;
pub mod vertical_centre_container;

pub trait IntoClass {
    fn into_class() -> String;
}

impl IntoClass for Deck {
    #[inline]
    fn into_class() -> String {
        "deck".to_owned()
    }
}

impl IntoClass for Package {
    #[inline]
    fn into_class() -> String {
        "package".to_owned()
    }
}

pub trait IntoLabel {
    fn into_label() -> String;
}

impl IntoLabel for Deck {
    #[inline]
    fn into_label() -> String {
        "Deck".to_owned()
    }
}

impl IntoLabel for Package {
    #[inline]
    fn into_label() -> String {
        "Package".to_owned()
    }
}

pub trait CardCollection {
    fn id(&self) -> String;

    fn name(&self) -> String;

    fn cards(&self) -> Vec<Card>;

    fn display_packages(&self) -> bool;

    async fn create(name: String, cards: Vec<Card>) -> Option<String>;

    async fn update(id: String, name: String, cards: Vec<Card>) -> Option<String>;
}

impl CardCollection for Deck {
    fn id(&self) -> String {
        self.id.clone().unwrap() // TODO fix
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    fn display_packages(&self) -> bool {
        true
    }

    async fn create(name: String, cards: Vec<Card>) -> Option<String> {
        match create_deck(name, cards).await {
            Ok(response) => Some(response.id),
            Err(_e) => {
                // TODO log e
                None
            }
        }
    }

    async fn update(id: String, name: String, cards: Vec<Card>) -> Option<String> {
        match update_deck(id.clone(), name, cards).await {
            Ok(()) => Some(id),
            Err(_e) => {
                // TODO log e
                None
            }
        }
    }
}

impl CardCollection for Package {
    fn id(&self) -> String {
        self.id.clone().unwrap() // TODO fix
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn cards(&self) -> Vec<Card> {
        self.cards.clone()
    }

    fn display_packages(&self) -> bool {
        false
    }

    async fn create(name: String, cards: Vec<Card>) -> Option<String> {
        match create_package(name, cards).await {
            Ok(response) => Some(response.id),
            Err(_e) => {
                // TODO log e
                None
            }
        }
    }

    async fn update(id: String, name: String, cards: Vec<Card>) -> Option<String> {
        match update_package(id.clone(), name, cards).await {
            Ok(()) => Some(id),
            Err(_e) => {
                // TODO log e
                None
            }
        }
    }
}

pub trait ToRoute {
    fn into_route(id: String) -> Route;
}

impl ToRoute for Deck {
    #[inline]
    fn into_route(id: String) -> Route {
        Route::DeckView { deck_id: id }
    }
}

impl ToRoute for Package {
    #[inline]
    fn into_route(id: String) -> Route {
        Route::PackageView { package_id: id }
    }
}
