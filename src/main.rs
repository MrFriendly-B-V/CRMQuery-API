mod espocrm;
mod appdata;
mod endpoints;
mod result;

use actix_web::{HttpServer, App};
use crate::appdata::{AppData, Config};
use espocrm_rs::EspoApiClient;
use log::{error as log_error, info, debug};

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    info!("Welcome to CRMQuery by MrFriendly B.V.");
    debug!("Reading configuration...");
    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            log_error!("Failed to create Config instance: {}", e);
            std::process::exit(1);
        }
    };

    let mut espo_client = EspoApiClient::new(&config.espo_url)
        .set_api_key(&config.api_key).build();

    match &config.secret_key {
        Some(key) => {
            let _ = espo_client.set_secret_key(key);
        },
        None => {}
    }

    let appdata = AppData {
        config,
        espo_client: espo_client.build()
    };

    info!("Startup complete.");

    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();

        App::new()
            .data(appdata.clone())
            .service(crate::endpoints::query::get)
            .wrap(cors)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

}
