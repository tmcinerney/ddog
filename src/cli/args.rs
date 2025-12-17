//! Main CLI argument definitions.

use clap::{Parser, Subcommand};

use super::logs::LogsAction;
use super::metrics::MetricsAction;
use super::spans::SpansAction;

/// Main CLI application structure.
#[derive(Parser, Debug)]
#[command(name = "ddog")]
#[command(about = "Query Datadog logs, APM spans, and metrics from the command line")]
#[command(version)]
pub struct Cli {
    /// Enable verbose/debug output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub domain: Domain,
}

/// Available domains for querying Datadog.
#[derive(Subcommand, Debug)]
pub enum Domain {
    /// Logs domain - search and analyze logs
    Logs {
        #[command(subcommand)]
        action: LogsAction,
    },

    /// Spans domain - search and analyze APM traces
    Spans {
        #[command(subcommand)]
        action: SpansAction,
    },

    /// Metrics domain - query and list metrics
    Metrics {
        #[command(subcommand)]
        action: MetricsAction,
    },
}
