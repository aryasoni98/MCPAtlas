//! Hybrid search: Reciprocal Rank Fusion (RRF) for merging BM25 and vector results.

use std::collections::HashMap;

use mcp_atlas_data::models::ProjectSummary;

/// Default RRF constant (k=60 is common in literature).
pub const RRF_K: u32 = 60;

/// Reciprocal Rank Fusion: merge two ranked lists by name.
/// Score for each item = sum of 1/(k + rank) across all lists.
/// Returns merged names sorted by combined score descending.
pub fn reciprocal_rank_fusion(
    bm25: &[ProjectSummary],
    vector: &[(String, f64)],
    k: u32,
) -> Vec<String> {
    let mut scores: HashMap<String, f64> = HashMap::new();
    for (rank_1based, p) in bm25.iter().enumerate() {
        let r = (rank_1based + 1) as u32;
        *scores.entry(p.name.clone()).or_insert(0.0) += 1.0 / (k + r) as f64;
    }
    for (rank_1based, (name, _)) in vector.iter().enumerate() {
        let r = (rank_1based + 1) as u32;
        *scores.entry(name.clone()).or_insert(0.0) += 1.0 / (k + r) as f64;
    }
    let mut order: Vec<(String, f64)> = scores.into_iter().collect();
    order.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    order.into_iter().map(|(name, _)| name).collect()
}
