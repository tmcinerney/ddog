//! Datadog Spans (APM) API client wrapper.
//!
//! Provides a simplified interface for searching APM spans with automatic pagination.

use datadog_api_client::datadog::{self, Configuration};
use datadog_api_client::datadogV2::api_spans::SpansAPI;
use datadog_api_client::datadogV2::model::{
    Span, SpansListRequest, SpansListRequestAttributes, SpansListRequestData,
    SpansListRequestPage, SpansListRequestType, SpansQueryFilter, SpansSort,
};
use futures_util::Stream;

/// Client for querying Datadog APM spans.
///
/// Wraps the Datadog SDK's SpansAPI with automatic pagination support.
pub struct SpansClient {
    api: SpansAPI,
}

impl SpansClient {
    /// Creates a new SpansClient with the given configuration.
    pub fn new(config: Configuration) -> Self {
        Self {
            api: SpansAPI::with_config(config),
        }
    }

    /// Searches APM spans matching the given query.
    ///
    /// Returns an async stream of span records. The stream handles pagination
    /// automatically, fetching up to 1000 records per API request.
    ///
    /// # Arguments
    ///
    /// * `query` - Datadog query syntax (e.g., "service:web env:prod @duration:>1s")
    /// * `from` - Start time (relative like "now-1h" or ISO8601)
    /// * `to` - End time (relative like "now" or ISO8601)
    pub fn search(
        &self,
        query: &str,
        from: &str,
        to: &str,
    ) -> impl Stream<Item = Result<Span, datadog::Error<datadog_api_client::datadogV2::api_spans::ListSpansError>>> + '_
    {
        let body = SpansListRequest::new().data(
            SpansListRequestData::new()
                .attributes(
                    SpansListRequestAttributes::new()
                        .filter(
                            SpansQueryFilter::new()
                                .query(query.to_string())
                                .from(from.to_string())
                                .to(to.to_string()),
                        )
                        .page(SpansListRequestPage::new().limit(1000))
                        .sort(SpansSort::TIMESTAMP_ASCENDING),
                )
                .type_(SpansListRequestType::SEARCH_REQUEST),
        );

        self.api.list_spans_with_pagination(body)
    }
}
