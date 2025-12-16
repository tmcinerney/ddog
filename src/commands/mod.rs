//! Command handlers for CLI subcommands.
//!
//! Each module implements a subcommand that queries Datadog and streams results.

pub mod list_metrics;
pub mod logs;
pub mod metrics;
pub mod spans;
