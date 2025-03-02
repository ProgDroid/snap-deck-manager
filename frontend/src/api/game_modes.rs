use anyhow::Result;
use common::game_mode::GameMode;
use gloo_net::http::Request;

pub async fn list() -> Result<Vec<GameMode>> {
    let url = "/api/game-modes";

    let response = Request::get(url).send().await?;

    Ok(response.json().await?)
}
