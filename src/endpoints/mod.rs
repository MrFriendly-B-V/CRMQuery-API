pub mod query;

use std::future::Future;
use std::pin::Pin;
use crate::AppData;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use serde::Serialize;
use tracing::warn;

#[derive(Debug, Serialize)]
pub struct GenericResponse<T> {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<HttpError>,
}

#[derive(Debug, Serialize)]
pub struct HttpError {
    pub location: &'static str,
    pub message: String,
}

#[derive(Debug)]
pub struct Session;

impl FromRequest for Session {
    type Error = crate::error::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let data: &Data<AppData> = req.app_data().unwrap();
            let host = data.config.authlander_host.clone();

            match authlander_client::check_session(req, &host, vec!["mrfriendly.crmquery.query"]).await {
                Ok(_) => Ok(Session),
                Err(e) if matches!(&e, authlander_client::Error::InternalError( .. )) => {
                    warn!("Authlander internal error: {e:?}");
                    Err(Self::Error::Authlander)
                },
                _ => Err(Self::Error::Unauthorized),
            }
        })
    }
}
