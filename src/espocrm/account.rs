use crate::appdata::AppData;
use espocrm_rs::{Where, FilterType, Value, Params, Order, Method};
use serde::Deserialize;
use crate::error::Result;

#[derive(Deserialize)]
struct Response {
    list: Vec<AccountData>
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    pub id:                     String,
    pub name:                   String,
    pub shipping_address_city:  Option<String>,
    pub shipping_address_state: Option<String>,
    pub created_at:             Option<String>,
    pub producten:              Option<Vec<String>>,
    pub relatie_type:           Option<Vec<String>>,
    pub email_address:          Option<String>
}

pub async fn get_accounts(appdata: &AppData, product: Option<String>, account_type: Option<String>, location_type: Option<String>, province: Option<String>) -> Result<Vec<AccountData>> {
    let mut filter = Vec::new();

    filter.push(Where {
        r#type: FilterType::IsFalse,
        attribute: "exrelatie".to_string(),
        value: None
    });

    filter.push(Where {
        r#type: FilterType::IsFalse,
        attribute: "excludeFromMailing".to_string(),
        value: None
    });

    if product.is_some() {
        filter.push(Where {
            r#type: FilterType::ArrayAnyOf,
            attribute: "producten".to_string(),
            value: Some(Value::String(product))
        });
    }

    if account_type.is_some() {
        filter.push(Where {
            r#type: FilterType::ArrayAnyOf,
            attribute: "relatieType".to_string(),
            value: Some(Value::String(account_type))
        });
    }

    if location_type.is_some() {
        filter.push(Where {
            r#type: FilterType::ArrayAnyOf,
            attribute: "soort".to_string(),
            value: Some(Value::String(location_type))
        });
    }

    if province.is_some() {
        filter.push(Where {
            r#type: FilterType::In,
            attribute: "shippingAddressState".to_string(),
            value: Some(Value::String(province))
        });

        filter.push(Where {
            r#type: FilterType::IsNotNull,
            attribute: "shippingAddressState".to_string(),
            value: None
        });
    }

    let params = Params::new()
        .set_offset(0)
        .set_where(filter)
        .set_order_by("createdAt")
        .set_order(Order::Desc)
        .set_select("id,producten,shippingAddressCity,shippingAddressState,relatieType,name,emailAddress")
        .build();

    let response: Response = appdata.espo_client.request::<(), &str>(Method::Get, "Account", Some(params), None)
        .await?
        .json()
        .await?;

    Ok(response.list)
}