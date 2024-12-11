mod api;
mod config;
mod entities;
mod repository;
mod services;

use api::{
    cards::get_cards,
    decks::{create as create_deck, get as get_deck, list as get_decks, update as update_deck},
    packages::{
        create as create_package, get as get_package, list as get_packages,
        update as update_package,
    },
};
use config::{api::Api as ApiConfig, db::Db as DbConfig};
use repository::surrealdb::SurrealDbRepository;

use actix_web::{
    middleware::Logger,
    web::{scope, Data},
    App, HttpServer,
};
use actix_web_lab::web::spa;
use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let db_config = DbConfig::new()?;

    let repository = SurrealDbRepository::new(db_config).await?;
    repository.update_all_cards().await?;

    let repository_data = Data::new(repository);

    let api_config = ApiConfig::new()?;

    // * This closure executes every time a new thread is spun up to handle a request
    Ok(HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(repository_data.clone())
            .service(
                scope("/api")
                    .service(get_cards)
                    .service(get_deck)
                    .service(create_deck)
                    .service(get_decks)
                    .service(update_deck)
                    .service(get_package)
                    .service(create_package)
                    .service(get_packages)
                    .service(update_package),
            )
            .service(
                spa()
                    .index_file("dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("dist")
                    .finish(),
            )
    })
    .bind(api_config.to_address())?
    .run()
    .await?)
}
