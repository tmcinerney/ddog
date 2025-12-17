//! Command-line interface definitions using clap.
//!
//! Defines the CLI structure with domain-based subcommands for querying Datadog.

mod args;
mod logs;
mod metrics;
mod shared;
mod spans;

pub use args::{Cli, Domain};
pub use logs::LogsAction;
pub use metrics::MetricsAction;
pub use shared::{Pagination, TimeFrom, TimeRange, TimeRangeRelativeOnly};
pub use spans::SpansAction;
