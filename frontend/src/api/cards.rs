use anyhow::Result;
use common::card::Card;
use gloo_net::http::Request;

pub async fn get() -> Result<Vec<Card>> {
    let url = "/api/cards";

    let response = Request::get(url).send().await?;

    Ok(response.json().await?)
}

pub async fn get_selectable() -> Result<Vec<Card>> {
    let url = "/api/cards?selectable_only";

    let response = Request::get(url).send().await?;

    Ok(response.json().await?)
}

pub async fn get_from_share_code(code: String) -> Result<Vec<Card>> {
    let url = format!("/api/cards?share_code={}", code);

    let response = Request::get(&url).send().await?;

    Ok(response.json().await?)
}

pub async fn update_all_cards() -> Result<()> {
    let url = "/api/cards/update";

    let _ = Request::post(&url).send().await?;

    Ok(())
}
