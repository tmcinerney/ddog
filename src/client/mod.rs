//! Datadog API client wrappers.
//!
//! Provides simplified interfaces to the Datadog SDK with automatic pagination.

mod logs;
mod metrics;
mod spans;

pub use logs::LogsClient;
pub use metrics::MetricsClient;
pub use spans::SpansClient;
