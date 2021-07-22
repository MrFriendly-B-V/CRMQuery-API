use crate::appdata::AppData;
use reqwest::Method;
use espocrm_rs::{Where, FilterType, Params, Order, Value};
use serde::{Deserialize, Serialize};

use crate::error;
use crate::result::Result;

#[derive(Deserialize)]
struct Response {
    list: Vec<Contact>
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id:             Option<String>,
    pub last_name:      Option<String>,
    pub first_name:     Option<String>,
    pub email_address:  Option<String>,
    pub phone_number:   Option<String>
}

pub async fn get_contacts(appdata: &AppData, account_id: Option<String>, contact_roles: Option<String>) -> Result<Vec<Contact>> {
    let mut where_filter = Vec::new();

    where_filter.push(Where {
        r#type: FilterType::IsFalse,
        attribute: "emailAddressIsOptedOut".to_string(),
        value: None
    });

    where_filter.push(Where {
        r#type: FilterType::IsFalse,
        attribute: "geenmassmailing".to_string(),
        value: None
    });

    if account_id.is_some() {
        where_filter.push(Where {
            r#type: FilterType::LinkedWith,
            attribute: "accounts".to_string(),
            value: Some(Value::array(vec![Value::string(account_id.unwrap())]))
        });
    }

    if contact_roles.is_some() {
        where_filter.push(Where {
            r#type: FilterType::ArrayAnyOf,
            attribute: "role".to_string(),
            value: Some(Value::string(contact_roles.unwrap()))
        });
    }

    let params = Params::new()
        .set_order_by("createdAt")
        .set_select("firstName,lastName,emailAddress,phoneNumber")
        .set_offset(0)
        .set_order(Order::Desc)
        .set_where(where_filter)
        .build();

    match match appdata.espo_client.request::<()>(Method::GET, "Contact".to_string(), Some(params), None).await {
        Ok(r) => r.json::<Response>().await,
        Err(e) => return Err(error!(e, "Failed to send GET request to EspoCRM"))
    } {
        Ok(d) => Ok(d.list),
        Err(e) => Err(error!(e, "Failed to deserialize response data"))
    }
}