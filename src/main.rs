//! # ddog
//!
//! A command-line tool for querying Datadog logs, APM spans, and metrics.
//!
//! ## Usage
//!
//! ```bash
//! ddog logs search "service:api AND status:error" --from now-1h
//! ddog spans search "service:web env:prod" --limit 50
//! ddog metrics query "avg:system.cpu.user{*}" --from now-1h
//! ddog metrics list --from now-1h
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
use ddog::client;
use ddog::config;
use ddog::error::AppError;

use cli::{Cli, Domain, LogsAction, MetricsAction, SpansAction};
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

    match cli.domain {
        Domain::Logs { action } => match action {
            LogsAction::Search {
                query,
                time_range,
                pagination,
                indexes,
            } => {
                logger.log_request("logs", &query, &time_range.from, &time_range.to);
                logger.log_api_endpoint("/api/v2/logs/events", "POST");
                logger.log_datadog_url("logs", &query, &time_range.from, &time_range.to, &site);

                let client = client::LogsClient::new(config);
                commands::logs::search::run(client, query, time_range, pagination, indexes, logger)
                    .await
            }
        },
        Domain::Spans { action } => match action {
            SpansAction::Search {
                query,
                time_range,
                pagination,
            } => {
                logger.log_request("spans", &query, &time_range.from, &time_range.to);
                logger.log_api_endpoint("/api/v2/spans/events/search", "POST");
                logger.log_datadog_url("spans", &query, &time_range.from, &time_range.to, &site);

                let client = client::SpansClient::new(config);
                commands::spans::search::run(client, query, time_range, pagination, logger).await
            }
        },
        Domain::Metrics { action } => match action {
            MetricsAction::Query {
                query,
                time_range,
                limit,
            } => {
                logger.log_request("metrics", &query, &time_range.from, &time_range.to);
                logger.log_api_endpoint("/api/v1/query", "GET");

                let client = client::MetricsClient::new(config);
                commands::metrics::query::run(client, query, time_range, limit, logger).await
            }
            MetricsAction::List { time_from } => {
                logger.log(&format!("Listing active metrics from {}", time_from.from));
                logger.log_api_endpoint("/api/v1/metrics", "GET");

                let client = client::MetricsClient::new(config);
                commands::metrics::list::run(client, time_from, logger).await
            }
        },
    }
}
