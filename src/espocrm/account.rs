use crate::appdata::AppData;
use crate::error::Result;
use espocrm_rs::{FilterType, Method, Order, Params, Value, Where};
use serde::Deserialize;
use tap::TapFallible;
use tracing::{instrument, trace, warn};

#[derive(Debug, Deserialize)]
struct Response {
    list: Vec<AccountData>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    pub id: String,
    pub name: String,
    pub shipping_address_city: Option<String>,
    pub shipping_address_state: Option<String>,
    pub created_at: Option<String>,
    pub producten: Option<Vec<String>>,
    pub relatie_type: Option<Vec<String>>,
    pub email_address: Option<String>,
}

#[instrument(skip(appdata))]
pub async fn get_accounts(
    appdata: &AppData,
    product: Option<String>,
    account_type: Option<String>,
    location_type: Option<String>,
    province: Option<String>,
) -> Result<Vec<AccountData>> {
    let mut filter = vec![
        Where {
            r#type: FilterType::IsFalse,
            attribute: "exrelatie".to_string(),
            value: None,
        },
        Where {
            r#type: FilterType::IsFalse,
            attribute: "excludeFromMailing".to_string(),
            value: None,
        },
    ];

    if product.is_some() {
        filter.push(Where {
            r#type: FilterType::ArrayAnyOf,
            attribute: "producten".to_string(),
            value: Some(Value::String(product)),
        });
    }

    if account_type.is_some() {
        filter.push(Where {
            r#type: FilterType::ArrayAnyOf,
            attribute: "relatieType".to_string(),
            value: Some(Value::String(account_type)),
        });
    }

    if location_type.is_some() {
        filter.push(Where {
            r#type: FilterType::ArrayAnyOf,
            attribute: "soort".to_string(),
            value: Some(Value::String(location_type)),
        });
    }

    if province.is_some() {
        filter.push(Where {
            r#type: FilterType::In,
            attribute: "shippingAddressState".to_string(),
            value: Some(Value::String(province)),
        });

        filter.push(Where {
            r#type: FilterType::IsNotNull,
            attribute: "shippingAddressState".to_string(),
            value: None,
        });
    }

    let params = Params::new()
        .set_offset(0)
        .set_where(filter)
        .set_order_by("createdAt")
        .set_order(Order::Desc)
        .set_select("id,producten,shippingAddressCity,shippingAddressState,relatieType,name,emailAddress")
        .build();

    trace!("Fetching accounts with the following filter: {params:?}");
    let response_text = appdata
        .espo_client
        .request::<(), &str>(Method::Get, "Account", Some(params), None)
        .await?
        .text()
        .await?;

    let response: Response = serde_json::from_str(&response_text)
        .tap_err(|e| warn!("Unable to deserialize response from EspoCRM: {e:?}. The response was: {response_text}"))?;

    Ok(response.list)
}
