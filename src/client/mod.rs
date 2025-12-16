//! Datadog API client wrappers.
//!
//! Provides simplified interfaces to the Datadog SDK with automatic pagination.

mod logs;
mod spans;

pub use logs::LogsClient;
pub use spans::SpansClient;
