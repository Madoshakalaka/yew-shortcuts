#!/usr/bin/env python3
"""Generate Rust code for ALL FontAwesome icons."""

import os
import re
import xml.etree.ElementTree as ET
from pathlib import Path
from collections import defaultdict

def parse_svg(svg_path):
    """Parse an SVG file and extract viewBox and path data."""
    try:
        tree = ET.parse(svg_path)
        root = tree.getroot()
        
        # Get viewBox
        view_box = root.get('viewBox', '')
        
        # Find all path elements
        paths = []
        for path in root.findall('.//{http://www.w3.org/2000/svg}path'):
            d = path.get('d', '')
            if d:
                paths.append(d)
        
        # If no namespaced paths found, try without namespace
        if not paths:
            for path in root.findall('.//path'):
                d = path.get('d', '')
                if d:
                    paths.append(d)
        
        return view_box, paths
    except Exception as e:
        print(f"Error parsing {svg_path}: {e}")
        return None, []

def icon_name_from_path(svg_path):
    """Generate Rust-friendly icon name from SVG file path."""
    # Get filename without extension
    name = svg_path.stem
    
    # Convert kebab-case to UPPER_SNAKE_CASE
    # Handle special cases first
    name = name.replace('-', '_')
    name = name.upper()
    
    # Handle numeric prefixes (like "0", "1", etc.)
    if name[0].isdigit():
        name = f"NUM_{name}"
    
    # Return the name without style prefix since icons are already organized by module
    return name

def generate_rust_code(icons_by_category):
    """Generate Rust code for FontAwesome icons organized by category."""
    rust_code = """//! FontAwesome icons for Yew
//! 
//! This module provides ALL FontAwesome 6.7.2 Free icons as Yew components.
//! Icons are stored as constants to minimize WASM binary size.
//! 
//! # Example
//! 
//! ```rust
//! use yew::prelude::*;
//! use yew_shortcuts::fontawesome::{FontAwesomeSvg, icons};
//! 
//! #[function_component]
//! fn MyComponent() -> Html {
//!     html! {
//!         <FontAwesomeSvg icon={icons::solid::HOUSE} />
//!     }
//! }
//! ```

use yew::prelude::*;

/// Props for the FontAwesomeSvg component
#[derive(PartialEq, Properties)]
pub struct FontAwesomeSvgProps {
    /// The icon to display (use constants from the `icons` module)
    pub icon: &'static Icon,
    
    /// Additional CSS classes
    #[prop_or_default]
    pub classes: Classes,
    
    /// Height attribute
    #[prop_or_default]
    pub height: Option<&'static str>,
    
    /// Width attribute
    #[prop_or_default]
    pub width: Option<&'static str>,
    
    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    
    /// Inline style
    #[prop_or_default]
    pub style: String,
}

/// FontAwesome icon data
#[derive(Debug, PartialEq)]
pub struct Icon {
    /// SVG viewBox attribute
    pub view_box: &'static str,
    /// SVG path data
    pub d: &'static str,
}

/// FontAwesome SVG component
#[function_component]
pub fn FontAwesomeSvg(props: &FontAwesomeSvgProps) -> Html {
    html! {
        <svg
            style={props.style.clone()}
            aria-hidden="true"
            class={props.classes.clone()}
            width={props.width}
            height={props.height}
            focusable="false"
            role="img"
            xmlns="http://www.w3.org/2000/svg"
            viewBox={props.icon.view_box}
            onclick={props.onclick.clone()}
        >
            <path fill="currentColor" d={props.icon.d} />
        </svg>
    }
}

/// FontAwesome icon constants
pub mod icons {
    use super::Icon;
    
"""
    
    # Add solid icons
    if 'solid' in icons_by_category and icons_by_category['solid']:
        rust_code += "    /// Solid style icons\n"
        rust_code += "    pub mod solid {\n"
        rust_code += "        use super::Icon;\n        \n"
        
        for icon_name, (view_box, d_path) in sorted(icons_by_category['solid'].items()):
            # Escape the path data for Rust string
            d_escaped = d_path.replace('\\', '\\\\').replace('"', '\\"')
            rust_code += f'        pub const {icon_name}: &Icon = &Icon {{\n'
            rust_code += f'            view_box: "{view_box}",\n'
            rust_code += f'            d: "{d_escaped}",\n'
            rust_code += f'        }};\n        \n'
        
        rust_code += "    }\n    \n"
    
    # Add regular icons
    if 'regular' in icons_by_category and icons_by_category['regular']:
        rust_code += "    /// Regular style icons\n"
        rust_code += "    pub mod regular {\n"
        rust_code += "        use super::Icon;\n        \n"
        
        for icon_name, (view_box, d_path) in sorted(icons_by_category['regular'].items()):
            # Escape the path data for Rust string
            d_escaped = d_path.replace('\\', '\\\\').replace('"', '\\"')
            rust_code += f'        pub const {icon_name}: &Icon = &Icon {{\n'
            rust_code += f'            view_box: "{view_box}",\n'
            rust_code += f'            d: "{d_escaped}",\n'
            rust_code += f'        }};\n        \n'
        
        rust_code += "    }\n    \n"
    
    # Add brand icons
    if 'brands' in icons_by_category and icons_by_category['brands']:
        rust_code += "    /// Brand icons\n"
        rust_code += "    pub mod brands {\n"
        rust_code += "        use super::Icon;\n        \n"
        
        for icon_name, (view_box, d_path) in sorted(icons_by_category['brands'].items()):
            # Escape the path data for Rust string
            d_escaped = d_path.replace('\\', '\\\\').replace('"', '\\"')
            rust_code += f'        pub const {icon_name}: &Icon = &Icon {{\n'
            rust_code += f'            view_box: "{view_box}",\n'
            rust_code += f'            d: "{d_escaped}",\n'
            rust_code += f'        }};\n        \n'
        
        rust_code += "    }\n    \n"
    
    # Add compatibility re-exports for existing code
    rust_code += """    // Re-export commonly used icons at the root level for backward compatibility
    pub use solid::{
        PLUS as SOLID_PLUS,
        MINUS as SOLID_MINUS,
        ARROW_RIGHT as SOLID_ARROW_RIGHT,
        ROTATE as SOLID_ROTATE,
        CIRCLE_NODES as SOLID_CIRCLE_NODES,
        GEARS as SOLID_GEARS,
        COMMENTS as SOLID_COMMENTS,
        MAGNIFYING_GLASS as SOLID_MAGNIFYING_GLASS,
        MICROPHONE as SOLID_MICROPHONE,
        SHARE as SOLID_SHARE,
        COPY as SOLID_COPY,
    };
    
    pub use regular::{
        EYE as REGULAR_EYE,
        EYE_SLASH as REGULAR_EYE_SLASH,
        THUMBS_UP as REGULAR_THUMBS_UP,
        CIRCLE_QUESTION as REGULAR_CIRCLE_QUESTION,
        CLIPBOARD as REGULAR_CLIPBOARD,
    };
    
    pub use brands::{
        SQUARE_X_TWITTER as BRAND_SQUARE_X_TWITTER,
        FACEBOOK as BRAND_FACEBOOK,
        LINKEDIN as BRAND_LINKEDIN,
    };
"""
    
    rust_code += "}\n"
    
    return rust_code

