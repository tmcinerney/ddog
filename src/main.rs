//! # dd-search
//!
//! A command-line tool for querying Datadog logs and APM spans.
//!
//! ## Usage
//!
//! ```bash
//! dd-search logs "service:api AND status:error" --from now-1h
//! dd-search spans "service:web env:prod" --limit 50
//! ```
//!
//! ## Environment Variables
//!
//! - `DD_API_KEY` - Datadog API key (required)
//! - `DD_APP_KEY` - Datadog application key (required)
//! - `DD_SITE` - Datadog site (optional, defaults to datadoghq.com)

use clap::Parser;

mod cli;
mod client;
mod commands;
mod config;
mod error;
mod output;

use cli::{Cli, Commands};
use error::AppError;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}

async fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let config = config::load_config()?;

    match cli.command {
        Commands::Logs {
            query,
            from,
            to,
            limit,
            indexes,
        } => {
            let client = client::LogsClient::new(config);
            commands::logs::run(client, query, from, to, indexes, limit).await
        }
        Commands::Spans {
            query,
            from,
            to,
            limit,
        } => {
            let client = client::SpansClient::new(config);
            commands::spans::run(client, query, from, to, limit).await
        }
    }
}
