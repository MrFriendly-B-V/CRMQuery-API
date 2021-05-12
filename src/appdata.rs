use serde::{Serialize, Deserialize};
use espocrm_rs::EspoApiClient;

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
    pub fn parse() -> Self {
        let use_env = std::env::var("USE_ENVIRONMENTAL_VARIABLES");
        return if use_env.is_err() {
            println!("Environmental variable 'USE_ENVIRONMENTAL_VARIABLES' is not set or not valid unicode. Using the configuration file.");
            Config::parse_from_file()
        } else {
            println!("Environmental variable 'USE_ENVIRONMENTAL_VARIABLES' is set. Using environmental variables for configuration.");
            Config::parse_from_env()
        };
    }

    fn parse_from_file() -> Self {
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
            fs::create_dir_all(path.as_path()).expect("Unable to create Configuration directory.");
        }

        path.push("config.yml");
        if !path.exists() {
            println!("Configuration file did not exist and has been created. Please configure CRMQuery before restarting. You can find the configuration file at '{}'", path.as_path().to_str().unwrap());
            fs::write(path.as_path(), serde_yaml::to_string(&Config::default()).unwrap()).expect("Unable to write Configuration file.");

            std::process::exit(0);
        } else {
            println!("Configuration file at '{}' exists.", path.as_path().to_str().unwrap());
            let config_contents = fs::read_to_string(path.as_path()).expect("Unable to read Configuration file.");

            return serde_yaml::from_str(&config_contents).expect("Unable to parse configuration file. Is your syntax valid?");
        }
    }

    fn parse_from_env() -> Self {
        use std::env;
        use std::process;

        let espo_url = env::var("ESPO_URL");
        if espo_url.is_err() {
            eprintln!("Environmental variable 'ESPO_URL' is not set. Exiting");
            process::exit(1);
        }

        let api_key = env::var("API_KEY");
        if api_key.is_err() {
            eprintln!("Environmental variable 'API_KEY' is not set. Exiting");
            process::exit(1);
        }

        let secret_key = env::var("SECRET_KEY");
        if secret_key.is_err() {
            eprintln!("Environmental variable 'SECRET_KEY' is not set. Not using HMAC Authentication. It is highly recommended that you use this!");
        }

        Config {
            espo_url: espo_url.unwrap(),
            api_key: api_key.unwrap(),
            secret_key: if secret_key.is_err() { None } else { Some(secret_key.unwrap()) }
        }
    }
}