use actix_web::web;
use crate::endpoints::Session;
use crate::error::Result;
use crate::espocrm::contact::{get_contacts, Contact};
use crate::AppData;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument, trace};

#[derive(Debug, Deserialize)]
pub struct Request {
    products: Option<String>,
    relation_type: Option<String>,
    location_type: Option<String>,
    province: Option<String>,
    city: Option<String>,
    contact_role: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ResponseData {
    result: Vec<ResultData>,
}

#[derive(Debug, Serialize)]
pub struct ResultData {
    account_id: String,
    account_products: Option<Vec<String>>,
    contacts: Vec<Contact>,
    account_type: Option<Vec<String>>,
    account_name: String,
    account_email: Option<String>,
    shipping_address_city: Option<String>,
    shipping_address_state: Option<String>,
}

#[instrument(skip(data))]
pub async fn query(data: web::Data<AppData>, payload: web::Json<Request>, _: Session) -> Result<web::Json<ResponseData>> {
    debug!("Fetching accounts from EspoCRM");
    let accounts = crate::espocrm::account::get_accounts(
        &data,
        payload.products.clone(),
        payload.relation_type.clone(),
        payload.location_type.clone(),
        payload.province.clone(),
    )
    .await?;

    let mut results: Vec<ResultData> = Vec::new();

    debug!("Iterating over accounts to fetch associated data");
    'account_loop: for account in accounts {
        if let (Some(city), Some(shipping_city)) = (&payload.city, &account.shipping_address_city) {
            trace!("Account {} has a a city and shipping address", &account.name);

            let shipping_city = shipping_city.to_lowercase();
            let city = city.to_lowercase();

            let mut r#match = false;
            for city in city.split(',').collect::<Vec<&str>>() {
                if city == shipping_city {
                    r#match = true;
                }
            }

            if !r#match {
                trace!("Account {} does not match the requirments", account.name);
                continue 'account_loop;
            }
        }

        debug!("Fetching contacts for account {}", account.id);
        let contacts = get_contacts(&data, Some(account.id.clone()), payload.contact_role.clone()).await?;

        let result = ResultData {
            account_id: account.id.clone(),
            account_products: account.producten,
            contacts,
            account_type: account.relatie_type,
            account_name: account.name,
            account_email: account.email_address,
            shipping_address_city: account.shipping_address_city,
            shipping_address_state: account.shipping_address_state,
        };

        results.push(result);
    }

    Ok(web::Json(ResponseData { result: results }))
}
