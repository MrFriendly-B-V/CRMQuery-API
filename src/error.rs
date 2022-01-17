use actix_web::http::StatusCode;
use actix_web::ResponseError;
use paperclip::actix::api_v2_errors;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[api_v2_errors]
pub enum Error {
    #[error("Reqwest {0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Envy {0:?}")]
    Envy(#[from] envy::Error),
    #[error("Ser/Der Json {0:?}")]
    Json(#[from] serde_json::Error),
    #[error("IO Error {0:?}")]
    Io(#[from] std::io::Error),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Authlander error")]
    Authlander,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Reqwest(_) | Self::Envy(_) | Self::Json(_) | Self::Io(_) | Self::Authlander => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}
