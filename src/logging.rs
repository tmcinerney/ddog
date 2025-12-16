//! Verbose logging utilities.
//!
//! Provides functions for verbose/debug output when the --verbose flag is enabled.

/// Logger for verbose output.
///
/// Writes to stderr to avoid interfering with NDJSON output on stdout.
pub struct VerboseLogger {
    enabled: bool,
}

impl VerboseLogger {
    /// Creates a new logger.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether verbose logging is enabled
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Logs a message if verbose mode is enabled.
    pub fn log(&self, message: &str) {
        if self.enabled {
            eprintln!("[DEBUG] {}", message);
        }
    }

    /// Constructs and logs a Datadog UI URL for viewing logs/spans.
    ///
    /// # Arguments
    ///
    /// * `resource_type` - Either "logs" or "spans"
    /// * `query` - The search query
    /// * `from` - Start time
    /// * `to` - End time
    /// * `site` - Datadog site (e.g., "datadoghq.com" or "datadoghq.eu")
    pub fn log_datadog_url(
        &self,
        resource_type: &str,
        query: &str,
        from: &str,
        to: &str,
        site: &str,
    ) {
        if !self.enabled {
            return;
        }

        let query_param = urlencoding::encode(query);
        
        let base_url = if site == "datadoghq.com" {
            "https://app.datadoghq.com"
        } else if site == "datadoghq.eu" {
            "https://app.datadoghq.eu"
        } else {
            &format!("https://app.{}", site)
        };

        // For Datadog UI, we need to convert times to milliseconds since epoch
        // For relative times like "now-1h", we'll approximate or note that user needs to adjust
        let (from_ts, to_ts) = self.convert_times_for_url(from, to);
        
        let url = match resource_type {
            "logs" => format!(
                "{}/logs?query={}&from_ts={}&to_ts={}&live=false",
                base_url, query_param, from_ts, to_ts
            ),
            "spans" => format!(
                "{}/apm/traces?query={}&from_ts={}&to_ts={}",
                base_url, query_param, from_ts, to_ts
            ),
            _ => return,
        };

        self.log(&format!("Datadog UI URL: {}", url));
        if from.starts_with("now") || to.starts_with("now") {
            self.log("Note: URL uses approximate timestamps. Adjust time range in UI if needed.");
        }
    }

    /// Converts time strings to Unix timestamps in milliseconds for URL construction.
    fn convert_times_for_url(&self, from: &str, to: &str) -> (String, String) {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let from_ts = if from == "now" {
            now_ms.to_string()
        } else if from.starts_with("now-") {
            // Approximate relative time (this is a simplified conversion)
            // For better accuracy, we'd need to parse the full relative time syntax
            let offset_ms = self.parse_relative_time(&from[4..]);
            (now_ms.saturating_sub(offset_ms)).to_string()
        } else if from.chars().all(|c| c.is_ascii_digit()) {
            // Already a Unix timestamp
            from.to_string()
        } else {
            // ISO8601 or other format - approximate to now-1h for URL
            // User can adjust in UI
            (now_ms - 3600000).to_string()
        };

        let to_ts = if to == "now" {
            now_ms.to_string()
        } else if to.starts_with("now-") {
            let offset_ms = self.parse_relative_time(&to[4..]);
            (now_ms.saturating_sub(offset_ms)).to_string()
        } else if to.chars().all(|c| c.is_ascii_digit()) {
            to.to_string()
        } else {
            now_ms.to_string()
        };

        (from_ts, to_ts)
    }

    /// Parses relative time string (e.g., "1h", "30m") to milliseconds.
    fn parse_relative_time(&self, time_str: &str) -> u64 {
        // Simple parser for common formats
        if time_str.ends_with('s') {
            if let Ok(secs) = time_str[..time_str.len()-1].parse::<u64>() {
                return secs * 1000;
            }
        } else if time_str.ends_with('m') {
            if let Ok(mins) = time_str[..time_str.len()-1].parse::<u64>() {
                return mins * 60 * 1000;
            }
        } else if time_str.ends_with('h') {
            if let Ok(hours) = time_str[..time_str.len()-1].parse::<u64>() {
                return hours * 3600 * 1000;
            }
        } else if time_str.ends_with('d') {
            if let Ok(days) = time_str[..time_str.len()-1].parse::<u64>() {
                return days * 24 * 3600 * 1000;
            }
        }
        // Default to 1 hour if we can't parse
        3600000
    }

    /// Logs request details.
    pub fn log_request(&self, resource_type: &str, query: &str, from: &str, to: &str) {
        if !self.enabled {
            return;
        }

        self.log(&format!("Resource type: {}", resource_type));
        self.log(&format!("Query: {}", query));
        self.log(&format!("Time range: {} to {}", from, to));
    }

    /// Logs API endpoint information.
    pub fn log_api_endpoint(&self, endpoint: &str, method: &str) {
        if self.enabled {
            self.log(&format!("API {} {}", method, endpoint));
        }
    }

    /// Logs configuration information (without sensitive data).
    pub fn log_config(&self, site: &str, has_api_key: bool, has_app_key: bool) {
        if !self.enabled {
            return;
        }

        self.log(&format!("Datadog site: {}", site));
        self.log(&format!("API key: {}", if has_api_key { "set" } else { "not set" }));
        self.log(&format!("App key: {}", if has_app_key { "set" } else { "not set" }));
    }

    /// Logs error details with context.
    pub fn log_error(&self, error: &str, context: &str) {
        if self.enabled {
            self.log(&format!("Error: {} (context: {})", error, context));
        }
    }
}

impl Default for VerboseLogger {
    fn default() -> Self {
        Self::new(false)
    }
}

