use crate::appdata::AppData;
use crate::error::Result;
use espocrm_rs::{FilterType, Method, Order, Params, Value, Where};
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Apiv2Schema)]
struct Response {
    list: Vec<Contact>,
}

#[derive(Deserialize, Serialize, Apiv2Schema)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id: Option<String>,
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub email_address: Option<String>,
    pub phone_number: Option<String>,
}

pub async fn get_contacts(appdata: &AppData, account_id: Option<String>, contact_roles: Option<String>) -> Result<Vec<Contact>> {
    let mut where_filter = vec![
        Where {
            r#type: FilterType::IsFalse,
            attribute: "emailAddressIsOptedOut".to_string(),
            value: None,
        },
        Where {
            r#type: FilterType::IsFalse,
            attribute: "geenmassmailing".to_string(),
            value: None,
        },
    ];

    if account_id.is_some() {
        where_filter.push(Where {
            r#type: FilterType::LinkedWith,
            attribute: "accounts".to_string(),
            value: Some(Value::array(vec![Value::string(account_id.unwrap())])),
        });
    }

    if contact_roles.is_some() {
        where_filter.push(Where {
            r#type: FilterType::ArrayAnyOf,
            attribute: "role".to_string(),
            value: Some(Value::string(contact_roles.unwrap())),
        });
    }

    let params = Params::new()
        .set_order_by("createdAt")
        .set_select("firstName,lastName,emailAddress,phoneNumber")
        .set_offset(0)
        .set_order(Order::Desc)
        .set_where(where_filter)
        .build();

    let response: Response = appdata
        .espo_client
        .request::<(), &str>(Method::Get, "Contact", Some(params), None)
        .await?
        .json()
        .await?;
    Ok(response.list)
}
