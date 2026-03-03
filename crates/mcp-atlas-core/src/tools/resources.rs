//! MCP resource discovery and reading.

use std::sync::Arc;

use anyhow::Result;
use serde_json::{Value, json};

use super::search;
use crate::server::AppState;

/// Read an MCP resource by URI.
pub fn read_resource(state: &Arc<AppState>, uri: &str) -> Result<Value> {
    match uri {
        "cncf://landscape/overview" => {
            let stats = search::handle_get_stats(state)?;
            Ok(json!({
                "contents": [{
                    "uri": uri,
                    "mimeType": "text/markdown",
                    "text": stats["content"][0]["text"]
                }]
            }))
        }
        _ if uri.starts_with("cncf://projects/") => {
            let name = uri.strip_prefix("cncf://projects/").unwrap_or("");
            let args = json!({ "name": name });
            let result = search::handle_get_project(state, &args)?;
            Ok(json!({
                "contents": [{
                    "uri": uri,
                    "mimeType": "application/json",
                    "text": result["content"][0]["text"]
                }]
            }))
        }
        _ if uri.starts_with("cncf://categories/") => {
            let result = search::handle_list_categories(state)?;
            Ok(json!({
                "contents": [{
                    "uri": uri,
                    "mimeType": "text/markdown",
                    "text": result["content"][0]["text"]
                }]
            }))
        }
        _ => anyhow::bail!("Unknown resource URI: {uri}"),
    }
}

/// List available MCP resources.
pub fn resources_list() -> Value {
    json!({
        "resources": [
            {
                "uri": "cncf://landscape/overview",
                "name": "CNCF Landscape Overview",
                "description": "High-level statistics about the CNCF landscape",
                "mimeType": "text/markdown"
            },
            {
                "uri": "cncf://categories/all",
                "name": "CNCF Categories",
                "description": "All landscape categories and subcategories",
                "mimeType": "text/markdown"
            }
        ]
    })
}

/// List MCP resource templates (URI patterns with placeholders).
pub fn resource_templates_list() -> Value {
    json!({
        "resourceTemplates": [
            {
                "uriTemplate": "cncf://projects/{name}",
                "name": "CNCF Project Details",
                "description": "Get details for a specific CNCF project by name",
                "mimeType": "application/json"
            },
            {
                "uriTemplate": "cncf://categories/{category}",
                "name": "CNCF Category",
                "description": "Get projects in a specific landscape category",
                "mimeType": "text/markdown"
            }
        ]
    })
}
