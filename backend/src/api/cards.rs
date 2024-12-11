use actix_web::{
    get,
    web::{Data, Json, Query},
    Result,
};

use common::card::Card;
use serde::Deserialize;

use crate::{
    repository::surrealdb::SurrealDbRepository, services::share_code::decode_share_code_string,
};

#[derive(Deserialize)]
struct Filters {
    selectable_only: Option<String>,
    ids: Option<Vec<String>>,
    share_code: Option<String>,
}

#[get("/cards")]
pub async fn get_cards(
    db: Data<SurrealDbRepository>,
    filters: Query<Filters>,
) -> Result<Json<Vec<Card>>> {
    let from_share_code = filters.share_code.is_some();

    let selectable_only = filters.selectable_only.is_some();

    let ids = filters.ids.clone().unwrap_or_default();

    let result = if ids.is_empty() {
        if selectable_only {
            db.get_all_selectable_cards().await
        } else if from_share_code {
            let codes = decode_share_code_string(filters.share_code.clone().unwrap()).unwrap(); // TODO fix

            db.get_cards_from_share_codes(codes).await
        } else {
            db.get_all_cards().await
        }
    } else {
        db.get_cards(ids).await
    };

    Ok(Json(result))
}