def main():
    svg_base_dir = Path('/home/maa/Projects/yew-shortcuts/src/fontawesome-free-6.7.2-web')
    
    icons_by_category = {
        'solid': {},
        'regular': {},
        'brands': {}
    }
    
    total_icons = 0
    errors = []
    
    print("Parsing FontAwesome SVG files...")
    
    # Process each category
    for category in ['solid', 'regular', 'brands']:
        category_dir = svg_base_dir / category
        if not category_dir.exists():
            print(f"Warning: {category_dir} does not exist")
            continue
            
        print(f"\nProcessing {category} icons...")
        
        for svg_file in sorted(category_dir.glob('*.svg')):
            view_box, paths = parse_svg(svg_file)
            
            if view_box and paths:
                # Use the first path (most icons have only one)
                icon_name = icon_name_from_path(svg_file)
                
                icons_by_category[category][icon_name] = (view_box, paths[0])
                total_icons += 1
            else:
                errors.append(str(svg_file))
        
        print(f"  Processed {len(icons_by_category[category])} {category} icons")
    
    print(f"\nTotal icons processed: {total_icons}")
    
    if errors:
        print(f"\nErrors processing {len(errors)} files:")
        for error in errors[:10]:  # Show first 10 errors
            print(f"  - {error}")
        if len(errors) > 10:
            print(f"  ... and {len(errors) - 10} more")
    
    # Generate Rust code
    print("\nGenerating Rust code...")
    rust_code = generate_rust_code(icons_by_category)
    
    # Write to file
    output_file = Path('/home/maa/Projects/yew-shortcuts/yew-shortcuts/src/fontawesome.rs')
    with open(output_file, 'w') as f:
        f.write(rust_code)
    
    print(f"Generated {output_file}")
    print(f"File size: {len(rust_code) / 1024 / 1024:.2f} MB")

if __name__ == "__main__":
    main()