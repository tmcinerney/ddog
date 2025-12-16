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
- `-t, --to <TIME>` - End time (default: `now`)
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
- `-t, --to <TIME>` - End time (default: `now`)
- `-l, --limit <N>` - Max results (default: 100, use 0 for unlimited)

**Examples:**

```bash
# Search spans by service
dd-search spans "service:web env:prod"

# Find slow spans
dd-search spans "service:api @duration:>1s" --limit 50

# Search with absolute time range
dd-search spans "service:db" --from "2024-01-15T10:00:00Z" --to "2024-01-15T11:00:00Z"
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

## License

MIT
