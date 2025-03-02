use actix_web::{
    get,
    web::{Data, Json},
    Result,
};

use common::game_mode::GameMode;

use crate::repository::surrealdb::SurrealDbRepository;

#[get("/game-modes")]
pub async fn get_game_modes(db: Data<SurrealDbRepository>) -> Result<Json<Vec<GameMode>>> {
    let result = db.get_all_game_modes().await;

    Ok(Json(result))
}
