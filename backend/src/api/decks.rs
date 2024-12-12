use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    Result,
};
use serde::{Deserialize, Serialize};

use common::{card::Card, deck::Deck};

use crate::{
    repository::surrealdb::SurrealDbRepository, services::share_code::encode_share_code_strings,
};

#[get("/deck/{deck_id}")]
pub async fn get(
    deck_identifier: Path<String>,
    db: Data<SurrealDbRepository>,
) -> Result<Json<Deck>> {
    let deck = db.get_deck(deck_identifier.into_inner()).await;

    Ok(Json(deck.expect("Could not find deck")))
}

#[derive(Deserialize)]
struct PostData {
    name: String,
    cards: Vec<Card>,
}

#[derive(Serialize)]
struct PostResponse {
    id: String,
}

#[post("/deck")]
pub async fn create(
    db: Data<SurrealDbRepository>,
    body: Json<PostData>,
) -> Result<Json<PostResponse>> {
    let id = db
        .create_deck(Deck::new(
            body.name.clone(),
            body.cards.clone(),
            share_code(&body),
        ))
        .await;

    let id = id.expect("Could not create deck");

    let post_response = PostResponse { id: id.id.to_raw() };

    Ok(Json(post_response))
}

#[patch("/deck/{deck_id}")]
pub async fn update(
    deck_id: Path<String>,
    db: Data<SurrealDbRepository>,
    body: Json<PostData>,
) -> Result<Json<PostResponse>> {
    let mut deck = Deck::new(body.name.clone(), body.cards.clone(), share_code(&body));
    deck.id = Some(deck_id.clone());

    let id = db.update_deck(deck).await;

    id.expect("Could not update deck");

    let post_response = PostResponse {
        id: deck_id.into_inner(),
    };

    Ok(Json(post_response))
}

#[get("/decks")]
pub async fn list(db: Data<SurrealDbRepository>) -> Result<Json<Vec<Deck>>> {
    let decks = db.get_all_decks().await;

    Ok(Json(decks))
}

fn share_code(post_data: &PostData) -> String {
    let codes: Vec<String> = post_data
        .cards
        .iter()
        .map(|card| card.share_code.clone())
        .collect();

    encode_share_code_strings(&codes)
}
