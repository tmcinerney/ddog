//! Configuration loading from environment variables.
//!
//! Validates that required Datadog credentials are set before creating
//! the API client configuration.

use datadog_api_client::datadog::Configuration;

use crate::error::AppError;

/// Loads and validates Datadog configuration from environment variables.
///
/// # Required Environment Variables
///
/// - `DD_API_KEY` - Datadog API key
/// - `DD_APP_KEY` - Datadog application key
///
/// # Optional Environment Variables
///
/// - `DD_SITE` - Datadog site (defaults to `datadoghq.com`)
///
/// # Errors
///
/// Returns `AppError::Config` if required environment variables are missing or empty.
pub fn load_config() -> Result<Configuration, AppError> {
    let api_key = std::env::var("DD_API_KEY")
        .map_err(|_| AppError::Config("DD_API_KEY environment variable not set".into()))?;

    let app_key = std::env::var("DD_APP_KEY")
        .map_err(|_| AppError::Config("DD_APP_KEY environment variable not set".into()))?;

    if api_key.is_empty() {
        return Err(AppError::Config("DD_API_KEY is empty".into()));
    }
    if app_key.is_empty() {
        return Err(AppError::Config("DD_APP_KEY is empty".into()));
    }

    // DD_SITE is optional - the SDK reads it automatically
    // Defaults to datadoghq.com if not set

    Ok(Configuration::new())
}
