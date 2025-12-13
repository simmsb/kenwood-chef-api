use clap::{Parser, Subcommand};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

pub mod entities;
pub mod ingest;
pub mod server;
pub mod types;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Server,
    IngestData(ingest::IngestData),
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    let cli = Cli::parse();

    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    match cli.command {
        Commands::Server => server::run().await?,
        Commands::IngestData(data) => ingest::run(data).await?,
    }

    Ok(())
}
