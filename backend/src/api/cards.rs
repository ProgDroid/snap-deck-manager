use actix_web::{
    get,
    web::{Data, Json, Query},
    Result,
};

use common::card::Card;
use serde::Deserialize;

use crate::repository::surrealdb::SurrealDbRepository;

#[derive(Deserialize)]
struct Filters {
    selectable_only: Option<String>,
    ids: Option<Vec<String>>,
}

#[get("/cards")]
pub async fn get_cards(
    db: Data<SurrealDbRepository>,
    filters: Query<Filters>,
) -> Result<Json<Vec<Card>>> {
    let selectable_only = filters.selectable_only.is_some();

    let ids = filters.ids.clone().unwrap_or_default();

    let result = if ids.is_empty() {
        if selectable_only {
            db.get_all_selectable_cards().await
        } else {
            db.get_all_cards().await
        }
    } else {
        db.get_cards(ids).await
    };

    Ok(Json(result))
}
