//! Command-line interface definitions using clap.
//!
//! Defines the CLI structure with subcommands for `logs` and `spans` queries.

use clap::{Parser, Subcommand};

/// Main CLI application structure.
#[derive(Parser)]
#[command(name = "dd-search")]
#[command(about = "Query Datadog logs and APM spans from the command line")]
#[command(version)]
pub struct Cli {
    /// Enable verbose/debug output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for querying Datadog.
#[derive(Subcommand)]
pub enum Commands {
    /// Search logs using Datadog query syntax
    Logs {
        /// Datadog query string (e.g., "service:api AND @http.status_code:500")
        query: String,

        /// Start time - relative (now-1h), ISO8601 (2024-01-15T10:00:00Z), or Unix ms (1705315200000)
        #[arg(short, long, default_value = "now-1h")]
        from: String,

        /// End time - relative (now), ISO8601 (2024-01-15T10:00:00Z), or Unix ms (1705315200000)
        #[arg(short, long, default_value = "now")]
        to: String,

        /// Maximum number of results to return (use 0 for unlimited)
        #[arg(short, long, default_value = "100")]
        limit: u64,

        /// Log indexes to search (comma-separated, default: all)
        #[arg(short, long, value_delimiter = ',', default_value = "*")]
        indexes: Vec<String>,
    },

    /// Search APM spans using Datadog query syntax
    Spans {
        /// Datadog query string (e.g., "service:web env:prod @duration:>1s")
        query: String,

        /// Start time - relative (now-1h), ISO8601 (2024-01-15T10:00:00Z), or Unix ms (1705315200000)
        #[arg(short, long, default_value = "now-1h")]
        from: String,

        /// End time - relative (now), ISO8601 (2024-01-15T10:00:00Z), or Unix ms (1705315200000)
        #[arg(short, long, default_value = "now")]
        to: String,

        /// Maximum number of results to return (use 0 for unlimited)
        #[arg(short, long, default_value = "100")]
        limit: u64,
    },
}
