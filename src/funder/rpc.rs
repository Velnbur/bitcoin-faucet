use bdk::{
    blockchain::{ConfigurableBlockchain, RpcBlockchain, RpcConfig},
    database::SqliteDatabase,
    descriptor,
    wallet::AddressIndex,
    SignOptions, SyncOptions,
};
use bitcoin::{
    secp256k1::{Secp256k1, SecretKey},
    PrivateKey, Transaction,
};
use bitcoincore_rpc::{RawTx, RpcApi};
use color_eyre::eyre::{self, ensure, Context};
use rand::thread_rng;

use crate::server::api::RecipientWithAmount;

use super::{keyvalue::KeyValueStorage, Funder, FunderConfig, FunderFromConfig};

pub struct RpcFunder {
    wallet: bdk::Wallet<SqliteDatabase>,
    client: bitcoincore_rpc::Client,
    blockchain: RpcBlockchain,
}

impl FunderFromConfig for RpcFunder {
    fn from_config(config: FunderConfig) -> eyre::Result<Self> {
        let client = bitcoincore_rpc::Client::new(
            &config.bitcoin_url,
            rpc_auth_from_tuple(config.bitcoin_auth.clone()),
        )?;

        let network = match config.network {
            Some(network) => network,
            None => client.get_blockchain_info()?.chain.parse()?,
        };

        let db = SqliteDatabase::new(config.wallet_data_path);

        let secret_key = Self::get_or_generate_private_key(&db, config.secret_key)?;
        let private_key = PrivateKey::new(secret_key, network);

        let wallet = bdk::Wallet::new(descriptor!(wpkh(private_key))?, None, network, db)?;

        let blockchain = RpcBlockchain::from_config(&RpcConfig {
            url: config.bitcoin_url,
            auth: bdk_auth_from_tuple(config.bitcoin_auth),
            wallet_name: format!(
                "funder-{}",
                wallet.get_descriptor_for_keychain(bdk::KeychainKind::External)
            ),
            sync_params: None,
            network,
        })
        .wrap_err("failed to initialize rpc blockchain provider")?;

        Ok(Self::new(wallet, client, blockchain))
    }
}

impl RpcFunder {
    /// Get private key from sqlite database if it wasn't provided in the config. In case if it's missing,
    /// generate new one and store it in the database.
    fn get_or_generate_private_key(
        db: &SqliteDatabase,
        config_key: Option<SecretKey>,
    ) -> eyre::Result<SecretKey> {
        if let Some(secret_key) = config_key {
            return Ok(secret_key);
        }

        let storage = KeyValueStorage::new(db);

        if let Some(private_key) = storage.private_key()? {
            return Ok(private_key.parse()?);
        }

        let secp_ctx = Secp256k1::new();
        let secret_key = secp_ctx.generate_keypair(&mut thread_rng()).0;

        storage.insert_private_key(&secret_key.secret_bytes().raw_hex())?;

        Ok(secret_key)
    }
}

fn rpc_auth_from_tuple(auth: Option<(String, String)>) -> bitcoincore_rpc::Auth {
    match auth {
        Some((username, password)) => bitcoincore_rpc::Auth::UserPass(username, password),
        None => bitcoincore_rpc::Auth::None,
    }
}

fn bdk_auth_from_tuple(auth: Option<(String, String)>) -> bdk::blockchain::rpc::Auth {
    match auth {
        Some((username, password)) => bdk::blockchain::rpc::Auth::UserPass { username, password },
        None => bdk::blockchain::rpc::Auth::None,
    }
}

impl Funder for RpcFunder {
    /// Broadcast and fund transactions for given recipients with
    fn fund(&self, recipients: Vec<RecipientWithAmount>) -> eyre::Result<Transaction> {
        let amounts_sum = recipients.iter().map(|(_, amount)| *amount).sum::<u64>();

        let balance = self.wallet.get_balance()?;

        if balance.get_spendable() < amounts_sum {
            self.generate_until_enough(amounts_sum)?;
        }

        let funding_tx = self.build_funding_tx(&recipients)?;

        self.broadcast_tx(&funding_tx)?;

        self.generate_blocks_and_sync(1)?;

        Ok(funding_tx)
    }
}

impl RpcFunder {
    pub fn new(
        wallet: bdk::Wallet<SqliteDatabase>,
        client: bitcoincore_rpc::Client,
        blockchain: RpcBlockchain,
    ) -> Self {
        Self {
            wallet,
            client,
            blockchain,
        }
    }

    /// Generate 100 + 1 blocks to make coins from coinbase mature.
    pub fn init(&self, options: SyncOptions) -> eyre::Result<()> {
        if self.wallet.get_balance()?.get_spendable() != 0 {
            return Ok(());
        }

        self.generate_blocks_and_sync_with_opts(101, options)
    }

    pub fn sync(&self, options: SyncOptions) -> eyre::Result<()> {
        self.wallet.sync(&self.blockchain, options)?;

        Ok(())
    }

    /// Generate blocks until coins from coinbase are mature and/or the pending
    /// balance is enough to fullfill the funding request.
    fn generate_until_enough(&self, amount: u64) -> eyre::Result<()> {
        while self.wallet.get_balance()?.get_spendable() < amount {
            self.generate_blocks_and_sync(1)?;
        }

        Ok(())
    }

    fn generate_blocks_and_sync(&self, number: usize) -> eyre::Result<()> {
        self.generate_blocks_and_sync_with_opts(number, SyncOptions::default())
    }

    fn generate_blocks_and_sync_with_opts(
        &self,
        number: usize,
        opts: SyncOptions,
    ) -> Result<(), eyre::Error> {
        self.generate_blocks(number)?;
        self.sync(opts)?;

        Ok(())
    }

    /// Generate block to self using Bitcoin Core RPC.
    fn generate_blocks(&self, number: usize) -> Result<(), eyre::Error> {
        let address = self.wallet.get_address(AddressIndex::Peek(0))?;
        self.client.generate_to_address(number as u64, &address)?;
        Ok(())
    }

    fn build_funding_tx(&self, recipients: &[RecipientWithAmount]) -> eyre::Result<Transaction> {
        let mut builder = self.wallet.build_tx();

        for (recipient, amount) in recipients {
            builder.add_recipient(recipient.payload.script_pubkey(), *amount);
        }

        let (mut psbt, _details) = builder.finish()?;

        let finished = self.wallet.sign(&mut psbt, SignOptions::default())?;

        ensure!(finished, "Failed to sign and finalize the transaction");

        let tx = psbt.extract_tx();

        Ok(tx)
    }

    fn broadcast_tx(&self, tx: &Transaction) -> eyre::Result<()> {
        let tx_hex = bitcoin::consensus::serialize(tx).raw_hex();

        self.client.send_raw_transaction(tx_hex)?;

        Ok(())
    }
}
