use cli::Cli;
use color_eyre::eyre;

mod cli;
mod config;
mod funder;
mod server;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    color_eyre::install()?;

    let cli = Cli::from_args();

    cli.exec().await
}
