use std::path::PathBuf;

use bitcoin::{secp256k1::SecretKey, Network, Transaction};
use color_eyre::eyre::{self};

use crate::server::api::RecipientWithAmount;

mod keyvalue;
pub(crate) mod rpc;

pub struct FunderConfig {
    /// Optional secret key which will be used by funder to sign messages.
    ///
    /// If `None` is provided, the funder will generate a new keypair.
    pub secret_key: Option<SecretKey>,

    /// URL of the Bitcoin Core RPC server.
    pub bitcoin_url: String,

    /// Optional username and password for the Bitcoin Core RPC server.
    pub bitcoin_auth: Option<(String, String)>,

    /// Path to the directory where the wallet data will be stored.
    pub wallet_data_path: PathBuf,

    /// Optional network to use for the wallet.
    ///
    /// If `None` is provided, the funder will get the fist network fron the
    /// `getnetworkinfo` RPC call.
    pub network: Option<Network>,
}

pub trait FunderFromConfig {
    fn from_config(config: FunderConfig) -> eyre::Result<Self>
    where
        Self: Sized;
}

pub trait Funder {
    /// Create and broadcast a transaction that funds the given recipients
    /// returning the tx.
    fn fund(&self, recipients: Vec<RecipientWithAmount>) -> eyre::Result<Transaction>;
}
