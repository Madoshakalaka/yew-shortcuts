#!/usr/bin/env python3
"""Generate a list of all icon names for the demo."""

import re
from pathlib import Path

def main():
    fontawesome_rs = Path(__file__).parent.parent / "yew-shortcuts" / "src" / "fontawesome.rs"
    
    content = fontawesome_rs.read_text()
    
    # Find all icon definitions - updated pattern
    icon_pattern = r'pub const (\w+): &Icon = &Icon \{'
    
    solid_icons = []
    regular_icons = []
    brands_icons = []
    
    current_module = None
    in_icons_module = False
    
    for line in content.split('\n'):
        if 'pub mod icons {' in line:
            in_icons_module = True
        
        if in_icons_module:
            if 'pub mod solid {' in line:
                current_module = 'solid'
            elif 'pub mod regular {' in line:
                current_module = 'regular'
            elif 'pub mod brands {' in line:
                current_module = 'brands'
        
        if in_icons_module and current_module:
            match = re.search(icon_pattern, line)
            if match:
                icon_name = match.group(1)
                if current_module == 'solid':
                    solid_icons.append(icon_name)
                elif current_module == 'regular':
                    regular_icons.append(icon_name)
                elif current_module == 'brands':
                    brands_icons.append(icon_name)
    
    # Generate Rust code
    print(f"// Total icons: {len(solid_icons) + len(regular_icons) + len(brands_icons)}")
    print(f"// Solid: {len(solid_icons)}, Regular: {len(regular_icons)}, Brands: {len(brands_icons)}")
    print()
    
    print("const SOLID_ICONS: &[(&str, &Icon)] = &[")
    for icon in sorted(solid_icons):
        print(f'    ("{icon}", icons::solid::{icon}),')
    print("];")
    print()
    
    print("const REGULAR_ICONS: &[(&str, &Icon)] = &[")
    for icon in sorted(regular_icons):
        print(f'    ("{icon}", icons::regular::{icon}),')
    print("];")
    print()
    
    print("const BRANDS_ICONS: &[(&str, &Icon)] = &[")
    for icon in sorted(brands_icons):
        print(f'    ("{icon}", icons::brands::{icon}),')
    print("];")

if __name__ == "__main__":
    main()