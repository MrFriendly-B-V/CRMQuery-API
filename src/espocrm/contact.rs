use crate::appdata::AppData;
use reqwest::Method;
use espocrm_rs::{Where, FilterType, Params, Order, Value, NoGeneric};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Response {
    list: Vec<Contact>
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub email_address: String,
    pub phone_number: String,
}

pub async fn get_contacts(appdata: &AppData, account_id: Option<String>, contact_roles: Option<String>) -> Result<Vec<Contact>, String> {
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
        .set_offset(0)
        .set_order(Order::Desc)
        .set_where(where_filter)
        .build();

    let result = appdata.espo_client.request::<NoGeneric>(Method::GET, "Contact".to_string(), Some(params), None).await;
    if result.is_err() {
        Err(result.err().unwrap().to_string())
    } else {
        let response: Response = result.unwrap().json().await.unwrap();
        Ok(response.list)
    }
}