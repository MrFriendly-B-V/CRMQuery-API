use crate::appdata::AppData;
use reqwest::Method;
use std::future::Future;
use espocrm_rs::{Where, FilterType, Params, Order, Value, NoGeneric};

pub async fn get_contacts(appdata: &AppData, account_id: Option<String>, contact_roles: Option<String>) -> impl Future<Output=reqwest::Result<reqwest::Response>> + '_{
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
            attribute: "account".to_string(),
            value: Some(Value::string(account_id.unwrap()))
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

    appdata.espo_client.request::<NoGeneric>(Method::GET, "Contact".to_string(), Some(params), None)
}