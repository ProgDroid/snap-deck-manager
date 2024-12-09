use anyhow::Result;
use common::{card::Card, package::Package};
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

pub async fn get(id: &String) -> Result<Package> {
    let url = format!("/api/package/{id}");

    let response = Request::get(&url).send().await?;

    Ok(response.json().await?)
}

#[derive(Serialize)]
struct PostBody {
    name: String,
    cards: Vec<String>,
}

#[derive(Deserialize)]
pub struct PostResponse {
    pub id: String,
}

pub async fn create(name: String, cards: Vec<Card>) -> Result<PostResponse> {
    let url = "/api/package";

    let card_ids: Vec<String> = cards.iter().map(|card| card.id.clone()).collect();

    let body = PostBody {
        name,
        cards: card_ids,
    };

    let response = Request::post(url).json(&body)?.send().await?;

    Ok(response.json().await?)
}

pub async fn list() -> Result<Vec<Package>> {
    let response = Request::get("/api/packages").send().await?;

    Ok(response.json().await?)
}

#[derive(Serialize)]
struct PatchBody {
    name: String,
    cards: Vec<String>,
}

pub async fn update(id: String, name: String, cards: Vec<Card>) -> Result<()> {
    let url = format!("/api/package/{id}");

    let card_ids: Vec<String> = cards.iter().map(|card| card.id.clone()).collect();

    let body = PatchBody {
        name,
        cards: card_ids,
    };

    let _ = Request::patch(&url).json(&body)?.send().await?;

    Ok(())
}
