mod appdata;
mod endpoints;
mod error;
mod espocrm;

use crate::appdata::{AppData, Config};
use actix_web::{App, HttpServer, web};
use actix_web::middleware::TrailingSlash;
use espocrm_rs::EspoApiClient;
use tracing::{debug, error, info};
use tracing_subscriber::layer::SubscriberExt;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    setup_tracing();

    info!("Starting CRMQuery");
    debug!("Reading configuration");
    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to create Config instance: {}", e);
            std::process::exit(1);
        }
    };

    debug!("Creating EspoCRM client");
    let espo_client = EspoApiClient::new(&config.espo_url)
        .set_api_key(&config.espo_api_key)
        .set_secret_key(&config.secret_key)
        .build();

    let appdata = AppData { config, espo_client };
    let appdata = web::Data::new(appdata);

    info!("Startup complete.");

    HttpServer::new(move || App::new()
        .app_data(appdata.clone())
        .wrap(actix_cors::Cors::permissive())
        .wrap(tracing_actix_web::TracingLogger::default())
        .wrap(actix_web::middleware::NormalizePath::new(TrailingSlash::Trim))
        .route("/query", web::post().to(endpoints::query::query))
    )
    .bind("[::]:8080")?
    .run()
    .await
}

fn setup_tracing() {
    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .compact()
        )
        .with(tracing_subscriber::EnvFilter::from_default_env());
    tracing::subscriber::set_global_default(subscriber).expect("Setting global tracing subscriber");
}