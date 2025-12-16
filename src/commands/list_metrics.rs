//! List metrics subcommand implementation.
//!
//! Handles the `dd-search list-metrics` command, listing available metrics to stdout.

use futures_util::StreamExt;
use serde::Serialize;

use crate::logging::VerboseLogger;
use crate::output::NdjsonWriter;
use dd_search::client::MetricsClient;
use dd_search::error::AppError;
use dd_search::time::parse_to_unix_seconds;

/// A metric name wrapper for JSON serialization.
#[derive(Debug, Serialize)]
struct MetricName {
    metric: String,
}

/// Executes the list-metrics command.
///
/// Lists active metrics within the specified time window and streams them to stdout as NDJSON.
pub async fn run(
    client: MetricsClient,
    from: String,
    to: Option<String>,
    logger: VerboseLogger,
) -> Result<(), AppError> {
    // Convert time string to Unix seconds
    let from_secs = parse_to_unix_seconds(&from)?;
    let _to_secs = if let Some(to_str) = to {
        Some(parse_to_unix_seconds(&to_str)?)
    } else {
        None
    };

    logger.log(&format!("Listing active metrics from {}", from_secs));

    let mut writer = NdjsonWriter::new();
    let mut stream = std::pin::pin!(client.list_active(from_secs));
    let mut count: u64 = 0;

    while let Some(result) = stream.next().await {
        let metric_name = result?;
        writer.write(&MetricName {
            metric: metric_name,
        })?;
        count += 1;
    }

    logger.log(&format!("Listed {} active metric(s)", count));
    Ok(())
}
