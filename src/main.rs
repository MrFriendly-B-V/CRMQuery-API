mod espocrm;
mod appdata;
mod endpoints;

use actix_web::{HttpServer, App};
use crate::appdata::AppData;
use espocrm_rs::EspoApiClient;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("Welcome to CRMQuery by MrFriendly B.V.");
    println!("Reading configuration...");
    let config = appdata::Config::parse();
    let mut espo_client = EspoApiClient::new(&config.espo_url)
        .set_api_key(&config.api_key).build();

    if config.secret_key.is_some() {
        espo_client.set_secret_key(&config.clone().secret_key.unwrap());
    }

    let appdata = AppData {
        config,
        espo_client: espo_client.build()
    };

    println!("Startup complete.");

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
