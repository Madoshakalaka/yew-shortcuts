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

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("pub const ") && trimmed.contains(": &Icon = &Icon {") {
            total_icons += 1;
        }
    }

    // Generate a constants file for the demo
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("icon_counts.rs");

    let generated = format!(
        "/// Auto-generated icon counts from build.rs\n\
         pub const TOTAL_ICONS: usize = {};\n",
        total_icons
    );

    fs::write(&dest_path, generated).expect("Failed to write icon counts");

    eprintln!("Counted {} total icons", total_icons);
}