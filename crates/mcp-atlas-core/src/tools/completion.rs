//! MCP completion handling — auto-complete for tool arguments.

use std::sync::Arc;

use serde_json::{Value, json};

use super::args;
use crate::server::AppState;

/// Handle MCP completion requests — provide auto-complete suggestions for tool arguments.
pub fn handle_completion(state: &Arc<AppState>, ref_obj: &Value, argument: &Value) -> Value {
    let arg_name = args::parse_string_arg(argument, "name", "");
    let arg_value = args::parse_string_arg(argument, "value", "");

    let suggestions: Vec<String> = match arg_name.as_str() {
        // Project name arguments — fuzzy match against all project names
        "project" | "name" | "tool_name" | "from" | "to" | "from_project" | "to_project" => {
            let lower = arg_value.to_lowercase();
            let mut names: Vec<String> = state
                .projects
                .iter()
                .filter(|p| p.name.to_lowercase().contains(&lower))
                .map(|p| p.name.clone())
                .collect();
            names.sort();
            names.dedup();
            names.into_iter().take(20).collect()
        }
        // Language arguments — complete from known languages in the dataset
        "language" => {
            let lower = arg_value.to_lowercase();
            let mut langs: Vec<String> = state
                .projects
                .iter()
                .filter_map(|p| p.github.as_ref().and_then(|g| g.language.as_ref()).cloned())
                .filter(|l| !l.is_empty() && l.to_lowercase().contains(&lower))
                .collect();
            langs.sort();
            langs.dedup();
            langs.into_iter().take(20).collect()
        }
        // Category arguments
        "category" => {
            let lower = arg_value.to_lowercase();
            let mut cats: Vec<String> = state
                .projects
                .iter()
                .map(|p| p.category.clone())
                .filter(|c| !c.is_empty() && c.to_lowercase().contains(&lower))
                .collect();
            cats.sort();
            cats.dedup();
            cats.into_iter().take(20).collect()
        }
        // Maturity filter
        "maturity" => {
            vec!["sandbox".into(), "incubating".into(), "graduated".into()]
        }
        // Relation type filter
        "relation" => {
            vec![
                "alternative".into(),
                "integrates".into(),
                "component".into(),
                "extends".into(),
                "supersedes".into(),
            ]
        }
        _ => {
            // Check if the ref is a resource template — complete project names for URI
            let ref_type = args::parse_string_arg(ref_obj, "type", "");
            if ref_type.as_str() == "ref/resource" {
                let lower = arg_value.to_lowercase();
                state
                    .projects
                    .iter()
                    .filter(|p| p.name.to_lowercase().contains(&lower))
                    .take(20)
                    .map(|p| p.name.clone())
                    .collect()
            } else {
                vec![]
            }
        }
    };

    json!({
        "completion": {
            "values": suggestions,
            "hasMore": false,
            "total": suggestions.len()
        }
    })
}
