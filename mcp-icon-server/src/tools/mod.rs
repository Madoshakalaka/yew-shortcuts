pub mod code_gen;
pub mod details;
pub mod search;

use crate::icon_index::IconIndex;
use crate::types::{CategoryInfo, ListCategoriesResult};

pub fn list_categories(index: &IconIndex) -> ListCategoriesResult {
    let categories = index
        .categories()
        .into_iter()
        .map(|(name, count)| CategoryInfo {
            name: name.to_string(),
            count,
        })
        .collect();

    ListCategoriesResult { categories }
}