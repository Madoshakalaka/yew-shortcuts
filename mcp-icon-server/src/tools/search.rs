use anyhow::Result;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::icon_index::IconIndex;
use crate::types::{SearchIconInfo, SearchIconsParams, SearchIconsResult, IconInfo};

pub fn search_icons(index: &IconIndex, params: SearchIconsParams) -> Result<SearchIconsResult> {
    let matcher = SkimMatcherV2::default();
    let query = params.query.to_lowercase();

    // Score and filter icons
    let mut scored_icons: Vec<(i64, &IconInfo)> = index
        .icons
        .iter()
        .filter_map(|icon| {
            // Apply category filter if specified
            if let Some(ref category) = params.category {
                if icon.category != *category {
                    return None;
                }
            }

            // Try exact match first
            if icon.name.contains(&query) {
                // Boost score for exact substring matches
                return Some((1000 + (100 - icon.name.len() as i64), icon));
            }

            // Then try fuzzy matching
            if let Some(score) = matcher.fuzzy_match(&icon.name, &query) {
                Some((score, icon))
            } else {
                None
            }
        })
        .collect();

    // Sort by score (highest first)
    scored_icons.sort_by(|a, b| b.0.cmp(&a.0));

    let total_matches = scored_icons.len();
    
    // Return all matches with simplified data
    let icons: Vec<SearchIconInfo> = scored_icons
        .into_iter()
        .map(|(_, icon)| SearchIconInfo {
            name: icon.name.clone(),
            rust_name: icon.rust_name.clone(),
            category: icon.category.clone(),
            import_path: icon.import_path.clone(),
        })
        .collect();

    Ok(SearchIconsResult {
        icons,
        total_matches,
    })
}