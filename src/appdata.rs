use crate::error::Result;
use espocrm_rs::EspoApiClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AppData {
    pub config: Config,
    pub espo_client: EspoApiClient,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub espo_url: String,
    pub espo_api_key: String,
    pub secret_key: String,
    pub authlander_host: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        Ok(envy::from_env::<Self>()?)
    }
}
