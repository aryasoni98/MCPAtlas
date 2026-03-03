//! Argument extraction helpers for MCP tool and resource handlers.
//! Reduces boilerplate from repeated `args.get("key").and_then(|v| v.as_X()).unwrap_or(default)`.
//!
//! Input validation: query/use_case strings are bounded (MAX_QUERY_LEN) to prevent abuse.

use serde_json::Value;

/// Maximum length (bytes) for search queries and use-case descriptions.
pub const MAX_QUERY_LEN: usize = 1024;

/// Maximum length (bytes) for project names and other identifier-like strings.
pub const MAX_PROJECT_NAME_LEN: usize = 256;

/// Maximum number of projects in a single compare_projects call.
pub const MAX_COMPARE_PROJECTS: usize = 20;

/// Maximum `limit` for paginated results (search, find_alternatives, etc.).
pub const MAX_LIMIT: usize = 100;

/// Maximum `offset` for paginated results.
pub const MAX_OFFSET: usize = 10_000;

/// Validate string length. Returns `Err` with a message if `s.len() > max`.
#[inline]
pub fn validate_string_len(s: &str, max: usize) -> Result<(), String> {
    if s.len() <= max {
        Ok(())
    } else {
        Err(format!("Input exceeds maximum length of {max} bytes"))
    }
}

/// Extract a string argument with a default.
#[inline]
pub fn parse_string_arg(args: &Value, key: &str, default: &str) -> String {
    args.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(default)
        .to_string()
}

/// Extract an optional string argument.
#[inline]
pub fn parse_optional_str<'a>(args: &'a Value, key: &str) -> Option<&'a str> {
    args.get(key).and_then(|v| v.as_str())
}

/// Extract a u64 argument with a default.
#[inline]
#[allow(dead_code)]
pub fn parse_u64_arg(args: &Value, key: &str, default: u64) -> u64 {
    args.get(key).and_then(|v| v.as_u64()).unwrap_or(default)
}

/// Extract a u64 argument as usize with a default.
#[inline]
pub fn parse_usize_arg(args: &Value, key: &str, default: usize) -> usize {
    args.get(key)
        .and_then(|v| v.as_u64())
        .unwrap_or(default as u64) as usize
}

/// Extract an optional u64 argument.
#[inline]
pub fn parse_optional_u64(args: &Value, key: &str) -> Option<u64> {
    args.get(key).and_then(|v| v.as_u64())
}
