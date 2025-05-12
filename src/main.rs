use actix_cors::Cors;
use actix_web::{App, HttpServer};

use config::ClankerConfig;
use dotenv::dotenv;
use routes::clanker_route::{health_check, hello_clanker, update_pool};

pub mod config;
pub mod routes;
pub mod services;
pub mod tx_utils;
pub mod update;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = ClankerConfig::get_config();

    println!("Clanker started on port: {}", config.port);

    return HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["POST"])
                    .allowed_headers(vec!["Content-Type"]),
            )
            .service(update_pool)
            .service(hello_clanker)
            .service(health_check)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await;
}

// cargo watch -c -w src -x run
