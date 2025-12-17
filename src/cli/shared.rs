//! Shared CLI argument structures used across multiple commands.

use clap::Args;

/// Time range arguments for logs and spans (supports ISO8601, relative, and Unix timestamps).
#[derive(Args, Debug, Clone)]
pub struct TimeRange {
    /// Start time - supports relative (now-1h), ISO8601 (2024-01-15T10:00:00Z), or Unix ms (1705315200000)
    #[arg(
        short,
        long,
        default_value = "now-1h",
        long_help = "Start time for the query.

Supported formats:
  • Relative: now, now-15m, now-1h, now-2d, now-1w, now-3mo, now-1y
    Units: s (seconds), m (minutes), h (hours), d (days), w (weeks), mo (months), y (years)
  • ISO8601: 2024-01-15T10:00:00Z, 2024-01-15T10:00:00+00:00
  • Unix timestamp: 1705315200000 (milliseconds since epoch)

Examples:
  --from now-30m     # 30 minutes ago
  --from now-1d      # 1 day ago
  --from 2024-01-15T10:00:00Z
  --from 1705315200000"
    )]
    pub from: String,

    /// End time - supports relative (now), ISO8601 (2024-01-15T10:00:00Z), or Unix ms (1705315200000)
    #[arg(
        short,
        long,
        default_value = "now",
        long_help = "End time for the query.

Supported formats:
  • Relative: now, now-15m, now-1h, now-2d, now-1w, now-3mo, now-1y
    Units: s (seconds), m (minutes), h (hours), d (days), w (weeks), mo (months), y (years)
  • ISO8601: 2024-01-15T10:00:00Z, 2024-01-15T10:00:00+00:00
  • Unix timestamp: 1705315200000 (milliseconds since epoch)

Examples:
  --to now           # Current time (default)
  --to now-5m        # 5 minutes ago
  --to 2024-01-15T11:00:00Z
  --to 1705318800000"
    )]
    pub to: String,
}

/// Time range arguments for metrics (supports only relative and Unix timestamps, no ISO8601).
#[derive(Args, Debug, Clone)]
pub struct TimeRangeRelativeOnly {
    /// Start time - supports relative (now-1h) or Unix ms (1705315200000). ISO8601 NOT supported for metrics.
    #[arg(
        short,
        long,
        default_value = "now-1h",
        long_help = "Start time for the query.

⚠️  Note: Metrics commands do NOT support ISO8601 format.

Supported formats:
  • Relative: now, now-15m, now-1h, now-2d, now-1w, now-3mo, now-1y
    Units: s (seconds), m (minutes), h (hours), d (days), w (weeks), mo (months), y (years)
  • Unix timestamp: 1705315200000 (milliseconds since epoch)

Examples:
  --from now-30m     # 30 minutes ago
  --from now-1d      # 1 day ago
  --from 1705315200000

Not supported:
  --from 2024-01-15T10:00:00Z  ❌ ISO8601 format not available for metrics"
    )]
    pub from: String,

    /// End time - supports relative (now) or Unix ms (1705315200000). ISO8601 NOT supported for metrics.
    #[arg(
        short,
        long,
        default_value = "now",
        long_help = "End time for the query.

⚠️  Note: Metrics commands do NOT support ISO8601 format.

Supported formats:
  • Relative: now, now-15m, now-1h, now-2d, now-1w, now-3mo, now-1y
    Units: s (seconds), m (minutes), h (hours), d (days), w (weeks), mo (months), y (years)
  • Unix timestamp: 1705315200000 (milliseconds since epoch)

Examples:
  --to now           # Current time (default)
  --to now-5m        # 5 minutes ago
  --to 1705318800000

Not supported:
  --to 2024-01-15T11:00:00Z  ❌ ISO8601 format not available for metrics"
    )]
    pub to: String,
}

/// Single time argument for commands that only need a start time (e.g., metrics list).
#[derive(Args, Debug, Clone)]
pub struct TimeFrom {
    /// Start time - supports relative (now-1h) or Unix ms (1705315200000). ISO8601 NOT supported for metrics.
    #[arg(
        short,
        long,
        default_value = "now-1h",
        long_help = "Start time for the query. Metrics active after this time will be listed.

⚠️  Note: Metrics commands do NOT support ISO8601 format.

Supported formats:
  • Relative: now, now-15m, now-1h, now-2d, now-1w, now-3mo, now-1y
    Units: s (seconds), m (minutes), h (hours), d (days), w (weeks), mo (months), y (years)
  • Unix timestamp: 1705315200000 (milliseconds since epoch)

Examples:
  --from now-30m     # 30 minutes ago
  --from now-1d      # 1 day ago
  --from 1705315200000

Not supported:
  --from 2024-01-15T10:00:00Z  ❌ ISO8601 format not available for metrics"
    )]
    pub from: String,
}

/// Pagination arguments for limiting query results.
#[derive(Args, Debug, Clone)]
pub struct Pagination {
    /// Maximum number of results to return (use 0 for unlimited)
    #[arg(
        short,
        long,
        default_value = "100",
        long_help = "Maximum number of results to return.

Set to 0 for unlimited results (use with caution on large datasets).

Examples:
  --limit 50         # Return up to 50 results
  --limit 1000       # Return up to 1000 results
  --limit 0          # Return all matching results (unlimited)"
    )]
    pub limit: u64,
}
