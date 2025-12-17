//! Logs search command implementation.
//!
//! Handles the `ddog logs search` command, streaming log results to stdout.

use futures_util::StreamExt;

use crate::cli::{Pagination, TimeRange};
use crate::logging::VerboseLogger;
use crate::output::NdjsonWriter;
use ddog::client::LogsClient;
use ddog::error::AppError;

/// Executes the logs search command.
///
/// Streams matching log records to stdout as NDJSON until the limit is reached
/// or all results are exhausted.
pub async fn run(
    client: LogsClient,
    query: String,
    time_range: TimeRange,
    pagination: Pagination,
    indexes: Vec<String>,
    logger: VerboseLogger,
) -> Result<(), AppError> {
    let mut writer = NdjsonWriter::new();
    let mut stream =
        std::pin::pin!(client.search(&query, &time_range.from, &time_range.to, indexes));
    let mut count: u64 = 0;

    while let Some(result) = stream.next().await {
        let log = result.map_err(|e| {
            let msg = format!("{}", e);
            logger.log_error(&msg, "logs API request");

            if msg.contains("401") {
                AppError::Auth(format!(
                    "Authentication failed (401): Invalid API or App key. {}",
                    msg
                ))
            } else if msg.contains("403") || msg.contains("Forbidden") {
                AppError::Auth(format!(
                    "Access denied (403): Your API key may not have permission to access logs. {}",
                    msg
                ))
            } else if msg.contains("400") || msg.contains("Bad Request") {
                AppError::InvalidQuery(msg)
            } else {
                AppError::Api(msg)
            }
        })?;

        writer.write(&log)?;
        count += 1;

        if pagination.limit > 0 && count >= pagination.limit {
            logger.log(&format!("Reached limit of {} results", pagination.limit));
            break;
        }
    }

    logger.log(&format!("Returned {} log(s)", count));
    Ok(())
}

#[cfg(test)]
mod tests {
    use ddog::error::AppError;

    fn parse_error_message(msg: &str) -> AppError {
        if msg.contains("401") || msg.contains("403") || msg.contains("Forbidden") {
            AppError::Auth(format!("Authentication failed: {}", msg))
        } else if msg.contains("400") || msg.contains("Bad Request") {
            AppError::InvalidQuery(msg.to_string())
        } else {
            AppError::Api(msg.to_string())
        }
    }

    #[test]
    fn test_error_parsing_401() {
        let error = parse_error_message("401 Unauthorized");
        assert!(matches!(error, AppError::Auth(_)));
        assert_eq!(error.exit_code(), 2);
    }

    #[test]
    fn test_error_parsing_403() {
        let error = parse_error_message("403 Forbidden");
        assert!(matches!(error, AppError::Auth(_)));
        assert_eq!(error.exit_code(), 2);
    }

    #[test]
    fn test_error_parsing_forbidden() {
        let error = parse_error_message("Forbidden access");
        assert!(matches!(error, AppError::Auth(_)));
        assert_eq!(error.exit_code(), 2);
    }

    #[test]
    fn test_error_parsing_400() {
        let error = parse_error_message("400 Bad Request");
        assert!(matches!(error, AppError::InvalidQuery(_)));
        assert_eq!(error.exit_code(), 4);
    }

    #[test]
    fn test_error_parsing_bad_request() {
        let error = parse_error_message("Bad Request: invalid syntax");
        assert!(matches!(error, AppError::InvalidQuery(_)));
        assert_eq!(error.exit_code(), 4);
    }

    #[test]
    fn test_error_parsing_generic_api_error() {
        let error = parse_error_message("500 Internal Server Error");
        assert!(matches!(error, AppError::Api(_)));
        assert_eq!(error.exit_code(), 3);
    }

    #[test]
    fn test_error_parsing_network_error() {
        let error = parse_error_message("Connection timeout");
        assert!(matches!(error, AppError::Api(_)));
        assert_eq!(error.exit_code(), 3);
    }
}
