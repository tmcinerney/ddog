//! Spans subcommand implementation.
//!
//! Handles the `dd-search spans` command, streaming APM span results to stdout.

use futures_util::StreamExt;

use crate::client::SpansClient;
use crate::error::AppError;
use crate::output::NdjsonWriter;

/// Executes the spans search command.
///
/// Streams matching span records to stdout as NDJSON until the limit is reached
/// or all results are exhausted.
pub async fn run(
    client: SpansClient,
    query: String,
    from: String,
    to: String,
    limit: u64,
) -> Result<(), AppError> {
    let mut writer = NdjsonWriter::new();
    let mut stream = std::pin::pin!(client.search(&query, &from, &to));
    let mut count: u64 = 0;

    while let Some(result) = stream.next().await {
        let span = result.map_err(|e| {
            let msg = format!("{}", e);
            if msg.contains("401") || msg.contains("403") || msg.contains("Forbidden") {
                AppError::Auth(format!("Authentication failed: {}", msg))
            } else if msg.contains("400") || msg.contains("Bad Request") {
                AppError::InvalidQuery(msg)
            } else {
                AppError::Api(msg)
            }
        })?;

        writer.write(&span)?;
        count += 1;

        if limit > 0 && count >= limit {
            break;
        }
    }

    Ok(())
}
