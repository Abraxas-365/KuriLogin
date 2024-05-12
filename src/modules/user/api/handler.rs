use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::modules::user::AppService;

pub async fn get_user(
    app_service: web::Data<Arc<AppService>>,
    token: web::Path<String>,
) -> impl Responder {
    match app_service.get_user_by_id(&token.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => e.error_response(),
    }
}
