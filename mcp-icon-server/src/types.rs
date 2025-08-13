use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconInfo {
    pub name: String,
    pub rust_name: String,
    pub category: String,
    pub width: u32,
    pub height: u32,
    pub view_box: String,
    pub path_data: String,
    pub import_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIconInfo {
    pub name: String,
    pub rust_name: String,
    pub category: String,
    pub import_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchIconsParams {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchIconsResult {
    pub icons: Vec<SearchIconInfo>,
    pub total_matches: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIconCodeParams {
    pub name: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIconCodeResult {
    pub code: String,
    pub import_statement: String,
    pub viewbox_default: String,
    pub viewbox_full: String,
    pub props: Vec<PropInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropInfo {
    pub name: String,
    pub prop_type: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIconDetailsParams {
    pub name: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetIconDetailsResult {
    pub icon: IconInfo,
    pub yew_component_example: String,
    pub raw_svg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCategoriesResult {
    pub categories: Vec<CategoryInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub name: String,
    pub count: usize,
}