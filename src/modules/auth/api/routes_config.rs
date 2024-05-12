use actix_web::web;

use super::handler::{login, oauth_callback};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::get().to(login))
            .route("/callback", web::get().to(oauth_callback)),
    );
}
