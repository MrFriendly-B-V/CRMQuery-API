pub mod query;
use serde::Serialize;

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

#[macro_export]
macro_rules! respond {
    ($error_location:expr, $error_message:expr) => {
        {
            use $crate::endpoints::{HttpError, GenericResponse};

            let error = HttpError { location: $error_location, message: $error_message.to_string()};
            let response = GenericResponse::<()> { data: None, error: Some(error) };
            let as_json = serde_json::to_string(&response).unwrap();
            as_json
        }
    };
    ($data:expr) => {
        {
            use $crate::endpoints::GenericResponse;
            let response = GenericResponse { data: Some($data), error: None };
            let as_json = serde_json::to_string(&response).unwrap();
            as_json
        }
    }
}