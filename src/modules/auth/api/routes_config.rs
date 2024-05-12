use actix_web::web;

use super::handler::{login, oauth_callback};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/google/login", web::get().to(login))
            .route("/google/callback", web::get().to(oauth_callback)),
    );
}
