use actix_web::{
    get,
    web::{Data, Json, Query},
    Result,
};

use common::card::Card;
use serde::Deserialize;

#[derive(Deserialize)]
struct Filters {
    selectable_only: Option<String>,
}

#[get("/cards")]
pub async fn get_cards(
    cards_data: Data<Vec<Card>>,
    filters: Query<Filters>,
) -> Result<Json<Vec<Card>>> {
    Ok(Json(if filters.selectable_only.is_some() {
        cards_data
            .into_inner()
            .iter()
            .filter(|&card| !card.is_token && card.source != "None" && !card.source.is_empty())
            .cloned()
            .collect()
    } else {
        cards_data.into_inner().to_vec()
    }))
}

// TODO work out how to actually get unreleased cards that aren't tokens or from locations
// * seems like I need to parse SeriesStartDates and check if any entries exist for it to be a selectable non-token
// * seems like I need to parse SeriesStartDates and check the first entry is in the future
// * https://stackoverflow.com/questions/71876855/idiomatic-way-to-check-if-a-chronodatetimeutc-is-within-date-and-time-range
