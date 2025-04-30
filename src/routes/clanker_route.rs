use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

use crate::services;

#[derive(Deserialize)]
struct UpdatePoolRequest {
    pool: String,
}

#[post("/clanker/update")]
async fn update_pool(body: web::Json<UpdatePoolRequest>) -> HttpResponse {
    match services::clanker_service::update(&body.pool).await {
        Ok(_) => HttpResponse::Ok().body("Success"),
        Err(e) => {
            print!("Failed to update pool: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update pool")
        }
    }
}
