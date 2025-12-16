//! Datadog Metrics API client wrapper.
//!
//! Provides a simplified interface for querying metrics timeseries data and listing available metrics.

use datadog_api_client::datadog::Configuration;
use datadog_api_client::datadogV1::api_metrics::{ListActiveMetricsOptionalParams, MetricsAPI};
use futures_util::stream::{self, Stream, StreamExt};
use std::pin::Pin;

use crate::error::AppError;

/// Client for querying Datadog metrics.
///
/// Wraps the Datadog SDK's MetricsAPI for querying timeseries data and listing metrics.
pub struct MetricsClient {
    api: MetricsAPI,
}

impl MetricsClient {
    /// Creates a new MetricsClient with the given configuration.
    pub fn new(config: Configuration) -> Self {
        Self {
            api: MetricsAPI::with_config(config),
        }
    }

    /// Queries metrics timeseries data.
    ///
    /// Returns an async stream of individual timeseries points. Each point is flattened
    /// from the API response into a single record containing the metric name, timestamp,
    /// value, tags, and scope.
    ///
    /// # Arguments
    ///
    /// * `query` - Datadog metric query string (e.g., "avg:system.cpu.user{*}")
    /// * `from` - Start time in Unix seconds
    /// * `to` - End time in Unix seconds
    pub fn query(
        &self,
        query: &str,
        from: i64,
        to: i64,
    ) -> Pin<Box<dyn Stream<Item = Result<MetricPoint, AppError>> + Send + '_>> {
        let query = query.to_string();
        let api = &self.api;

        Box::pin(
            stream::once(async move {
                // Call the Datadog API
                let result = api.query_metrics(from, to, query.clone()).await;

                // Handle the result
                match result {
                    Ok(response) => {
                        // Flatten all series and points into individual MetricPoint records
                        let points: Vec<MetricPoint> = response
                            .series
                            .unwrap_or_default()
                            .into_iter()
                            .flat_map(|series| {
                                let metric_name = series.metric.clone().unwrap_or_default();
                                let display_name = series.display_name.clone();
                                let query_index = series.query_index;
                                let aggr = series.aggr.clone().and_then(|a| a);
                                let scope = series.scope.clone().unwrap_or_default();
                                let tag_set = series.tag_set.clone().unwrap_or_default();

                                series.pointlist.unwrap_or_default().into_iter().filter_map(
                                    move |point| {
                                        // Extract timestamp and value from Option<f64>
                                        let timestamp_ms = point.first().copied().flatten()? as i64;
                                        let value = point.get(1).copied().flatten()?;

                                        Some(MetricPoint {
                                            metric: metric_name.clone(),
                                            display_name: display_name.clone(),
                                            query_index,
                                            aggr: aggr.clone(),
                                            scope: scope.clone(),
                                            tag_set: tag_set.clone(),
                                            timestamp: timestamp_ms / 1000, // Convert to seconds
                                            value,
                                        })
                                    },
                                )
                            })
                            .collect();

                        // Return a stream of the points
                        stream::iter(points.into_iter().map(Ok)).boxed()
                    }
                    Err(e) => {
                        // Convert the error and return it as a single-item stream
                        let app_error = convert_datadog_error(e);
                        stream::once(async move { Err(app_error) }).boxed()
                    }
                }
            })
            .flatten(),
        )
    }

    /// Lists active metrics within a time window.
    ///
    /// Returns an async stream of metric names that were actively reporting
    /// during the specified time period.
    ///
    /// # Arguments
    ///
    /// * `from` - Start time in Unix seconds
    pub fn list_active(
        &self,
        from: i64,
    ) -> Pin<Box<dyn Stream<Item = Result<String, AppError>> + Send + '_>> {
        let api = &self.api;

        Box::pin(
            stream::once(async move {
                // Call the Datadog API with from time and default optional params
                let result = api
                    .list_active_metrics(from, ListActiveMetricsOptionalParams::default())
                    .await;

                // Handle the result
                match result {
                    Ok(response) => {
                        // Extract metric names from the response
                        let metrics = response.metrics.unwrap_or_default();
                        stream::iter(metrics.into_iter().map(Ok)).boxed()
                    }
                    Err(e) => {
                        // Convert the error and return it as a single-item stream
                        let app_error = convert_datadog_error(e);
                        stream::once(async move { Err(app_error) }).boxed()
                    }
                }
            })
            .flatten(),
        )
    }
}

/// Converts a Datadog API error to an AppError.
fn convert_datadog_error<T: std::fmt::Display>(e: T) -> AppError {
    let msg = format!("{}", e);

    if msg.contains("401") {
        AppError::Auth(format!(
            "Authentication failed (401): Invalid API or App key. {}",
            msg
        ))
    } else if msg.contains("403") || msg.contains("Forbidden") {
        AppError::Auth(format!(
            "Access denied (403): Your API key may not have permission to access metrics. {}",
            msg
        ))
    } else if msg.contains("400") || msg.contains("Bad Request") {
        AppError::InvalidQuery(msg)
    } else {
        AppError::Api(msg)
    }
}

/// A single metric timeseries point.
///
/// This struct represents a flattened view of a metric point from the Datadog API.
#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricPoint {
    /// Metric name (e.g., "system.cpu.user")
    pub metric: String,

    /// Display name for the metric
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Query index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_index: Option<i64>,

    /// Aggregation method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggr: Option<String>,

    /// Scope of the metric query
    pub scope: String,

    /// List of tags associated with this metric series
    pub tag_set: Vec<String>,

    /// Timestamp in Unix seconds
    pub timestamp: i64,

    /// Metric value at this timestamp
    pub value: f64,
}
