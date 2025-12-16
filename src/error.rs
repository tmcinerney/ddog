//! Application error types and exit codes.
//!
//! Provides a unified error type for the application with appropriate
//! exit codes for different failure modes.

use std::io;
use thiserror::Error;

/// Application error type covering all failure modes.
///
/// Each variant maps to a specific exit code for scripting compatibility.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl AppError {
    /// Returns the exit code for this error type.
    ///
    /// Exit codes:
    /// - 2: Authentication failure (401/403)
    /// - 3: API error
    /// - 4: Invalid query syntax
    /// - 5: Configuration error
    /// - 6: IO error
    /// - 7: Serialization error
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::Auth(_) => 2,
            AppError::Api(_) => 3,
            AppError::InvalidQuery(_) => 4,
            AppError::Config(_) => 5,
            AppError::Io(_) => 6,
            AppError::Serialization(_) => 7,
        }
    }
}
