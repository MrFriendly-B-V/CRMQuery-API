pub mod query;

use paperclip::actix::Apiv2Security;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Data;
use futures_util::future::{err, ok, Ready};
use serde::Serialize;
use crate::AppData;

#[derive(Serialize)]
pub struct GenericResponse<T> {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data:   Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error:  Option<HttpError>
}

#[derive(Serialize)]
pub struct HttpError {
    pub location:   &'static str,
    pub message:    String
}

#[derive(Apiv2Security)]
#[openapi(apiKey, in = "header", name = "Authorization")]
pub struct Session;

impl FromRequest for Session {
    type Error = crate::error::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let runtime = tokio::runtime::Runtime::new().expect("Unable to create Tokio runtime");
        let _guard = runtime.enter();

        let data: &Data<AppData> = req.app_data().unwrap();
        match runtime.block_on(authlander_client::check_session(req.clone(), &data.config.authlander_host, vec!["mrfriendly.crmquery.query"])) {
            Ok(_) => {},
            Err(authlander_client::Error::InternalError) => return err(Self::Error::Authlander),
            _ => return err(Self::Error::Unauthorized),
        };

        ok(Self)
    }
    
}