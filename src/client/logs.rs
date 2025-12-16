//! Datadog Logs API client wrapper.
//!
//! Provides a simplified interface for searching logs with automatic pagination.

use datadog_api_client::datadog::{self, Configuration};
use datadog_api_client::datadogV2::api_logs::{ListLogsOptionalParams, LogsAPI};
use datadog_api_client::datadogV2::model::{
    Log, LogsListRequest, LogsListRequestPage, LogsQueryFilter, LogsSort,
};
use futures_util::Stream;

/// Client for querying Datadog logs.
///
/// Wraps the Datadog SDK's LogsAPI with automatic pagination support.
pub struct LogsClient {
    api: LogsAPI,
}

impl LogsClient {
    /// Creates a new LogsClient with the given configuration.
    pub fn new(config: Configuration) -> Self {
        Self {
            api: LogsAPI::with_config(config),
        }
    }

    /// Searches logs matching the given query.
    ///
    /// Returns an async stream of log records. The stream handles pagination
    /// automatically, fetching up to 1000 records per API request.
    ///
    /// # Arguments
    ///
    /// * `query` - Datadog query syntax (e.g., "service:api AND status:error")
    /// * `from` - Start time (relative like "now-1h" or ISO8601)
    /// * `to` - End time (relative like "now" or ISO8601)
    /// * `indexes` - Log indexes to search (use ["*"] for all)
    pub fn search(
        &self,
        query: &str,
        from: &str,
        to: &str,
        indexes: Vec<String>,
    ) -> impl Stream<Item = Result<Log, datadog::Error<datadog_api_client::datadogV2::api_logs::ListLogsError>>> + '_
    {
        let body = LogsListRequest::new()
            .filter(
                LogsQueryFilter::new()
                    .query(query.to_string())
                    .from(from.to_string())
                    .to(to.to_string())
                    .indexes(indexes),
            )
            .page(LogsListRequestPage::new().limit(1000))
            .sort(LogsSort::TIMESTAMP_ASCENDING);

        self.api
            .list_logs_with_pagination(ListLogsOptionalParams::default().body(body))
    }
}
