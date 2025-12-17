//! Spans domain command actions.

use clap::Subcommand;

use super::shared::{Pagination, TimeRange};

/// Available actions for the spans domain.
#[derive(Subcommand, Debug)]
pub enum SpansAction {
    /// Search APM spans using Datadog query syntax
    Search {
        /// Datadog query string (e.g., "service:web env:prod @duration:>1s")
        query: String,

        #[command(flatten)]
        time_range: TimeRange,

        #[command(flatten)]
        pagination: Pagination,
    },
}
