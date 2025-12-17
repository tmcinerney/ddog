//! Metrics list command implementation.
//!
//! Handles the `ddog metrics list` command, listing available metrics to stdout.

use futures_util::StreamExt;
use serde::Serialize;

use crate::cli::TimeFrom;
use crate::logging::VerboseLogger;
use crate::output::NdjsonWriter;
use ddog::client::MetricsClient;
use ddog::error::AppError;
use ddog::time::parse_to_unix_seconds;

/// A metric name wrapper for JSON serialization.
#[derive(Debug, Serialize)]
struct MetricName {
    metric: String,
}

/// Executes the metrics list command.
///
/// Lists active metrics after the specified start time and streams them to stdout as NDJSON.
pub async fn run(
    client: MetricsClient,
    time_from: TimeFrom,
    logger: VerboseLogger,
) -> Result<(), AppError> {
    // Convert time string to Unix seconds
    let from_secs = parse_to_unix_seconds(&time_from.from)?;

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
