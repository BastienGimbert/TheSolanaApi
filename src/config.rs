use std::env;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Settings {
    pub bind_address: String,
    pub validators_csv: PathBuf,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("validators csv file not found at {0}")]
    MissingValidatorsCsv(String),
}

impl Settings {
    pub fn from_env() -> Result<Self, ConfigError> {
        let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0:80".to_string());
        let csv_path =
            env::var("VALIDATORS_CSV").unwrap_or_else(|_| "config/validators.csv".to_string());
        let validators_csv = PathBuf::from(csv_path.clone());

        if !validators_csv.exists() {
            return Err(ConfigError::MissingValidatorsCsv(csv_path));
        }

        Ok(Self {
            bind_address,
            validators_csv,
        })
    }
}
