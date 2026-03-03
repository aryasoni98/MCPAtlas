//! Argument extraction helpers for MCP tool and resource handlers.
//! Reduces boilerplate from repeated `args.get("key").and_then(|v| v.as_X()).unwrap_or(default)`.

use serde_json::Value;

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
