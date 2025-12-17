//! Metrics query command implementation.
//!
//! Handles the `ddog metrics query` command, streaming metric timeseries points to stdout.

use futures_util::StreamExt;

use crate::cli::TimeRange;
use crate::logging::VerboseLogger;
use crate::output::NdjsonWriter;
use ddog::client::MetricsClient;
use ddog::error::AppError;
use ddog::time::parse_to_unix_seconds;

/// Executes the metrics query command.
///
/// Queries metrics timeseries data and streams individual points to stdout as NDJSON
/// until the limit is reached or all results are exhausted.
pub async fn run(
    client: MetricsClient,
    query: String,
    time_range: TimeRange,
    limit: u64,
    logger: VerboseLogger,
) -> Result<(), AppError> {
    // Convert time strings to Unix seconds
    let from_secs = parse_to_unix_seconds(&time_range.from)?;
    let to_secs = parse_to_unix_seconds(&time_range.to)?;

    logger.log(&format!(
        "Querying metrics from {} to {} (Unix seconds)",
        from_secs, to_secs
    ));

    let mut writer = NdjsonWriter::new();
    let mut stream = std::pin::pin!(client.query(&query, from_secs, to_secs));
    let mut count: u64 = 0;

    while let Some(result) = stream.next().await {
        let point = result?;
        writer.write(&point)?;
        count += 1;

        if limit > 0 && count >= limit {
            logger.log(&format!("Reached limit of {} results", limit));
            break;
        }
    }

    logger.log(&format!("Returned {} metric point(s)", count));
    Ok(())
}
