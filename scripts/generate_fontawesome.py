#!/usr/bin/env python3
"""Generate Rust code for FontAwesome icons."""

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

def find_matching_svg(d_path, svg_dir):
    """Find SVG file that contains the given path data."""
    for svg_file in Path(svg_dir).rglob('*.svg'):
        view_box, paths = parse_svg(svg_file)
        if d_path in paths:
            return svg_file, view_box
    return None, None

def icon_name_from_path(svg_path):
    """Generate Rust-friendly icon name from SVG file path."""
    # Get filename without extension
    name = svg_path.stem
    
    # Get icon style (solid, regular, brands)
    style = svg_path.parent.name
    
    # Convert kebab-case to UPPER_SNAKE_CASE
    name = name.upper().replace('-', '_')
    
    # Add style prefix
    if style == 'solid':
        return f"SOLID_{name}"
    elif style == 'regular':
        return f"REGULAR_{name}"
    elif style == 'brands':
        return f"BRAND_{name}"
    else:
        return name

def generate_rust_code(icons_data):
    """Generate Rust code for FontAwesome icons."""
    rust_code = """//! FontAwesome icons for Yew
//! 
//! This module provides FontAwesome icons as Yew components.
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
//!         <FontAwesomeSvg icon={icons::SOLID_PLUS} />
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
    
    # Add icon constants
    for icon_name, (view_box, d_path) in sorted(icons_data.items()):
        # Escape the path data for Rust string
        d_escaped = d_path.replace('"', r'\"')
        rust_code += f'    pub const {icon_name}: &Icon = &Icon {{\n'
        rust_code += f'        view_box: "{view_box}",\n'
        rust_code += f'        d: "{d_escaped}",\n'
        rust_code += f'    }};\n\n'
    
    rust_code += "}\n"
    
    return rust_code

def main():
    # Read found icons
    found_icons_file = Path('/home/maa/Projects/yew-shortcuts/scripts/found_icons.txt')
    svg_base_dir = Path('/home/maa/Projects/yew-shortcuts/src/fontawesome-free-6.7.2-web')
    
    icons_data = {}
    icon_mapping = {}
    
    print("Matching icons with SVG files...")
    
    with open(found_icons_file, 'r') as f:
        for line in f:
            parts = line.strip().split('|')
            if len(parts) >= 2:
                view_box = parts[0]
                d_path = parts[1]
                hint = parts[2] if len(parts) > 2 else ''
                
                # Try to find matching SVG
                svg_file, found_view_box = find_matching_svg(d_path, svg_base_dir)
                
                if svg_file:
                    icon_name = icon_name_from_path(svg_file)
                    icons_data[icon_name] = (view_box, d_path)
                    icon_mapping[d_path] = icon_name
                    print(f"  Found: {icon_name} <- {svg_file.relative_to(svg_base_dir)}")
                else:
                    print(f"  WARNING: No matching SVG found for icon with hint: {hint}")
                    # Generate a generic name
                    generic_name = f"ICON_{len(icons_data)}"
                    icons_data[generic_name] = (view_box, d_path)
                    icon_mapping[d_path] = generic_name
    
    print(f"\nMatched {len(icons_data)} icons")
    
    # Generate Rust code
    rust_code = generate_rust_code(icons_data)
    
    # Write to file
    output_file = Path('/home/maa/Projects/yew-shortcuts/src/fontawesome.rs')
    with open(output_file, 'w') as f:
        f.write(rust_code)
    
    print(f"Generated {output_file}")
    
    # Write mapping file for migration
    mapping_file = Path('/home/maa/Projects/yew-shortcuts/scripts/icon_mapping.txt')
    with open(mapping_file, 'w') as f:
        for d_path, icon_name in icon_mapping.items():
            f.write(f"{d_path}|{icon_name}\n")
    
    print(f"Generated mapping file: {mapping_file}")

if __name__ == "__main__":
    main()