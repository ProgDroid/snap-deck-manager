use anyhow::Result;
use common::{card::Card, deck::Deck};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

pub async fn get(id: &String) -> Result<Deck> {
    let url = format!("/api/deck/{id}");

    let response = Request::get(&url).send().await?;

    Ok(response.json().await?)
}

#[derive(Serialize)]
struct PostBody {
    name: String,
    cards: Vec<Card>,
}

#[derive(Deserialize)]
pub struct PostResponse {
    pub id: String,
}

pub async fn create(name: String, cards: Vec<Card>) -> Result<PostResponse> {
    let url = "/api/deck";

    let body = PostBody { name, cards };

    let response = Request::post(url).json(&body)?.send().await?;

    Ok(response.json().await?)
}

pub async fn list() -> Result<Vec<Deck>> {
    let response = Request::get("/api/decks").send().await?;

    Ok(response.json().await?)
}

#[derive(Serialize)]
struct PatchBody {
    name: String,
    cards: Vec<Card>,
}

pub async fn update(id: String, name: String, cards: Vec<Card>) -> Result<()> {
    let url = format!("/api/deck/{id}");

    let body = PatchBody { name, cards };

    let _ = Request::patch(&url).json(&body)?.send().await?;

    Ok(())
}
