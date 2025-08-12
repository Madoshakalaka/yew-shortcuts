use roxmltree::Document;
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct IconData {
    name: String,
    category: String,
    view_box: String,
    path_data: String,
    comment: String,
    file_path: String,
}

fn snake_case_to_upper(s: &str) -> String {
    let mut result = s.to_uppercase().replace('-', "_");
    // Ensure identifiers don't start with a number
    if result.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        result = format!("ICON_{}", result);
    }
    result
}

fn process_svg_file(path: &Path) -> Result<IconData, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    
    // Extract the comment (it's between <!-- and -->)
    let comment = if let Some(start) = content.find("<!--") {
        if let Some(end) = content.find("-->") {
            content[start + 4..end].trim().to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    
    let doc = Document::parse(&content)?;
    
    let root = doc.root_element();
    let svg = root
        .descendants()
        .find(|n| n.tag_name().name() == "svg")
        .ok_or("No SVG element found")?;
    
    let view_box = svg
        .attribute("viewBox")
        .ok_or("No viewBox attribute")?
        .to_string();
    
    let path_elem = svg
        .descendants()
        .find(|n| n.tag_name().name() == "path")
        .ok_or("No path element found")?;
    
    let path_data = path_elem
        .attribute("d")
        .ok_or("No d attribute in path")?
        .to_string();
    
    let file_name = path
        .file_stem()
        .ok_or("No file stem")?
        .to_str()
        .ok_or("Invalid UTF-8 in filename")?;
    
    let category = path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .ok_or("Cannot determine category")?;
    
    let name = snake_case_to_upper(file_name);
    
    Ok(IconData {
        name,
        category: category.to_string(),
        view_box,
        path_data,
        comment,
        file_path: path.display().to_string(),
    })
}

fn process_icons_directory(dir: &Path) -> (Vec<IconData>, String) {
    let mut icons = Vec::new();
    let mut license_comment = None;
    
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "svg"))
    {
        match process_svg_file(entry.path()) {
            Ok(icon_data) => {
                // Check that all icons have the same license comment
                if let Some(ref expected_comment) = license_comment {
                    if icon_data.comment != *expected_comment {
                        eprintln!(
                            "Warning: Different comment in {}: '{}' vs expected '{}'",
                            icon_data.file_path, icon_data.comment, expected_comment
                        );
                    }
                } else {
                    license_comment = Some(icon_data.comment.clone());
                }
                
                icons.push(icon_data);
            }
            Err(e) => {
                eprintln!("Error processing {}: {}", entry.path().display(), e);
            }
        }
    }
    
    let license = license_comment.unwrap_or_default();
    (icons, license)
}

