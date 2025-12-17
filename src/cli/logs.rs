//! Logs domain command actions.

use clap::Subcommand;

use super::shared::{Pagination, TimeRange};

/// Available actions for the logs domain.
#[derive(Subcommand, Debug)]
pub enum LogsAction {
    /// Search logs using Datadog query syntax
    Search {
        /// Datadog query string (e.g., "service:api AND @http.status_code:500")
        query: String,

        #[command(flatten)]
        time_range: TimeRange,

        #[command(flatten)]
        pagination: Pagination,

        /// Log indexes to search (comma-separated, default: all)
        #[arg(short, long, value_delimiter = ',', default_value = "*")]
        indexes: Vec<String>,
    },
}
