//! Metrics domain command actions.

use clap::Subcommand;

use super::shared::{TimeFrom, TimeRangeRelativeOnly};

/// Available actions for the metrics domain.
#[derive(Subcommand, Debug)]
pub enum MetricsAction {
    /// Query metrics timeseries data
    #[command(
        long_about = "Query metrics timeseries data using Datadog's metric query syntax.

⚠️  Time Format Limitation:
  Metrics queries do NOT support ISO8601 timestamps.
  Use relative times (now-1h) or Unix timestamps only.

Query Syntax:
  • Basic: avg:system.cpu.user{*}
  • Aggregation: sum, avg, min, max, count
  • Tag filtering: avg:system.cpu.user{env:prod}
  • Multiple tags: avg:system.cpu.user{env:prod,service:web}
  • Wildcards: avg:system.cpu.user{host:web-*}
  • Arithmetic: avg:system.cpu.user{*} + avg:system.cpu.system{*}
  • Functions: avg:system.cpu.user{*}.rollup(avg, 60)

Output Format:
  Each line contains a JSON object with timestamp and metric value.
  Pipe to jq for processing: ddog metrics query \"...\" | jq '.value'

Examples:
  # Query CPU usage
  ddog metrics query \"avg:system.cpu.user{*}\" --from now-1h

  # Query with host filter
  ddog metrics query \"max:system.mem.used{host:prod-*}\"

  # Multiple metrics
  ddog metrics query \"avg:system.cpu.user{*},avg:system.cpu.system{*}\"

  # With arithmetic
  ddog metrics query \"avg:system.cpu.user{*} + avg:system.cpu.system{*}\"

  # Get average value with jq
  ddog metrics query \"avg:system.cpu.idle{*}\" | jq -s 'add / length | .value'

Documentation:
  https://docs.datadoghq.com/dashboards/querying/"
    )]
    Query {
        /// Datadog metric query (e.g., "avg:system.cpu.user{*}")
        #[arg(long_help = "Datadog metric query using Datadog's metric query syntax.

Format: <aggregation>:<metric_name>{<tag_filters>}[.<function>]

Supports:
  • Aggregations: avg, sum, min, max, count
  • Tag filtering: {env:prod}, {env:prod,service:web}
  • Wildcards: {host:web-*}, {service:*}
  • Arithmetic: metric1 + metric2, metric1 - metric2
  • Functions: .rollup(avg, 60), .fill(null)

Examples:
  \"avg:system.cpu.user{*}\"
  \"max:system.mem.used{host:prod-*}\"
  \"sum:redis.net.connections{env:prod,cluster:main}\"
  \"avg:system.cpu.user{*} + avg:system.cpu.system{*}\"
  \"avg:system.load.1{*}.rollup(avg, 60)\"")]
        query: String,

        #[command(flatten)]
        time_range: TimeRangeRelativeOnly,

        /// Maximum number of data points to return (use 0 for unlimited)
        #[arg(
            short,
            long,
            default_value = "1000",
            long_help = "Maximum number of data points to return.

Set to 0 for unlimited results. Note that Datadog may still apply
its own limits based on the time range and metric resolution.

Examples:
  --limit 100        # Return up to 100 data points
  --limit 5000       # Return up to 5000 data points
  --limit 0          # Return all available data points"
        )]
        limit: u64,
    },

    /// List active metrics within a time window
    #[command(long_about = "List active metrics within a time window.

Lists all metrics that were actively reporting data during the specified
time range. Useful for discovering available metrics or finding metrics
by pattern.

⚠️  Time Format Limitation:
  Metrics commands do NOT support ISO8601 timestamps.
  Use relative times (now-1h) or Unix timestamps only.

Output Format:
  Each line contains a JSON object with metric name and metadata.
  Use grep to filter: ddog metrics list | grep \"system.cpu\"

Examples:
  # List metrics active in the last hour
  ddog metrics list --from now-1h

  # Find CPU metrics
  ddog metrics list | grep \"system.cpu\"

  # Count active metrics
  ddog metrics list | wc -l

  # Extract metric names with jq
  ddog metrics list | jq -r '.metric' | sort | uniq

Note: The Datadog API only accepts a start time (--from). The --to option
is not available for this command as the API returns all metrics active after
the specified start time.")]
    List {
        #[command(flatten)]
        time_from: TimeFrom,
    },
}
