use std::sync::Arc;

use bdk::bitcoincore_rpc::RawTx;
use bitcoin::Txid;
use jsonrpsee::{
    core::{async_trait, RpcResult},
    types::{error::INTERNAL_ERROR_CODE, ErrorObjectOwned},
};
use tokio::sync::Mutex;
use tracing::{debug, error, instrument};

use crate::funder::{rpc::RpcFunder, Funder};

use super::api::{FaucetRpcServer, FundRequestParams, RecipientWithAmount};

pub struct FaucetRpcServerImpl {
    funder: Arc<Mutex<RpcFunder>>,
}

// SAFETY: `Server` is `Send` and `Sync` because it is `Arc<Mutex<Funder>>` is `Send` and `Sync`.
unsafe impl Send for FaucetRpcServerImpl {}
unsafe impl Sync for FaucetRpcServerImpl {}

impl FaucetRpcServerImpl {
    pub fn new(funder: RpcFunder) -> Self {
        Self {
            funder: Arc::new(Mutex::new(funder)),
        }
    }
}

#[async_trait]
impl FaucetRpcServer for FaucetRpcServerImpl {
    #[instrument(skip(self), level = "debug")]
    async fn fund(&self, request: FundRequestParams) -> RpcResult<Txid> {
        let recipients: Vec<RecipientWithAmount> = request.into();

        let wallet = self.funder.lock().await;

        let tx = wallet.fund(recipients).map_err(|err| {
            error!(%err, "Failed to fund transaction");

            ErrorObjectOwned::owned(
                INTERNAL_ERROR_CODE,
                "Internal error",
                Option::<Vec<u8>>::None,
            )
        })?;

        let txid = tx.txid();

        debug!(?txid, tx = tx.raw_hex(), "Funded transaction");

        Ok(tx.txid())
    }
}
