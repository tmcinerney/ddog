//! Metrics domain command actions.

use clap::Subcommand;

use super::shared::TimeRange;

/// Available actions for the metrics domain.
#[derive(Subcommand, Debug)]
pub enum MetricsAction {
    /// Query metrics timeseries data
    Query {
        /// Datadog metric query (e.g., "avg:system.cpu.user{*}")
        query: String,

        #[command(flatten)]
        time_range: TimeRange,

        /// Maximum number of data points to return (use 0 for unlimited)
        #[arg(short, long, default_value = "1000")]
        limit: u64,
    },

    /// List active metrics within a time window
    List {
        #[command(flatten)]
        time_range: TimeRange,
    },
}
