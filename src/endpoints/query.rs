use actix_web::{web, post, HttpResponse};
use crate::AppData;
use crate::espocrm::contact::{get_contacts, Contact};
use serde::{Serialize, Deserialize};
use std::future::Future;
use std::pin::Pin;

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
pub struct Response {
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

    let accounts = crate::espocrm::account::get_accounts(&appdata, form.products.clone(), form.relation_type.clone(), form.location_type.clone(), form.province.clone()).await;
    if accounts.is_err()  {
        return HttpResponse::InternalServerError().body(accounts.err().unwrap());
    }

    let accounts_unwrapped = accounts.unwrap();

    let mut results: Vec<ResultData> = Vec::new();
    let mut processors: Vec<Box<dyn Future<Output=ResultData>>> = Vec::new();
    'account_loop: for account in accounts_unwrapped {

        if form.city.is_some() {

            let city_lower = account.shipping_address_city.clone().unwrap().to_lowercase();

            let mut r#match = false;
            for city in form.city.clone().unwrap().split(",").collect::<Vec<&str>>() {
                if city == &city_lower {
                    r#match = true;
                }
            }

            if !r#match {
                continue 'account_loop;
            }
        }

        let contacts = get_contacts(&appdata, Some(account.id.clone()), form.contact_role.clone().clone());
        let account_clone = account.clone();
        let processor = async {
            let data = contacts.await;
            if data.is_err() {

            }

            let result = ResultData {
                account_id: account_clone.id.clone(),
                account_products: account_clone.producten,
                contacts: data.unwrap(),
                account_type: account_clone.relatie_type,
                account_name: account_clone.name,
                account_email: account_clone.email_address;
                shipping_address_city: account_clone.shipping_address_city,
                shipping_address_state: account_clone.shipping_address_state
            };

            result
        };
        processors.push(Box::new(processor));
    }

    for processor in processors {
        let result = Pin::from(processor).await;
        results.push(result);
    }

    runtime.shutdown_background();
    HttpResponse::Ok().json(Response { result: results })
}
