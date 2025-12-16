//! # dd-search
//!
//! A command-line tool for querying Datadog logs, APM spans, and metrics.
//!
//! ## Usage
//!
//! ```bash
//! dd-search logs "service:api AND status:error" --from now-1h
//! dd-search spans "service:web env:prod" --limit 50
//! dd-search metrics "avg:system.cpu.user{*}" --from now-1h
//! dd-search list-metrics --from now-1h
//! ```
//!
//! ## Environment Variables
//!
//! - `DD_API_KEY` - Datadog API key (required)
//! - `DD_APP_KEY` - Datadog application key (required)
//! - `DD_SITE` - Datadog site (optional, defaults to datadoghq.com)

use clap::Parser;

mod cli;
mod commands;
mod logging;
mod output;

// Import from library crate
use dd_search::client;
use dd_search::config;
use dd_search::error::AppError;

use cli::{Cli, Commands};
use logging::VerboseLogger;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}

async fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let logger = VerboseLogger::new(cli.verbose);
    let config = config::load_config()?;

    // Get site for URL construction
    let site = std::env::var("DD_SITE").unwrap_or_else(|_| "datadoghq.com".to_string());
    let has_api_key = std::env::var("DD_API_KEY").is_ok();
    let has_app_key = std::env::var("DD_APP_KEY").is_ok();

    logger.log_config(&site, has_api_key, has_app_key);

    match cli.command {
        Commands::Logs {
            query,
            from,
            to,
            limit,
            indexes,
        } => {
            logger.log_request("logs", &query, &from, &to);
            logger.log_api_endpoint("/api/v2/logs/events", "POST");
            logger.log_datadog_url("logs", &query, &from, &to, &site);

            let client = client::LogsClient::new(config);
            commands::logs::run(client, query, from, to, indexes, limit, logger).await
        }
        Commands::Spans {
            query,
            from,
            to,
            limit,
        } => {
            logger.log_request("spans", &query, &from, &to);
            logger.log_api_endpoint("/api/v2/spans/events/search", "POST");
            logger.log_datadog_url("spans", &query, &from, &to, &site);

            let client = client::SpansClient::new(config);
            commands::spans::run(client, query, from, to, limit, logger).await
        }
        Commands::Metrics {
            query,
            from,
            to,
            limit,
        } => {
            logger.log_request("metrics", &query, &from, &to);
            logger.log_api_endpoint("/api/v1/query", "GET");

            let client = client::MetricsClient::new(config);
            commands::metrics::run(client, query, from, to, limit, logger).await
        }
        Commands::ListMetrics { from, to } => {
            let to_display = to.as_deref().unwrap_or("now");
            logger.log(&format!(
                "Listing active metrics from {} to {}",
                from, to_display
            ));
            logger.log_api_endpoint("/api/v1/metrics", "GET");

            let client = client::MetricsClient::new(config);
            commands::list_metrics::run(client, from, to, logger).await
        }
    }
}
