//! Load and validate a custom webhook payload from a file.

use anyhow::{Context, Result};
use std::fs;

/// Loads a JSON payload from a given file path.
///
/// This ensures the file is readable and the contents are valid JSON.
/// The raw JSON string is returned for signing and sending.
pub async fn load_from_file(path: &str) -> Result<String> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("Failed to read payload file at {}", path))?;

    // Validate it's valid JSON
    serde_json::from_str::<serde_json::Value>(&contents)
        .with_context(|| "Invalid JSON format in payload file")?;

    Ok(contents)
}
