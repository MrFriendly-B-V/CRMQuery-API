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
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();
    info!("Starting CRMQuery");
    debug!("Reading configuration...");
    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            log_error!("Failed to create Config instance: {}", e);
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
            .data(appdata.clone())
            .service(crate::endpoints::query::get)
            .wrap(cors)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

}
