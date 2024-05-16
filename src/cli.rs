use clap::Parser;
use color_eyre::eyre::{self, Context};
use std::path::PathBuf;
use tracing::debug;

use crate::{config::Config, server};

#[derive(clap::Parser)]
pub struct Cli {
    #[clap(long, short, default_value = "/etc/bitcoin-faucet/config.toml")]
    pub config: PathBuf,
}

impl Cli {
    pub fn from_args() -> Self {
        Self::parse()
    }

    pub async fn exec(self) -> eyre::Result<()> {
        let config = Config::load(self.config).wrap_err("failed to parse config")?;

        debug!(?config, "Starting server...");

        server::run(config).await.wrap_err("failed to start server")
    }
}
