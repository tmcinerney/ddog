//! Logs subcommand implementation.
//!
//! Handles the `dd-search logs` command, streaming log results to stdout.

use futures_util::StreamExt;

use crate::client::LogsClient;
use crate::error::AppError;
use crate::output::NdjsonWriter;

/// Executes the logs search command.
///
/// Streams matching log records to stdout as NDJSON until the limit is reached
/// or all results are exhausted.
pub async fn run(
    client: LogsClient,
    query: String,
    from: String,
    to: String,
    indexes: Vec<String>,
    limit: u64,
) -> Result<(), AppError> {
    let mut writer = NdjsonWriter::new();
    let mut stream = std::pin::pin!(client.search(&query, &from, &to, indexes));
    let mut count: u64 = 0;

    while let Some(result) = stream.next().await {
        let log = result.map_err(|e| {
            let msg = format!("{}", e);
            if msg.contains("401") || msg.contains("403") || msg.contains("Forbidden") {
                AppError::Auth(format!("Authentication failed: {}", msg))
            } else if msg.contains("400") || msg.contains("Bad Request") {
                AppError::InvalidQuery(msg)
            } else {
                AppError::Api(msg)
            }
        })?;

        writer.write(&log)?;
        count += 1;

        if limit > 0 && count >= limit {
            break;
        }
    }

    Ok(())
}
