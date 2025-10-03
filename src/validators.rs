use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone)]
pub struct Validator {
    name: String,
    location: String,
    rpc_url: Url,
}

impl Validator {
    pub fn new(name: String, location: String, rpc_url: Url) -> Self {
        Self {
            name,
            location,
            rpc_url,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn location(&self) -> &str {
        &self.location
    }

    pub fn rpc_url(&self) -> &Url {
        &self.rpc_url
    }

    pub fn host_header(&self) -> Option<String> {
        let host = self.rpc_url.host()?;
        let mut host = host.to_string();
        if let Some(port) = self.rpc_url.port() {
            host = format!("{}:{}", host, port);
        }
        Some(host)
    }

    pub fn summary(&self) -> ValidatorSummary {
        ValidatorSummary {
            name: self.name.clone(),
            location: self.location.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidatorSummary {
    pub name: String,
    pub location: String,
}

#[derive(Debug, Clone)]
pub struct ValidatorRegistry {
    validators: Vec<Validator>,
    index_by_name: HashMap<String, usize>,
    index_by_location: HashMap<String, Vec<usize>>,
}

impl ValidatorRegistry {
    pub fn from_csv(path: &Path) -> Result<Self, RegistryError> {
        let reader = File::open(path)?;
        Self::from_reader(reader)
    }

    pub fn from_reader<R: Read>(reader: R) -> Result<Self, RegistryError> {
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .trim(csv::Trim::All)
            .from_reader(reader);

        let mut validators = Vec::new();

        for (row_idx, result) in csv_reader.deserialize::<ValidatorCsvRecord>().enumerate() {
            let record = result?;
            let row_number = row_idx + 2; // account for header row
            let validator = Validator::try_from_record(record, row_number, validators.len() + 1)?;
            validators.push(validator);
        }

        Self::new(validators)
    }

    pub fn new(validators: Vec<Validator>) -> Result<Self, RegistryError> {
        if validators.is_empty() {
            return Err(RegistryError::Empty);
        }

        let mut index_by_name = HashMap::with_capacity(validators.len());
        let mut index_by_location: HashMap<String, Vec<usize>> = HashMap::new();

        for (idx, validator) in validators.iter().enumerate() {
            let name_key = normalize_key(validator.name());
            if index_by_name.insert(name_key.clone(), idx).is_some() {
                return Err(RegistryError::DuplicateName(validator.name().to_string()));
            }

            let location_key = normalize_key(validator.location());
            index_by_location.entry(location_key).or_default().push(idx);
        }

        Ok(Self {
            validators,
            index_by_name,
            index_by_location,
        })
    }

    pub fn validators(&self) -> &[Validator] {
        &self.validators
    }

    pub fn summaries(&self) -> Vec<ValidatorSummary> {
        self.validators.iter().map(|v| v.summary()).collect()
    }

    pub fn select(
        &self,
        name: Option<&str>,
        location: Option<&str>,
    ) -> Result<Validator, SelectionError> {
        if let Some(name) = name.and_then(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        }) {
            return self
                .get_by_name(name)
                .ok_or_else(|| SelectionError::UnknownValidator(name.to_string()));
        }

        if let Some(location) = location.and_then(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        }) {
            return self
                .random_in_location(location)
                .ok_or_else(|| SelectionError::UnknownLocation(location.to_string()));
        }

        self.random().ok_or(SelectionError::Empty)
    }

    pub fn get_by_name(&self, name: &str) -> Option<Validator> {
        let key = normalize_key(name);
        self.index_by_name
            .get(&key)
            .map(|idx| self.validators[*idx].clone())
    }

    pub fn random_in_location(&self, location: &str) -> Option<Validator> {
        let key = normalize_key(location);
        let mut rng = rand::thread_rng();
        self.index_by_location.get(&key).and_then(|indexes| {
            indexes
                .choose(&mut rng)
                .map(|idx| self.validators[*idx].clone())
        })
    }

    pub fn random(&self) -> Option<Validator> {
        let mut rng = rand::thread_rng();
        self.validators.choose(&mut rng).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }
}

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("csv error: {0}")]
    Csv(#[from] csv::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid record at row {0}: {1}")]
    InvalidRecord(usize, String),
    #[error("duplicate validator name '{0}'")]
    DuplicateName(String),
    #[error("no validators configured")]
    Empty,
}

#[derive(Debug, Error)]
pub enum SelectionError {
    #[error("validator '{0}' not found")]
    UnknownValidator(String),
    #[error("no validator available for location '{0}'")]
    UnknownLocation(String),
    #[error("no validators available")]
    Empty,
}

#[derive(Debug, Deserialize)]
struct ValidatorCsvRecord {
    #[serde(default)]
    name: Option<String>,

    #[serde(default, alias = "rpc_url")]
    rpc_endpoint: Option<String>,

    #[serde(default, alias = "ip", alias = "host", alias = "address")]
    endpoint_host: Option<String>,

    #[serde(default, alias = "rpc_port")]
    endpoint_port: Option<u16>,

    #[serde(default)]
    protocol: Option<String>,

    #[serde(default)]
    location: Option<String>,
}

impl Validator {
    fn try_from_record(
        record: ValidatorCsvRecord,
        row_number: usize,
        ordinal: usize,
    ) -> Result<Self, RegistryError> {
        let location = record
            .location
            .unwrap_or_else(|| "unspecified".to_string())
            .trim()
            .to_string();

        let protocol = record
            .protocol
            .unwrap_or_else(default_protocol)
            .trim()
            .to_ascii_lowercase();

        if protocol != "http" && protocol != "https" {
            return Err(RegistryError::InvalidRecord(
                row_number,
                format!("unsupported protocol '{protocol}'"),
            ));
        }

        let mut url = if let Some(endpoint) = record
            .rpc_endpoint
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            Url::parse(endpoint).map_err(|err| {
                RegistryError::InvalidRecord(row_number, format!("invalid url '{endpoint}': {err}"))
            })?
        } else if let Some(host) = record
            .endpoint_host
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            let formatted_host = prepare_host_for_url(host);
            let candidate = format!("{protocol}://{formatted_host}");
            let mut parsed = Url::parse(&candidate).map_err(|err| {
                RegistryError::InvalidRecord(row_number, format!("invalid host '{host}': {err}"))
            })?;

            if parsed.port().is_none() {
                let port = record.endpoint_port.unwrap_or_else(default_rpc_port);
                parsed.set_port(Some(port)).map_err(|_| {
                    RegistryError::InvalidRecord(row_number, "invalid port".to_string())
                })?;
            }

            parsed
        } else {
            return Err(RegistryError::InvalidRecord(
                row_number,
                "missing rpc_url or host/ip column".to_string(),
            ));
        };

        if url.scheme() != "http" && url.scheme() != "https" {
            return Err(RegistryError::InvalidRecord(
                row_number,
                format!("unsupported url scheme `{}`", url.scheme()),
            ));
        }

        if url.host().is_none() {
            return Err(RegistryError::InvalidRecord(
                row_number,
                "url is missing host".to_string(),
            ));
        }

        if url.port().is_none() {
            url.set_port(Some(default_rpc_port())).map_err(|_| {
                RegistryError::InvalidRecord(row_number, "invalid port".to_string())
            })?;
        }

        let name = record
            .name
            .and_then(|value| {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            })
            .unwrap_or_else(|| generate_default_name(&location, ordinal));

        Ok(Validator::new(name, location, url))
    }
}

fn normalize_key(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn default_protocol() -> String {
    "http".to_string()
}

fn default_rpc_port() -> u16 {
    8899
}

fn prepare_host_for_url(host: &str) -> String {
    let trimmed = host.trim();

    if trimmed.contains(':')
        && !trimmed.contains('.')
        && !trimmed.starts_with('[')
        && !trimmed.ends_with(']')
    {
        format!("[{trimmed}]")
    } else {
        trimmed.to_string()
    }
}

fn generate_default_name(location: &str, ordinal: usize) -> String {
    let sanitized = location
        .trim()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();

    let cleaned = sanitized.trim_matches('-');

    if cleaned.is_empty() {
        format!("validator-{ordinal}")
    } else {
        format!("{cleaned}-{ordinal}")
    }
}