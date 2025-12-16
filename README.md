# dd-search

A command-line tool for querying Datadog logs and APM spans. Outputs NDJSON for easy piping to `jq` or other tools.

## Installation

```bash
cargo build --release
cp target/release/dd-search /usr/local/bin/
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

## Usage

### Logs

```bash
dd-search logs <QUERY> [OPTIONS]
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
dd-search logs "service:api AND status:error"

# Search last 15 minutes with custom limit
dd-search logs "service:web @http.status_code:500" --from now-15m --limit 50

# Search specific indexes
dd-search logs "env:production" --indexes main,web

# Pipe to jq for filtering
dd-search logs "service:api" | jq '.attributes.message'
```

### Spans

```bash
dd-search spans <QUERY> [OPTIONS]
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
dd-search spans "service:web env:prod"

# Find slow spans
dd-search spans "service:api @duration:>1s" --limit 50

# Search with absolute time range (ISO8601)
dd-search spans "service:db" --from "2024-01-15T10:00:00Z" --to "2024-01-15T11:00:00Z"

# Search with Unix timestamp (milliseconds)
dd-search spans "service:api" --from "1705315200000" --to "1705318800000"
```

## Query Syntax

Queries use [Datadog's native query syntax](https://docs.datadoghq.com/logs/explorer/search_syntax/):

- **Attributes**: `@http.status_code:500`
- **Tags**: `env:production`
- **Wildcards**: `service:web*`
- **Ranges**: `@http.status_code:[200 TO 299]`
- **Boolean**: `service:api AND status:error`
- **Negation**: `-env:development`

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
  - `cli.rs` - Command-line interface definitions
  - `client/` - Datadog API client wrappers
  - `commands/` - Command implementations (logs, spans)
  - `config.rs` - Configuration loading
  - `error.rs` - Error types and exit codes
  - `output.rs` - NDJSON output writer
  - `time.rs` - Time range validation utilities
- `tests/` - Integration tests

## License

MIT
