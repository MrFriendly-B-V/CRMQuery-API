use crate::endpoints::Session;
use crate::error::Result;
use crate::espocrm::contact::{get_contacts, Contact};
use crate::AppData;
use paperclip::actix::{api_v2_operation, web, Apiv2Schema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Apiv2Schema)]
pub struct Request {
    products: Option<String>,
    relation_type: Option<String>,
    location_type: Option<String>,
    province: Option<String>,
    city: Option<String>,
    contact_role: Option<String>,
}

#[derive(Serialize, Apiv2Schema)]
pub struct ResponseData {
    result: Vec<ResultData>,
}

#[derive(Serialize, Apiv2Schema)]
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

#[api_v2_operation]
pub async fn query(data: web::Data<AppData>, payload: web::Json<Request>, _session: Session) -> Result<web::Json<ResponseData>> {
    let runtime = tokio::runtime::Runtime::new().expect("Unable to create Tokio runtime");
    let _guard = runtime.enter();

    let accounts = crate::espocrm::account::get_accounts(
        &data,
        payload.products.clone(),
        payload.relation_type.clone(),
        payload.location_type.clone(),
        payload.province.clone(),
    )
    .await?;

    let mut results: Vec<ResultData> = Vec::new();

    'account_loop: for account in accounts {
        if let (Some(city), Some(shipping_city)) = (&payload.city, &account.shipping_address_city) {
            let shipping_city = shipping_city.to_lowercase();
            let city = city.to_lowercase();

            let mut r#match = false;
            for city in city.split(',').collect::<Vec<&str>>() {
                if city == shipping_city {
                    r#match = true;
                }
            }

            if !r#match {
                continue 'account_loop;
            }
        }

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

    runtime.shutdown_background();
    Ok(web::Json(ResponseData { result: results }))
}
