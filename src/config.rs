use std::{net::SocketAddr, path::PathBuf};

use bitcoin::{secp256k1::SecretKey, Network};
use color_eyre::eyre;
use url::Url;

use crate::funder::FunderConfig;

const FAUCET_BITCOIN_USERNAME_ENV: &str = "FAUCET_BITCOIN_USERNAME";
const FAUCET_BITCOIN_PASSWORD_ENV: &str = "FAUCET_BITCOIN_PASSWORD";

#[derive(serde::Deserialize)]
pub(crate) struct RawConfig {
    #[serde(default = "default_listen_address")]
    pub listen_address: String,

    #[serde(default = "default_database_path", rename = "database")]
    pub database_path: String,

    #[serde(default)]
    pub secret_key: Option<SecretKey>,

    pub bitcoin_url: String,

    #[serde(default = "default_bitcoin_username")]
    pub bitcoin_username: String,

    #[serde(default = "default_bitcoin_password")]
    pub bitcoin_password: String,

    #[serde(default)]
    pub bitcoin_network: Option<Network>,
}

fn default_listen_address() -> String {
    "0.0.0.0:18777".to_string()
}

fn default_database_path() -> String {
    "/root/bitcoin-faucet/db.sqlite".to_string()
}

fn default_bitcoin_username() -> String {
    from_env_or_default(FAUCET_BITCOIN_USERNAME_ENV)
}

fn default_bitcoin_password() -> String {
    from_env_or_default(FAUCET_BITCOIN_PASSWORD_ENV)
}

fn from_env_or_default(env_var: &'static str) -> String {
    std::env::var(env_var).unwrap_or_default()
}

#[derive(Debug)]
pub struct Config {
    pub listen_address: SocketAddr,
    pub database_path: PathBuf,

    pub secret_key: Option<SecretKey>,

    pub bitcoin_url: Url,
    pub bitcoin_auth: Option<(String, String)>,
    pub bitcoin_network: Option<Network>,
}

impl Config {
    pub fn load(path: PathBuf) -> eyre::Result<Self> {
        let config = std::fs::read_to_string(path).unwrap_or_default();

        let raw: RawConfig = toml::from_str(&config)?;

        Self::from_raw(raw)
    }

    fn from_raw(raw: RawConfig) -> eyre::Result<Self> {
        let listen_address = raw.listen_address.parse()?;
        let database_path = raw.database_path.parse()?;
        let bitcoin_url = Url::parse(&raw.bitcoin_url)?;

        let auth = if raw.bitcoin_username.is_empty() || raw.bitcoin_password.is_empty() {
            None
        } else {
            Some((raw.bitcoin_username, raw.bitcoin_password))
        };

        Ok(Self {
            listen_address,
            database_path,
            secret_key: raw.secret_key,
            bitcoin_url,
            bitcoin_auth: auth,
            bitcoin_network: raw.bitcoin_network,
        })
    }
}

impl From<&Config> for FunderConfig {
    fn from(value: &Config) -> Self {
        FunderConfig {
            secret_key: value.secret_key,
            bitcoin_url: value.bitcoin_url.to_string(),
            bitcoin_auth: value.bitcoin_auth.clone(),
            wallet_data_path: value.database_path.clone(),
            network: value.bitcoin_network,
        }
    }
}
