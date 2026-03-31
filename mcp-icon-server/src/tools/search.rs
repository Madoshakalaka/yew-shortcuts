use anyhow::Result;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::icon_index::IconIndex;
use crate::types::{SearchIconInfo, SearchIconsParams, SearchIconsResult, IconInfo};

pub fn search_icons(index: &IconIndex, params: SearchIconsParams) -> Result<SearchIconsResult> {
    let matcher = SkimMatcherV2::default();
    let query = params.query.to_lowercase();
    let words: Vec<&str> = query.split_whitespace().collect();
    let hyphenated_query = query.replace(' ', "-");

    let mut scored_icons: Vec<(i64, &IconInfo)> = index
        .icons
        .iter()
        .filter_map(|icon| {
            if let Some(ref category) = params.category {
                if icon.category != *category {
                    return None;
                }
            }

            let len_bonus = 100 - icon.name.len() as i64;

            if icon.name.contains(&hyphenated_query) {
                return Some((2000 + len_bonus, icon));
            }

            let mut total_score: i64 = 0;
            let mut matched_any = false;

            for word in &words {
                if icon.name.contains(word) {
                    total_score += 500 + len_bonus;
                    matched_any = true;
                } else if let Some(score) = matcher.fuzzy_match(&icon.name, word) {
                    total_score += score;
                    matched_any = true;
                }
            }

            if matched_any {
                Some((total_score, icon))
            } else {
                None
            }
        })
        .collect();

    // Sort by score (highest first)
    scored_icons.sort_by_key(|b| std::cmp::Reverse(b.0));

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