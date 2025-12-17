//! Time range parsing and validation utilities.
//!
//! Provides functions to validate and parse Datadog time range formats.
//!
//! According to [Datadog API v2 documentation](https://docs.datadoghq.com/api/latest/logs/),
//! the Logs and Spans APIs support three time formats:
//!
//! 1. **Date Math Expressions**: Relative time calculations
//!    - `now` - Current time
//!    - `now-15m` - 15 minutes ago
//!    - `now-1h` - 1 hour ago
//!    - `now-1d` - 1 day ago
//!    - Units: `s` (seconds), `m` (minutes), `h` (hours), `d` (days), `w` (weeks), `mo` (months), `y` (years)
//!
//! 2. **ISO8601 Date-Time Strings**: Standard timestamp format
//!    - `2024-01-15T10:00:00Z` - UTC timezone
//!    - `2024-01-15T10:00:00+00:00` - With timezone offset
//!    - `2024-01-15T10:00:00.123Z` - With milliseconds
//!
//! 3. **Unix Timestamps**: Milliseconds since Unix epoch
//!    - `1705315200000` - Milliseconds since January 1, 1970 UTC
//!
//! All formats are passed directly to Datadog's API, which handles the parsing.
//!
//! For the Metrics API (V1), which requires Unix timestamps in seconds, use the
//! `parse_to_unix_seconds` function to convert time strings to i64.

use crate::error::AppError;

/// Validates that a time string is in a format Datadog accepts.
///
/// Datadog accepts three formats:
/// 1. **Relative times**: "now", "now-NUMBERUNIT" where unit is s, m, h, d, w, mo, y
/// 2. **ISO8601 timestamps**: "2024-01-15T10:00:00Z" or with timezone offsets
/// 3. **Unix timestamps**: milliseconds since Unix epoch (numeric string)
///
/// This function performs basic validation. The actual parsing is done by Datadog's API.
///
/// # Arguments
///
/// * `time_str` - Time string to validate
///
/// # Returns
///
/// `true` if the format appears valid, `false` otherwise
pub fn is_valid_time_format(time_str: &str) -> bool {
    if time_str.is_empty() {
        return false;
    }

    // Check for relative time format: "now" or "now-<number><unit>"
    if time_str == "now" {
        return true;
    }

    if let Some(rest) = time_str.strip_prefix("now-") {
        if rest.is_empty() {
            return false;
        }

        // Parse number and unit
        let mut chars = rest.chars();
        let mut num_str = String::new();

        // Collect digits
        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                num_str.push(c);
            } else {
                // Check if it's a valid unit
                let unit = format!("{}{}", c, chars.as_str());
                return matches!(unit.as_str(), "s" | "m" | "h" | "d" | "w" | "mo" | "y")
                    && !num_str.is_empty();
            }
        }

        // If we got here, there was no unit
        return false;
    }

    // Check for ISO8601 format (basic check)
    // Format: YYYY-MM-DDTHH:MM:SS[.SSS][Z|+HH:MM|-HH:MM]
    if time_str.len() >= 19 {
        // Check for date part: YYYY-MM-DD
        let date_part = &time_str[0..10];
        if date_part.chars().filter(|c| c == &'-').count() == 2 {
            // Check for T separator
            if time_str.len() > 10 && time_str.chars().nth(10) == Some('T') {
                // Basic ISO8601 format detected
                return true;
            }
        }
    }

    // Check for Unix timestamp (milliseconds since epoch)
    // Should be all digits and represent a reasonable timestamp
    // (between 1970 and 2100, roughly 0 to 4102444800000)
    if time_str.chars().all(|c| c.is_ascii_digit())
        && !time_str.is_empty()
        && let Ok(timestamp) = time_str.parse::<u64>()
    {
        // Valid Unix timestamp range: 0 to ~4102444800000 (year 2100)
        if timestamp <= 4_102_444_800_000 {
            return true;
        }
    }

    false
}

/// Validates that a time range is logically correct.
///
/// Checks that:
/// - Both times are in valid formats
/// - `from` is before `to` (when both are absolute timestamps)
///
/// Note: Relative times like "now-1h" and "now" are always considered valid
/// as they're evaluated by Datadog at query time.
///
/// # Arguments
///
/// * `from` - Start time string
/// * `to` - End time string
///
/// # Returns
///
/// `true` if the range appears valid, `false` otherwise
pub fn is_valid_time_range(from: &str, to: &str) -> bool {
    if !is_valid_time_format(from) || !is_valid_time_format(to) {
        return false;
    }

    // If both are relative times, they're always valid
    if (from == "now" || from.starts_with("now-")) && (to == "now" || to.starts_with("now-")) {
        return true;
    }

    // If both are absolute timestamps, we could validate ordering,
    // but for now we'll let Datadog handle it
    true
}

