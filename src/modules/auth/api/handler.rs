use std::{collections::HashMap, sync::Arc};

use actix_web::{http::header::LOCATION, web, HttpResponse, Responder, ResponseError};

use crate::modules::auth::AppService;

pub async fn login(
    app_service: web::Data<Arc<AppService>>,
    provider_name: web::Path<String>,
) -> impl Responder {
    match app_service
        .initiate_oauth(get_provider_id(&provider_name.to_string()))
        .await
    {
        Ok(auth_url) => HttpResponse::Found()
            .append_header((LOCATION, auth_url))
            .finish(),
        Err(e) => e.error_response(),
    }
}

pub async fn oauth_callback(
    app_service: web::Data<Arc<AppService>>,
    provider_name: web::Path<String>,
    query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    if let Some(code) = query.get("code") {
        match app_service
            .oauth_login(
                code.to_string(),
                get_provider_id(&provider_name.to_string()),
            )
            .await
        {
            Ok(jwt) => HttpResponse::Ok().json(jwt),
            Err(e) => e.error_response(),
        }
    } else {
        HttpResponse::BadRequest().body("Missing authorization code.")
    }
}

fn get_provider_id(provider_name: &str) -> i32 {
    match provider_name {
        "google" => 1,
        _ => 0,
    }
}
