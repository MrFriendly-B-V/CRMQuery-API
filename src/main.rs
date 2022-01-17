mod espocrm;
mod appdata;
mod endpoints;
mod error;

use actix_web::{HttpServer, App};
use crate::appdata::{AppData, Config};
use espocrm_rs::EspoApiClient;
use log::{error, info, debug};
use paperclip::actix::{OpenApiExt, web};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "INFO");
    }
    env_logger::init();

    info!("Starting CRMQuery");
    debug!("Reading configuration...");
    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create Config instance: {}", e);
            std::process::exit(1);
        }
    };

    let espo_client = EspoApiClient::new(&config.espo_url)
        .set_api_key(&config.espo_api_key)
        .set_secret_key(&config.secret_key)
        .build();

    let appdata = AppData {
        config,
        espo_client
    };

    info!("Startup complete.");

    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();

        App::new()
            .wrap_api()
            .data(appdata.clone())
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::NormalizePath::new(actix_web::middleware::normalize::TrailingSlash::Trim))
            .route("/query", web::post().to(crate::endpoints::query::query))
            .with_json_spec_at("/spec")
            .build()
    })
    .bind("[::]:8080")?
    .run()
    .await

}
