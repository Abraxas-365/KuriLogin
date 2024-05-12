use actix_web::web;

use super::handler::get_user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/me").route("/", web::get().to(get_user)));
}
