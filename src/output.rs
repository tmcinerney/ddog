//! NDJSON (Newline Delimited JSON) output writer.
//!
//! Provides streaming output of JSON records, one per line, suitable for
//! piping to tools like `jq` or processing line-by-line.

use serde::Serialize;
use std::io::{self, BufWriter, Stdout, Write};

/// Writes JSON records as newline-delimited JSON (NDJSON) to stdout.
///
/// Each record is serialized as compact JSON followed by a newline.
/// Output is flushed after each record for real-time streaming.
pub struct NdjsonWriter {
    writer: BufWriter<Stdout>,
}

impl NdjsonWriter {
    /// Creates a new NDJSON writer to stdout.
    pub fn new() -> Self {
        Self {
            writer: BufWriter::new(io::stdout()),
        }
    }

    /// Writes a single record as JSON followed by a newline.
    ///
    /// The output is flushed immediately to support real-time streaming.
    pub fn write<T: Serialize>(&mut self, record: &T) -> io::Result<()> {
        serde_json::to_writer(&mut self.writer, record)?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()
    }
}

impl Default for NdjsonWriter {
    fn default() -> Self {
        Self::new()
    }
}
