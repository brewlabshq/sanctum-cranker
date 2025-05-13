use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;

use crate::services;

#[derive(Deserialize)]
struct UpdatePoolRequest {
    pool: String,
}

#[get("/")]
async fn hello_cranker() -> HttpResponse {
    HttpResponse::Ok().body("Cranker service is running")
}

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Cranker service is healthy")
}

#[post("/cranker/update")]
async fn update_pool(body: web::Json<UpdatePoolRequest>) -> HttpResponse {
    match services::cranker_service::update(&body.pool).await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(e) => {
            print!("Failed to update pool: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update pool")
        }
    }
}
