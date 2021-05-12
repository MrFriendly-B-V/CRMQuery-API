use actix_web::{web, get, HttpRequest, HttpResponse};
use crate::AppData;

struct EspoAccountResponse {
    list: Vec<Account>
}

struct Account {

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
        return HttpResponse::InternalServerError().finish();
    }

    let accounts_unwrapped = accounts.unwrap();
    println!("{:?}", accounts_unwrapped.text().await.unwrap());

    runtime.shutdown_background();

    HttpResponse::Ok().finish()
}

fn str_to_string_option(input: Option<&str>) -> Option<String> {
    return if input.is_none() {
        None
    } else {
        Some(input.unwrap().to_string())
    };
}