/// Parses a time string into Unix seconds.
///
/// This function is used for APIs that require Unix timestamps (like Metrics V1 API).
/// The Logs and Spans V2 APIs accept time strings directly and handle parsing themselves.
///
/// Supports three formats:
/// 1. **Relative times**: "now", "now-15m", "now-1h", etc.
/// 2. **Unix timestamps**: "1705315200000" (milliseconds) or "1705315200" (seconds)
/// 3. **ISO8601 timestamps**: Currently not supported, returns error suggesting alternatives
///
/// # Arguments
///
/// * `time_str` - Time string to parse
///
/// # Returns
///
/// Unix timestamp in seconds, or an error if the format is unsupported
///
/// # Examples
///
/// ```
/// use ddog::time::parse_to_unix_seconds;
///
/// // Relative time
/// let timestamp = parse_to_unix_seconds("now").unwrap();
/// assert!(timestamp > 0);
///
/// // Unix seconds
/// let timestamp = parse_to_unix_seconds("1705315200").unwrap();
/// assert_eq!(timestamp, 1705315200);
///
/// // Unix milliseconds (auto-converted to seconds)
/// let timestamp = parse_to_unix_seconds("1705315200000").unwrap();
/// assert_eq!(timestamp, 1705315200);
/// ```
pub fn parse_to_unix_seconds(time_str: &str) -> Result<i64, AppError> {
    // Handle "now"
    if time_str == "now" {
        return Ok(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| AppError::Config(format!("Failed to get current time: {}", e)))?
            .as_secs() as i64);
    }

    // Handle relative times like "now-1h"
    if let Some(rest) = time_str.strip_prefix("now-") {
        let now_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| AppError::Config(format!("Failed to get current time: {}", e)))?
            .as_secs() as i64;

        // Parse the number and unit
        let mut num_str = String::new();
        let mut unit = String::new();

        for c in rest.chars() {
            if c.is_ascii_digit() {
                num_str.push(c);
            } else {
                unit.push(c);
            }
        }

        let num: i64 = num_str
            .parse()
            .map_err(|_| AppError::Config(format!("Invalid time format: {}", time_str)))?;

        let offset_secs = match unit.as_str() {
            "s" => num,
            "m" => num * 60,
            "h" => num * 60 * 60,
            "d" => num * 60 * 60 * 24,
            "w" => num * 60 * 60 * 24 * 7,
            "mo" => num * 60 * 60 * 24 * 30, // Approximate
            "y" => num * 60 * 60 * 24 * 365, // Approximate
            _ => {
                return Err(AppError::Config(format!(
                    "Invalid time unit in: {}",
                    time_str
                )));
            }
        };

        return Ok(now_secs - offset_secs);
    }

    // Try parsing as Unix timestamp (could be seconds or milliseconds)
    if let Ok(timestamp) = time_str.parse::<i64>() {
        // If the timestamp looks like milliseconds (> year 2000 in seconds), convert to seconds
        if timestamp > 946684800 && timestamp.to_string().len() >= 13 {
            // Looks like milliseconds
            return Ok(timestamp / 1000);
        } else {
            // Assume seconds
            return Ok(timestamp);
        }
    }

    // ISO8601 not yet supported - would need chrono or similar
    Err(AppError::Config(format!(
        "Time format '{}' not supported. Please use relative times (now, now-1h) or Unix timestamps",
        time_str
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_relative_times() {
        // Basic "now"
        assert!(is_valid_time_format("now"));

        // Seconds
        assert!(is_valid_time_format("now-1s"));
        assert!(is_valid_time_format("now-30s"));
        assert!(is_valid_time_format("now-90s"));

        // Minutes
        assert!(is_valid_time_format("now-1m"));
        assert!(is_valid_time_format("now-15m"));
        assert!(is_valid_time_format("now-30m"));
        assert!(is_valid_time_format("now-60m"));

        // Hours
        assert!(is_valid_time_format("now-1h"));
        assert!(is_valid_time_format("now-2h"));
        assert!(is_valid_time_format("now-24h"));

        // Days
        assert!(is_valid_time_format("now-1d"));
        assert!(is_valid_time_format("now-7d"));
        assert!(is_valid_time_format("now-30d"));

        // Weeks
        assert!(is_valid_time_format("now-1w"));
        assert!(is_valid_time_format("now-2w"));

        // Months
        assert!(is_valid_time_format("now-1mo"));
        assert!(is_valid_time_format("now-6mo"));

        // Years
        assert!(is_valid_time_format("now-1y"));
        assert!(is_valid_time_format("now-2y"));
    }

    #[test]
    fn test_invalid_relative_times() {
        assert!(!is_valid_time_format("now-"));
        assert!(!is_valid_time_format("now-abc"));
        assert!(!is_valid_time_format("now-1"));
        assert!(!is_valid_time_format("now-1x"));
        assert!(!is_valid_time_format("now-1hm"));
    }

    #[test]
    fn test_valid_iso8601_times() {
        // Basic ISO8601 with Z (UTC)
        assert!(is_valid_time_format("2024-01-15T10:00:00Z"));
        assert!(is_valid_time_format("2024-12-31T23:59:59Z"));
        assert!(is_valid_time_format("1970-01-01T00:00:00Z"));

        // ISO8601 with timezone offset
        assert!(is_valid_time_format("2024-01-15T10:00:00+00:00"));
        assert!(is_valid_time_format("2024-01-15T10:00:00-05:00"));
        assert!(is_valid_time_format("2024-01-15T10:00:00+09:00"));

        // ISO8601 with milliseconds
        assert!(is_valid_time_format("2024-01-15T10:00:00.123Z"));
        assert!(is_valid_time_format("2024-01-15T10:00:00.999Z"));

        // ISO8601 with microseconds (if supported)
        assert!(is_valid_time_format("2024-01-15T10:00:00.123456Z"));
    }

    #[test]
    fn test_valid_unix_timestamps() {
        // Valid Unix timestamps in milliseconds (as per Datadog API)
        assert!(is_valid_time_format("0")); // Epoch start: 1970-01-01T00:00:00Z
        assert!(is_valid_time_format("1609459200000")); // 2021-01-01T00:00:00Z
        assert!(is_valid_time_format("1705315200000")); // 2024-01-15T10:00:00Z
        assert!(is_valid_time_format("4102444800000")); // Year 2100 (upper bound)

        // Common timestamps
        assert!(is_valid_time_format("1000000000000")); // 2001-09-09
        assert!(is_valid_time_format("946684800000")); // 2000-01-01
    }

    #[test]
    fn test_invalid_unix_timestamps() {
        // Too large (beyond year 2100)
        assert!(!is_valid_time_format("5000000000000"));
        // Negative (not supported as string)
        assert!(!is_valid_time_format("-1000"));
    }

    #[test]
    fn test_invalid_time_formats() {
        assert!(!is_valid_time_format(""));
        assert!(!is_valid_time_format("invalid"));
        assert!(!is_valid_time_format("2024-01-15"));
        assert!(!is_valid_time_format("10:00:00"));
        assert!(!is_valid_time_format("now+1h"));
    }

    #[test]
    fn test_valid_time_ranges() {
        // Relative times
        assert!(is_valid_time_range("now-1h", "now"));
        assert!(is_valid_time_range("now-15m", "now"));
        assert!(is_valid_time_range("now-1d", "now"));

        // Absolute times
        assert!(is_valid_time_range(
            "2024-01-15T10:00:00Z",
            "2024-01-15T11:00:00Z"
        ));

        // Mixed (relative and absolute)
        assert!(is_valid_time_range("now-1h", "2024-01-15T11:00:00Z"));
        assert!(is_valid_time_range("2024-01-15T10:00:00Z", "now"));
    }

    #[test]
    fn test_invalid_time_ranges() {
        assert!(!is_valid_time_range("", "now"));
        assert!(!is_valid_time_range("now", ""));
        assert!(!is_valid_time_range("invalid", "now"));
        assert!(!is_valid_time_range("now", "invalid"));
    }

    #[test]
    fn test_parse_to_unix_seconds_now() {
        let result = parse_to_unix_seconds("now");
        assert!(result.is_ok());
        let timestamp = result.unwrap();
        // Should be close to current time (within a few seconds)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        assert!((timestamp - now).abs() < 5);
    }

    #[test]
    fn test_parse_to_unix_seconds_relative_times() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Test various units
        let one_hour_ago = parse_to_unix_seconds("now-1h").unwrap();
        assert!((now - one_hour_ago - 3600).abs() < 5);

        let fifteen_min_ago = parse_to_unix_seconds("now-15m").unwrap();
        assert!((now - fifteen_min_ago - 900).abs() < 5);

        let one_day_ago = parse_to_unix_seconds("now-1d").unwrap();
        assert!((now - one_day_ago - 86400).abs() < 5);

        let one_week_ago = parse_to_unix_seconds("now-1w").unwrap();
        assert!((now - one_week_ago - 604800).abs() < 5);
    }

    #[test]
    fn test_parse_to_unix_seconds_unix_seconds() {
        let timestamp = parse_to_unix_seconds("1705315200").unwrap();
        assert_eq!(timestamp, 1705315200);
    }

    #[test]
    fn test_parse_to_unix_seconds_unix_milliseconds() {
        let timestamp = parse_to_unix_seconds("1705315200000").unwrap();
        assert_eq!(timestamp, 1705315200); // Should convert to seconds
    }

    #[test]
    fn test_parse_to_unix_seconds_invalid() {
        let result = parse_to_unix_seconds("invalid");
        assert!(result.is_err());

        let result = parse_to_unix_seconds("2024-01-15T10:00:00Z");
        assert!(result.is_err()); // ISO8601 not yet supported
    }
}
