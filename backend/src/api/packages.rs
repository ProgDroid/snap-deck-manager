use actix_web::{
    get, patch, post,
    web::{Data, Json, Path},
    Result,
};
use serde::{Deserialize, Serialize};

use common::{card::Card, package::Package};

use crate::repository::surrealdb::SurrealDbRepository;

#[get("/package/{package_id}")]
pub async fn get(
    package_identifier: Path<String>,
    db: Data<SurrealDbRepository>,
) -> Result<Json<Package>> {
    let package = db.get_package(package_identifier.into_inner()).await;

    Ok(Json(package.expect("Could not find package")))
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

#[post("/package")]
pub async fn create(
    db: Data<SurrealDbRepository>,
    body: Json<PostData>,
) -> Result<Json<PostResponse>> {
    let id = db
        .create_package(Package::new(body.name.clone(), body.cards.clone()))
        .await;

    let id = id.expect("Could not create package");

    let post_response = PostResponse { id: id.id.to_raw() };

    Ok(Json(post_response))
}

#[patch("/package/{package_id}")]
pub async fn update(
    package_id: Path<String>,
    db: Data<SurrealDbRepository>,
    body: Json<PostData>,
) -> Result<Json<PostResponse>> {
    let mut package = Package::new(body.name.clone(), body.cards.clone());
    package.id = Some(package_id.clone());

    let id = db.update_package(package).await;

    id.expect("Could not update package");

    let post_response = PostResponse {
        id: package_id.into_inner(),
    };

    Ok(Json(post_response))
}

#[get("/packages")]
pub async fn list(db: Data<SurrealDbRepository>) -> Result<Json<Vec<Package>>> {
    let packages = db.get_all_packages().await;

    Ok(Json(packages))
}
