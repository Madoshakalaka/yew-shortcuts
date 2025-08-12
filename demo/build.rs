use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Tell cargo to rerun this build script if fontawesome.rs changes
    println!("cargo:rerun-if-changed=../yew-shortcuts/src/fontawesome.rs");
    
    // Read the fontawesome.rs file
    let fontawesome_path = Path::new("../yew-shortcuts/src/fontawesome.rs");
    let content = fs::read_to_string(fontawesome_path)
        .expect("Failed to read fontawesome.rs");
    
    // Count icons by looking for "pub const" patterns in the icons module
    let mut total_icons = 0;
    let mut solid_icons = 0;
    let mut regular_icons = 0;
    let mut brands_icons = 0;
    
    // Track which module we're in
    let mut current_module = "";
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Check for module declarations
        if trimmed.starts_with("pub mod solid {") {
            current_module = "solid";
        } else if trimmed.starts_with("pub mod regular {") {
            current_module = "regular";
        } else if trimmed.starts_with("pub mod brands {") {
            current_module = "brands";
        }
        
        // Count icon constants
        if trimmed.starts_with("pub const ") && trimmed.contains(": &Icon = &Icon {") {
            total_icons += 1;
            match current_module {
                "solid" => solid_icons += 1,
                "regular" => regular_icons += 1,
                "brands" => brands_icons += 1,
                _ => {}
            }
        }
    }
    
    // Generate a constants file for the demo
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("icon_counts.rs");
    
    let generated = format!(
        "/// Auto-generated icon counts from build.rs\n\
         pub const TOTAL_ICONS: usize = {};\n\
         pub const SOLID_ICONS_COUNT: usize = {};\n\
         pub const REGULAR_ICONS_COUNT: usize = {};\n\
         pub const BRANDS_ICONS_COUNT: usize = {};\n",
        total_icons, solid_icons, regular_icons, brands_icons
    );
    
    fs::write(&dest_path, generated).expect("Failed to write icon counts");
    
    eprintln!("Counted {} total icons ({} solid, {} regular, {} brands)", 
             total_icons, solid_icons, regular_icons, brands_icons);
}