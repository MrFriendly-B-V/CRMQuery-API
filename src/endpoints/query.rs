use actix_web::{web, post, HttpResponse};
use crate::AppData;
use crate::espocrm::contact::{get_contacts, Contact};
use serde::{Serialize, Deserialize};

use crate::respond;

#[derive(Deserialize)]
pub struct Request {
    products:       Option<String>,
    relation_type:  Option<String>,
    location_type:  Option<String>,
    province:       Option<String>,
    city:           Option<String>,
    contact_role:   Option<String>,
}

#[derive(Serialize)]
pub struct ResponseData {
    result: Vec<ResultData>
}

#[derive(Serialize)]
pub struct ResultData {
    account_id:             String,
    account_products:       Option<Vec<String>>,
    contacts:               Vec<Contact>,
    account_type:           Option<Vec<String>>,
    account_name:           String,
    account_email:          Option<String>,
    shipping_address_city:  Option<String>,
    shipping_address_state: Option<String>
}

#[post("/query")]
pub async fn get(form: web::Form<Request>, appdata: web::Data<AppData>) -> HttpResponse {
    let runtime = tokio::runtime::Runtime::new().expect("Unable to create Tokio runtime");
    let _guard = runtime.enter();

    let accounts = match crate::espocrm::account::get_accounts(&appdata, form.products.clone(), form.relation_type.clone(), form.location_type.clone(), form.province.clone()).await {
        Ok(ac) => ac,
        Err(e) => {
            log::warn!("Failed to query Accounts from EspoCRM: '{}'", e);
            return HttpResponse::InternalServerError().body(respond!("accounts", "Failed to query Accounts from EspoCRM"));
        }
    };

    let mut results: Vec<ResultData> = Vec::new();

    'account_loop: for account in accounts {
        match (&form.city, &account.shipping_address_city) {
            (Some(city), Some(shipping_city)) => {
                let shipping_city = shipping_city.to_lowercase();
                let city = city.to_lowercase();

                let mut r#match = false;
                for city in city.split(",").collect::<Vec<&str>>() {
                    if city == shipping_city {
                        r#match = true;
                    }
                }

                if !r#match {
                    continue 'account_loop;
                }
            },
            _ => {}
        }

        let contacts = match get_contacts(&appdata, Some(account.id.clone()), form.contact_role.clone()).await {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Failed to query Contacts from EspoCRM for account '{}': {}", &account.id, e);
                return HttpResponse::InternalServerError().body(respond!("contacts", format!("Failed to query Contacts from EspoCRM for account '{}'", &account.id)));
            }
        };

        let result = ResultData {
            account_id: account.id.clone(),
            account_products: account.producten,
            contacts,
            account_type: account.relatie_type,
            account_name: account.name,
            account_email: account.email_address,
            shipping_address_city: account.shipping_address_city,
            shipping_address_state: account.shipping_address_state
        };

        results.push(result);
    }

    runtime.shutdown_background();
    HttpResponse::Ok().body(respond!(ResponseData { result: results }))
}
