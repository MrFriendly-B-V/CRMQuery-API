use actix_web::{web, get, HttpRequest, HttpResponse};
use crate::AppData;
use crate::espocrm::contact::{get_contacts, Contact};
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;

#[derive(Serialize)]
pub struct Response {
    result: Vec<ResultData>
}

#[derive(Serialize)]
pub struct ResultData {
    account_id: String,
    account_products: Vec<String>,
    contacts: Vec<Contact>,
    account_type: Vec<String>,
    account_name: String,
    shipping_address_city: String,
    shipping_address_state: String
}

#[get("/get")]
pub async fn get(req: HttpRequest, appdata: web::Data<AppData>) -> HttpResponse {
    let qstring = qstring::QString::from(req.query_string());

    let products = str_to_string_option(qstring.get("products"));
    let relation_type = str_to_string_option(qstring.get("relation_type"));
    let location_type = str_to_string_option(qstring.get("relation_type"));
    let province = str_to_string_option(qstring.get("province"));
    let city = str_to_string_option(qstring.get("city"));
    let contact_roll = str_to_string_option(qstring.get("contact_roll"));
    let return_values = str_to_string_option(qstring.get("return_values"));

    if return_values.is_none() {
        return HttpResponse::BadRequest().finish();
    }

    let runtime = tokio::runtime::Runtime::new().expect("Unable to create Tokio runtime");
    let _guard = runtime.enter();

    let accounts = crate::espocrm::account::get_accounts(&appdata, products, relation_type, location_type, province).await;
    if accounts.is_err()  {
        return HttpResponse::InternalServerError().body(accounts.err().unwrap());
    }

    let accounts_unwrapped = accounts.unwrap();

    let mut results: Vec<ResultData> = Vec::new();
    let mut processors: Vec<Box<dyn Future<Output=ResultData>>> = Vec::new();
    'account_loop: for account in accounts_unwrapped {
        if city.is_some() {
            let city_lower = account.shipping_address_city.to_lowercase();

            let mut r#match = false;
            for city in city.clone().unwrap().split(",").collect::<Vec<&str>>() {
                if city == &city_lower {
                    r#match = true;
                }
            }

            if !r#match {
                continue 'account_loop;
            }
        }

        let contacts = get_contacts(&appdata, Some(account.id.clone()), contact_roll.clone());

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

fn str_to_string_option(input: Option<&str>) -> Option<String> {
    return if input.is_none() {
        None
    } else {
        Some(input.unwrap().to_string())
    };
}
