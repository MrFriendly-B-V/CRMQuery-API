use serde::{Serialize, Deserialize};
use espocrm_rs::EspoApiClient;
use crate::result::Result;
use crate::error;
use log::trace;

#[derive(Clone)]
pub struct AppData {
    pub config:         Config,
    pub espo_client:    EspoApiClient
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub espo_url:       String,
    pub api_key:        String,
    pub secret_key:     Option<String>
}

impl Default for Config {
    fn default() -> Self {
        Self {
            espo_url: "YOUR ESPO URL HERE".to_string(),
            api_key: "YOUR API KEY HERE".to_string(),
            secret_key: Some("YOUR SECRET KEY HERE. LEAVE BLANK IF YOU DO NOT WANT TO USE HMAC".to_string())
        }
    }
}

impl Config {
    pub fn new() -> Result<Self> {
        let use_env = std::env::var("USE_ENVIRONMENTAL_VARIABLES");
        return if use_env.is_err() {
            trace!("Environmental variable 'USE_ENVIRONMENTAL_VARIABLES' is not set or not valid unicode. Using the configuration file.");
            Self::parse_from_file()
        } else {
            trace!("Environmental variable 'USE_ENVIRONMENTAL_VARIABLES' is set. Using environmental variables for configuration.");
            Self::parse_from_env()
        };
    }

    fn parse_from_file() -> Result<Self> {
        use std::fs;
        use std::path;

        let mut path = path::PathBuf::new();
        #[cfg(windows)]
        {
            path.push(r#"C:\Program Files\CRMQuery\"#);
        }

        #[cfg(unix)]
        {
            path.push(r#"/etc/crmquery/"#);
        }

        if !path.exists() {
            match fs::create_dir_all(path.as_path()) {
                Ok(_) => {},
                Err(e) => return Err(error!(e, "Failed to create configuration directory"))
            }
        }

        path.push("config.yml");
        if !path.exists() {
            let default_serialized = match serde_yaml::to_string(&Config::default()) {
                Ok(ds) => ds,
                Err(e) => return Err(error!(e, "Failed to serialize default configuration"))
            };

            match fs::write(path.as_path(), default_serialized) {
                Ok(_) => Ok(Self::default()),
                Err(e) => Err(error!(e, "Failed to write default configuration to file"))
            }
        } else {
            let config_contents = match fs::read_to_string(path.as_path()) {
                Ok(cc) => cc,
                Err(e) => return Err(error!(e, "Failed to read configuration file to String"))
            };

            match serde_yaml::from_str(&config_contents) {
                Ok(c) => Ok(c),
                Err(e) => Err(error!(e, "Failed to deserialize configuration file"))
            }
        }
    }

    fn parse_from_env() -> Result<Self> {
        match envy::from_env::<Self>() {
            Ok(c) => Ok(c),
            Err(e) => Err(error!(e, "Failed to create Config instance from environmental variables"))
        }
    }
}