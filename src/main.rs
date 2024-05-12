#![allow(dead_code)]

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{
    modules::{
        auth::{self, infrastructure::GoogleProvider},
        user,
    },
    utils::{config::Config, postgres::PostgresRepository},
};
mod error;
mod modules;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,debug");
    env_logger::init();
    let config = Config::from_env();

    let repo = Arc::new(PostgresRepository::new().await);

    let google_provider = GoogleProvider::new(
        config.google_client_id.clone(),
        config.google_client_secret.clone(),
        config.google_redirect_uri.clone(),
    );

    let jwt_manager = Arc::new(auth::JwtManager::new(config.jwt_secret.clone()));

    let user_service = Arc::new(user::AppService::new(repo.clone(), jwt_manager.clone()));

    let auth_service = Arc::new(auth::AppService::new(
        Arc::new(google_provider),
        repo.clone(),
        user_service.clone(),
        jwt_manager,
    ));

    log::info!("Starting HTTP server on 0.0.0.0:80...");
    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .configure(auth::api::config)
            .configure(user::api::config)
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
