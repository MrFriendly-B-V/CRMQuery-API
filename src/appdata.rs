use serde::{Serialize, Deserialize};
use espocrm_rs::EspoApiClient;
use crate::result::Result;
use crate::error;

#[derive(Clone)]
pub struct AppData {
    pub config:         Config,
    pub espo_client:    EspoApiClient
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub espo_url:           String,
    pub espo_api_key:       String,
    pub secret_key:         String,
    pub authlander_host:    String,
}

impl Config {
    pub fn new() -> Result<Self> {
        match envy::from_env::<Self>() {
            Ok(c) => Ok(c),
            Err(e) => Err(error!(e, "Failed to create Config instance from environmental variables"))
        }
    }
}