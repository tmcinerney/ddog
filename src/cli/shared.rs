//! Shared CLI argument structures used across multiple commands.

use clap::Args;

/// Time range arguments used by logs, spans, and metrics commands.
#[derive(Args, Debug, Clone)]
pub struct TimeRange {
    /// Start time - relative (now-1h), ISO8601 (2024-01-15T10:00:00Z), or Unix ms (1705315200000)
    #[arg(short, long, default_value = "now-1h")]
    pub from: String,

    /// End time - relative (now), ISO8601 (2024-01-15T10:00:00Z), or Unix ms (1705315200000)
    #[arg(short, long, default_value = "now")]
    pub to: String,
}

/// Pagination arguments for limiting query results.
#[derive(Args, Debug, Clone)]
pub struct Pagination {
    /// Maximum number of results to return (use 0 for unlimited)
    #[arg(short, long, default_value = "100")]
    pub limit: u64,
}
