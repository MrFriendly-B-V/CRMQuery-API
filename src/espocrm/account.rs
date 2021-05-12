use reqwest::Method;
use crate::appdata::AppData;
use espocrm_rs::{Where, FilterType, Value, Params, Order, NoGeneric};

pub async fn get_accounts(appdata: &AppData, product: Option<String>, account_type: Option<String>, location_type: Option<String>, province: Option<String>) -> reqwest::Result<reqwest::Response> {
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
        .set_select("id,producten,shippingAddressCity,shippingAddressState,relatieType,name")
        .build();

    appdata.espo_client.request::<NoGeneric>(Method::GET, "Account".to_string(), Some(params), None).await
}