fn generate_rust_module(
    cropped_icons: &[IconData],
    full_icons: &[IconData],
    license: &str,
) -> String {
    let mut output = String::new();
    
    // Add header
    output.push_str("// This file is auto-generated. Do not edit manually.\n\n");
    
    // Add the shared license constant
    output.push_str(&format!("/// FontAwesome license comment shared by all icons\n"));
    output.push_str(&format!("pub const FONTAWESOME_LICENSE: &str = r#\"{}\"#;\n\n", license));
    
    // CroppedIcon struct for cropped icons
    output.push_str("/// Represents a cropped FontAwesome icon with its SVG path data\n");
    output.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
    output.push_str("pub struct CroppedIcon {\n");
    output.push_str("    /// The SVG viewBox attribute value\n");
    output.push_str("    pub view_box: &'static str,\n");
    output.push_str("    /// The SVG path data (d attribute)\n");
    output.push_str("    pub d: &'static str,\n");
    output.push_str("}\n\n");

    // FullIcon struct for full SVGs (with standard viewBox)
    output.push_str("/// Represents a full FontAwesome icon with standard viewBox\n");
    output.push_str("#[cfg(feature = \"full-svg\")]\n");
    output.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
    output.push_str("pub struct FullIcon {\n");
    output.push_str("    /// The SVG path data (d attribute)\n");
    output.push_str("    pub d: &'static str,\n");
    output.push_str("}\n\n");
    
    output.push_str("/// Standard viewBox for full SVG icons\n");
    output.push_str("#[cfg(feature = \"full-svg\")]\n");
    output.push_str("pub const FULL_VIEW_BOX: &str = \"0 0 640 640\";\n\n");
    
    // Icon struct that contains both
    output.push_str("/// Represents a FontAwesome icon that can have both cropped and full variants\n");
    output.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
    output.push_str("pub struct Icon {\n");
    output.push_str("    /// The cropped version of the icon\n");
    output.push_str("    pub cropped: CroppedIcon,\n");
    output.push_str("    /// The full version of the icon (when feature enabled)\n");
    output.push_str("    #[cfg(feature = \"full-svg\")]\n");
    output.push_str("    pub full: FullIcon,\n");
    output.push_str("}\n\n");
    
    // Group icons by category
    let mut cropped_by_category: BTreeMap<String, Vec<&IconData>> = BTreeMap::new();
    for icon in cropped_icons {
        cropped_by_category.entry(icon.category.clone()).or_default().push(icon);
    }
    
    let mut full_by_category: BTreeMap<String, Vec<&IconData>> = BTreeMap::new();
    for icon in full_icons {
        full_by_category.entry(icon.category.clone()).or_default().push(icon);
    }
    
    // Generate module for each category
    for (category, icons) in &cropped_by_category {
        output.push_str(&format!("/// {} icons\n", category));
        output.push_str(&format!("pub mod {} {{\n", category));
        output.push_str("    use super::{Icon, CroppedIcon};\n");
        output.push_str("    #[cfg(feature = \"full-svg\")]\n");
        output.push_str("    use super::FullIcon;\n\n");
        
        // Sort icons by name for consistent output
        let mut sorted_icons = icons.clone();
        sorted_icons.sort_by_key(|i| &i.name);
        
        // Get corresponding full icons
        let full_category_icons = full_by_category.get(category);
        
        for icon in sorted_icons {
            let icon_name = &icon.name;
            
            // Find corresponding full icon if available
            let full_icon = full_category_icons
                .and_then(|full_icons| full_icons.iter().find(|fi| fi.name == icon.name));
            
            output.push_str(&format!("    /// {} icon\n", icon_name));
            
            if full_icon.is_some() {
                // Generate Icon with both cropped and full
                output.push_str(&format!("    pub const {}: &Icon = &Icon {{\n", icon_name));
                output.push_str(&format!("        cropped: CroppedIcon {{\n"));
                output.push_str(&format!("            view_box: \"{}\",\n", icon.view_box));
                output.push_str(&format!("            d: r#\"{}\"#,\n", icon.path_data));
                output.push_str(&format!("        }},\n"));
                output.push_str(&format!("        #[cfg(feature = \"full-svg\")]\n"));
                output.push_str(&format!("        full: FullIcon {{\n"));
                output.push_str(&format!("            d: r#\"{}\"#,\n", full_icon.unwrap().path_data));
                output.push_str(&format!("        }},\n"));
                output.push_str("    };\n\n");
            } else {
                // Generate Icon with only cropped (shouldn't happen if icons match)
                output.push_str(&format!("    pub const {}: &Icon = &Icon {{\n", icon_name));
                output.push_str(&format!("        cropped: CroppedIcon {{\n"));
                output.push_str(&format!("            view_box: \"{}\",\n", icon.view_box));
                output.push_str(&format!("            d: r#\"{}\"#,\n", icon.path_data));
                output.push_str(&format!("        }},\n"));
                output.push_str("    };\n\n");
            }
        }
        
        output.push_str("}\n\n");
    }
    
    // Add icons module that re-exports all categories
    output.push_str("/// Module containing all FontAwesome icons organized by category\n");
    output.push_str("pub mod icons {\n");
    for category in cropped_by_category.keys() {
        output.push_str(&format!("    pub use super::{};\n", category));
    }
    output.push_str("}\n\n");
    
    output
}

fn main() {
    let cropped_dir = Path::new("../svgs-7");
    let full_dir = Path::new("../svgs-full-7");
    let output_path = Path::new("../yew-shortcuts/src/fontawesome.rs");
    
    // Process cropped icons
    println!("Processing cropped icons from {}...", cropped_dir.display());
    let (cropped_icons, license) = process_icons_directory(cropped_dir);
    
    // Process full icons
    println!("Processing full icons from {}...", full_dir.display());
    let (full_icons, _) = process_icons_directory(full_dir);
    
    // Count icons by category
    let mut cropped_counts: BTreeMap<String, usize> = BTreeMap::new();
    for icon in &cropped_icons {
        *cropped_counts.entry(icon.category.clone()).or_default() += 1;
    }
    
    let mut full_counts: BTreeMap<String, usize> = BTreeMap::new();
    for icon in &full_icons {
        *full_counts.entry(icon.category.clone()).or_default() += 1;
    }
    
    println!("\nCropped icons found:");
    for (category, count) in &cropped_counts {
        println!("  {}: {} icons", category, count);
    }
    println!("  Total: {} icons", cropped_icons.len());
    
    println!("\nFull icons found:");
    for (category, count) in &full_counts {
        println!("  {}: {} icons", category, count);
    }
    println!("  Total: {} icons", full_icons.len());
    
    // Generate Rust module
    println!("\nGenerating Rust module...");
    let rust_code = generate_rust_module(&cropped_icons, &full_icons, &license);
    
    // Write to file
    let mut file = fs::File::create(output_path).expect("Failed to create output file");
    file.write_all(rust_code.as_bytes())
        .expect("Failed to write to output file");
    
    println!("Successfully generated {}", output_path.display());
}