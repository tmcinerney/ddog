# ddog

A command-line tool for querying Datadog logs, APM spans, and metrics. Outputs NDJSON for easy piping to `jq` or other tools.

## Installation

```bash
cargo build --release
cp target/release/ddog /usr/local/bin/
```

## Configuration

Set the following environment variables:

| Variable | Required | Description |
|----------|----------|-------------|
| `DD_API_KEY` | Yes | Datadog API key |
| `DD_APP_KEY` | Yes | Datadog application key |
| `DD_SITE` | No | Datadog site (default: `datadoghq.com`, use `datadoghq.eu` for EU) |

```bash
export DD_API_KEY="your-api-key"
export DD_APP_KEY="your-app-key"
```

### Required Permissions

Your application key must have the following scopes/permissions:

| Command | Required Scope | Description |
|---------|---------------|-------------|
| `logs search` | `logs_read_data` | Read log data |
| `spans search` | `apm_read` | Read APM span data |
| `metrics query` | `timeseries_query` | Query metrics timeseries data |
| `metrics list` | `metrics_read` | List available metrics |

**Note:** If you get a 403 Forbidden error, check that your application key has the required permissions in your Datadog account settings.

## Usage

### Logs

```bash
ddog logs search <QUERY> [OPTIONS]
```

**Options:**
- `-f, --from <TIME>` - Start time (default: `now-1h`)
  - **Relative**: `now`, `now-15m`, `now-1h`, `now-1d`, etc.
    - Units: `s` (seconds), `m` (minutes), `h` (hours), `d` (days), `w` (weeks), `mo` (months), `y` (years)
    - Examples: `now-30s`, `now-2h`, `now-1w`
  - **ISO8601**: `2024-01-15T10:00:00Z` or `2024-01-15T10:00:00+00:00`
  - **Unix timestamp**: Milliseconds since epoch (e.g., `1705315200000`)
- `-t, --to <TIME>` - End time (default: `now`)
  - Same formats as `--from`
- `-l, --limit <N>` - Max results (default: 100, use 0 for unlimited)
- `-i, --indexes <LIST>` - Log indexes to search (default: all)

**Examples:**

```bash
# Search for errors in the last hour
ddog logs search "service:api AND status:error"

# Search last 15 minutes with custom limit
ddog logs search "service:web @http.status_code:500" --from now-15m --limit 50

# Search specific indexes
ddog logs search "env:production" --indexes main,web

# Pipe to jq for filtering
ddog logs search "service:api" | jq '.attributes.message'
```

### Spans

```bash
ddog spans search <QUERY> [OPTIONS]
```

**Options:**
- `-f, --from <TIME>` - Start time (default: `now-1h`)
  - **Relative**: `now`, `now-15m`, `now-1h`, `now-1d`, etc.
    - Units: `s` (seconds), `m` (minutes), `h` (hours), `d` (days), `w` (weeks), `mo` (months), `y` (years)
    - Examples: `now-30s`, `now-2h`, `now-1w`
  - **ISO8601**: `2024-01-15T10:00:00Z` or `2024-01-15T10:00:00+00:00`
  - **Unix timestamp**: Milliseconds since epoch (e.g., `1705315200000`)
- `-t, --to <TIME>` - End time (default: `now`)
  - Same formats as `--from`
- `-l, --limit <N>` - Max results (default: 100, use 0 for unlimited)

**Examples:**

```bash
# Search spans by service
ddog spans search "service:web env:prod"

# Find slow spans
ddog spans search "service:api @duration:>1s" --limit 50

# Search with absolute time range (ISO8601)
ddog spans search "service:db" --from "2024-01-15T10:00:00Z" --to "2024-01-15T11:00:00Z"

# Search with Unix timestamp (milliseconds)
ddog spans search "service:api" --from "1705315200000" --to "1705318800000"
```

### Metrics

```bash
ddog metrics query <QUERY> [OPTIONS]
```

**Options:**
- `-f, --from <TIME>` - Start time (default: `now-1h`)
  - **Relative**: `now`, `now-15m`, `now-1h`, `now-1d`, etc.
  - **Unix timestamp**: Seconds or milliseconds since epoch (e.g., `1705315200` or `1705315200000`)
  - **Note**: ISO8601 format not yet supported for metrics
- `-t, --to <TIME>` - End time (default: `now`)
  - Same formats as `--from`
- `-l, --limit <N>` - Max data points (default: 1000, use 0 for unlimited)

**Examples:**

```bash
# Query average CPU usage over last hour
ddog metrics query "avg:system.cpu.user{*}" --from now-1h --to now

# Query with host filter
ddog metrics query "max:system.mem.used{host:prod-*}" --from now-15m --to now

# Query multiple metrics
ddog metrics query "avg:system.cpu.user{*},avg:system.cpu.system{*}" --from now-6h --to now

# Pipe to jq for processing
ddog metrics query "avg:redis.net.connections{*}" --from now-1d --to now | jq '.value'

# Filter by specific timestamp
ddog metrics query "avg:system.load.1{*}" | jq 'select(.timestamp > 1705315200)'

# Get average of all values
ddog metrics query "avg:system.cpu.idle{*}" --from now-1h | jq -s 'add / length | .value'
```

