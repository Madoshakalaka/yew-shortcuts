use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

use crate::types::IconInfo;

// Include the generated fontawesome.rs file content at compile time
const FONTAWESOME_RS: &str = include_str!("../../yew-shortcuts/src/fontawesome.rs");

#[derive(Clone)]
pub struct IconIndex {
    pub icons: Vec<IconInfo>,
    pub by_name_category: HashMap<(String, String), usize>, // (name, category) -> index
}

impl IconIndex {
    pub fn load() -> Result<Self> {
        let icons = parse_fontawesome_file()?;
        
        // Build lookup index
        let mut by_name_category = HashMap::new();
        for (idx, icon) in icons.iter().enumerate() {
            by_name_category.insert((icon.name.clone(), icon.category.clone()), idx);
        }

        Ok(Self {
            icons,
            by_name_category,
        })
    }

    pub fn find_icon(&self, name: &str, category: &str) -> Option<&IconInfo> {
        self.by_name_category
            .get(&(name.to_string(), category.to_string()))
            .and_then(|&idx| self.icons.get(idx))
    }

    pub fn categories(&self) -> Vec<(&str, usize)> {
        let mut category_counts: HashMap<&str, usize> = HashMap::new();
        for icon in &self.icons {
            *category_counts.entry(&icon.category).or_insert(0) += 1;
        }
        let mut result: Vec<_> = category_counts.into_iter().collect();
        result.sort_by_key(|(cat, _)| *cat);
        result
    }
}

fn parse_fontawesome_file() -> Result<Vec<IconInfo>> {
    // Use the state machine approach directly as it's more reliable
    parse_with_state_machine()
}

fn parse_with_state_machine() -> Result<Vec<IconInfo>> {
    let mut icons = Vec::new();
    let lines: Vec<&str> = FONTAWESOME_RS.lines().collect();
    let mut i = 0;
    let mut current_module = String::new();

    while i < lines.len() {
        let line = lines[i];

        // Check for module declaration
        if line.contains("pub mod") && line.contains("{") {
            if let Some(caps) = Regex::new(r"pub mod (\w+)")?.captures(line) {
                current_module = caps[1].to_string();
            }
        }

        // Check for icon constant  
        if line.contains("pub const") && line.contains(": &Icon") && !current_module.is_empty() {
            // Extract icon name
            let icon_name = if let Some(caps) = Regex::new(r"pub const (\w+):")?.captures(line) {
                caps[1].to_string()
            } else {
                i += 1;
                continue;
            };

            // Look for viewBox in next few lines
            let mut view_box = String::new();
            let mut path_data = String::new();
            
            for j in i..std::cmp::min(i + 20, lines.len()) {
                if lines[j].contains("view_box:") {
                    if let Some(caps) = Regex::new(r#"view_box: "([^"]+)""#)?.captures(lines[j]) {
                        view_box = caps[1].to_string();
                    }
                }
                if lines[j].contains("d: r#") {
                    // Path data might span multiple lines
                    // Find start of raw string
                    if let Some(start_idx) = lines[j].find("r#\"") {
                        let start_line = &lines[j][start_idx + 3..];
                        // Check if it ends on same line
                        if let Some(end_idx) = start_line.find("\"#") {
                            path_data = start_line[..end_idx].to_string();
                        } else {
                            // Multi-line path data
                            let mut path_parts = vec![start_line];
                            for k in (j + 1)..std::cmp::min(j + 10, lines.len()) {
                                if let Some(end_idx) = lines[k].find("\"#") {
                                    path_parts.push(&lines[k][..end_idx]);
                                    path_data = path_parts.join("");
                                    break;
                                } else {
                                    path_parts.push(lines[k].trim());
                                }
                            }
                        }
                    }
                }
                
                if !view_box.is_empty() && !path_data.is_empty() {
                    break;
                }
            }

            if !view_box.is_empty() && !path_data.is_empty() {
                let (width, height) = parse_view_box(&view_box)?;
                let name = rust_name_to_kebab_case(&icon_name);
                let import_path = format!("icons::{}::{}", current_module, icon_name);

                icons.push(IconInfo {
                    name,
                    rust_name: icon_name,
                    category: current_module.clone(),
                    width,
                    height,
                    view_box,
                    path_data,
                    import_path,
                });
            }
        }

        i += 1;
    }

    Ok(icons)
}

fn parse_view_box(view_box: &str) -> Result<(u32, u32)> {
    let parts: Vec<&str> = view_box.split_whitespace().collect();
    if parts.len() != 4 {
        anyhow::bail!("Invalid viewBox format: {}", view_box);
    }
    
    let width = parts[2].parse::<u32>()?;
    let height = parts[3].parse::<u32>()?;
    
    Ok((width, height))
}

fn rust_name_to_kebab_case(rust_name: &str) -> String {
    rust_name
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect()
}