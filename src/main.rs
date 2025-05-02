use actix_cors::Cors;
use actix_web::{App, HttpServer};

use config::ClankerConfig;
use dotenv::dotenv;
use routes::clanker_route::update_pool;

pub mod config;
pub mod routes;
pub mod services;
pub mod tx_utils;
pub mod update;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = ClankerConfig::get_config();

    return HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec!["Content-Type"]),
            )
            .service(update_pool)
    })
    .bind(("localhost", config.port))?
    .run()
    .await;
}

// cargo watch -c -w src -x run
