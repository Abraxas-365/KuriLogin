use actix_web::web;

use super::handler::{login, oauth_callback};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/{provider_name}/login", web::get().to(login))
            .route("/{provider_name}/callback", web::get().to(oauth_callback)),
    );
}