### List Metrics

```bash
ddog metrics list [OPTIONS]
```

**Options:**
- `-f, --from <TIME>` - Start time (default: `now-1h`)
  - Metrics active after this time will be listed
- `-t, --to <TIME>` - End time (optional, defaults to now)

**Examples:**

```bash
# List all metrics active in the last hour
ddog metrics list --from now-1h

# Find specific metrics with grep
ddog metrics list --from now-1h | grep "system.cpu"

# Count total active metrics
ddog metrics list --from now-1h | wc -l

# List and filter with jq
ddog metrics list --from now-1d | jq -r '.metric' | sort | uniq
```

## Query Syntax

### Logs and Spans

Queries use [Datadog's native query syntax](https://docs.datadoghq.com/logs/explorer/search_syntax/):

- **Attributes**: `@http.status_code:500`
- **Tags**: `env:production`
- **Wildcards**: `service:web*`
- **Ranges**: `@http.status_code:[200 TO 299]`
- **Boolean**: `service:api AND status:error`
- **Negation**: `-env:development`

### Metrics

Metrics queries use [Datadog's metric query syntax](https://docs.datadoghq.com/dashboards/querying/):

- **Basic**: `avg:system.cpu.user{*}` - Average CPU across all hosts
- **Aggregation**: `sum`, `avg`, `min`, `max`, `count`
- **Tag filtering**: `avg:system.cpu.user{env:prod}`
- **Multiple tags**: `avg:system.cpu.user{env:prod,service:web}`
- **Wildcards**: `avg:system.cpu.user{host:web-*}`
- **Arithmetic**: `avg:system.cpu.user{*} + avg:system.cpu.system{*}`
- **Functions**: `avg:system.cpu.user{*}.rollup(avg, 60)` - 60s rollup

## Output

Results are output as newline-delimited JSON (NDJSON), one record per line. This format works well with:

- `jq` for JSON filtering
- Line-oriented tools like `grep`, `head`, `tail`
- Streaming to files or other processes

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 2 | Authentication failure |
| 3 | API error |
| 4 | Invalid query |
| 5 | Configuration error |
| 6 | IO error |
| 7 | Serialization error |

## Development

### Setup

#### Pre-commit Hooks

This project uses [cargo-husky](https://github.com/rhysd/cargo-husky) for Rust-native git hooks. The hooks are automatically installed when you run `cargo test` for the first time.

To manually install the hooks:

```bash
# Run cargo test once to trigger hook installation
cargo test

# Or manually install the hook
cp .husky/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

The pre-commit hooks will:
- Check code formatting with `cargo fmt`
- Run clippy linter with strict warnings
- Run unit tests

#### GitHub Actions

CI/CD is configured via GitHub Actions:

**CI Workflow** (`.github/workflows/ci.yml`):
- **Check**: Formatting, clippy linting, and unit tests (runs first)
- **Build**: Release build (only after checks pass)

**Release Workflow** (`.github/workflows/release.yml`):
- Uses [release-please](https://github.com/googleapis/release-please) for automated releases
- Parses conventional commits to determine version bumps
- Creates release PRs with changelogs
- Builds and uploads binaries for Linux and macOS when releases are created

#### Conventional Commits

This project uses [Conventional Commits](https://www.conventionalcommits.org/) for automatic versioning:

| Commit Type | Example | Version Bump |
|-------------|---------|--------------|
| `fix:` | `fix: handle empty query` | Patch (0.0.X) |
| `feat:` | `feat: add trace search` | Minor (0.X.0) |
| `feat!:` or `BREAKING CHANGE:` | `feat!: change output format` | Major (X.0.0) |
| `docs:`, `chore:`, `refactor:` | `docs: update README` | No release |

**Examples:**
```bash
git commit -m "fix: correct time parsing for negative offsets"
git commit -m "feat: add support for trace ID lookup"
git commit -m "feat!: change default output to JSON Lines"
```

### Running Tests

```bash
# Run all unit tests
cargo test

# Run integration tests (requires DD_API_KEY and DD_APP_KEY)
cargo test --test integration_tests -- --ignored
```

### Project Structure

- `src/` - Main source code
  - `cli/` - Command-line interface definitions
    - `args.rs` - Main CLI structure and domain enum
    - `shared.rs` - Shared argument structures (TimeRange, Pagination)
    - `logs.rs`, `spans.rs`, `metrics.rs` - Domain-specific action enums
  - `client/` - Datadog API client wrappers
    - `logs.rs` - Logs API client
    - `spans.rs` - Spans API client
    - `metrics.rs` - Metrics API client
  - `commands/` - Command implementations organized by domain
    - `logs/search.rs` - Logs search command
    - `spans/search.rs` - Spans search command
    - `metrics/query.rs` - Metrics query command
    - `metrics/list.rs` - List metrics command
  - `config.rs` - Configuration loading
  - `error.rs` - Error types and exit codes
  - `output.rs` - NDJSON output writer
  - `time.rs` - Time parsing and validation utilities
- `tests/` - Integration tests

## License

MIT
