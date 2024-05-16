use bdk::SyncOptions;
use bitcoincore_rpc::jsonrpc::serde_json::json;
use color_eyre::eyre::{self, Context};
use jsonrpsee::server::Server;
use jsonrpsee_server::{middleware::http::ProxyGetRequestLayer, Methods, RpcModule};
use tokio::select;

use crate::{
    config::Config,
    funder::{rpc::RpcFunder, FunderConfig, FunderFromConfig},
    server::r#impl::FaucetRpcServerImpl,
};

use self::api::FaucetRpcServer;

pub(crate) mod api;
pub(crate) mod r#impl;

pub async fn run(config: Config) -> eyre::Result<()> {
    let funder_config = FunderConfig::from(&config);

    let funder = RpcFunder::from_config(funder_config).wrap_err("failed to create funder")?;

    funder
        .init(SyncOptions::default())
        .wrap_err("Failed to init funder")?; // TODO: may be add progress bar

    let faucet_impl = FaucetRpcServerImpl::new(funder);

    let middleware = tower::ServiceBuilder::new()
        // Proxy `GET /health` requests to internal `system_health` method.
        .layer(ProxyGetRequestLayer::new("/health", "system_health").unwrap());

    let mut module = RpcModule::new(());
    module
        .register_method("system_health", |_, _| json!({"health": "true"}))
        .unwrap();

    let server = Server::builder()
        .http_only()
        .set_http_middleware(middleware)
        .build(config.listen_address)
        .await?;

    let mut methods: Methods = module.into();
    methods.merge(faucet_impl.into_rpc()).unwrap();

    let faucet_handle = server.start(methods);

    select! {
        _ = tokio::signal::ctrl_c() => {
            faucet_handle.stop()?;
        }
    }

    Ok(())
}
