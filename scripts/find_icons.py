#!/usr/bin/env python3
"""Find all FontAwesome icon usages in cognet and shiori projects."""

import re
import os
from pathlib import Path
from collections import defaultdict

def extract_fontawesome_usage(file_path):
    """Extract FontAwesome icon usage from a Rust file."""
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Pattern to match FontawesomeSvg component usage
    # Captures view_box and d attributes
    pattern = r'<FontawesomeSvg[^>]*?view_box\s*=\s*["{]([^"}]+)["}][^>]*?d\s*=\s*["{]([^"}]+)["}]'
    
    icons = []
    for match in re.finditer(pattern, content, re.DOTALL):
        view_box = match.group(1)
        d_path = match.group(2)
        
        # Try to find a comment or nearby text that might indicate the icon name
        # Look backwards from the match position for comments
        before_match = content[:match.start()]
        lines_before = before_match.split('\n')[-5:]  # Last 5 lines before match
        
        icon_hint = None
        for line in lines_before:
            if '//' in line:
                icon_hint = line.split('//')[-1].strip()
                break
        
        icons.append({
            'view_box': view_box,
            'd': d_path,
            'hint': icon_hint,
            'file': str(file_path),
            'line': content[:match.start()].count('\n') + 1
        })
    
    return icons

def find_all_icons():
    """Find all FontAwesome icons in cognet and shiori projects."""
    all_icons = []
    
    # Search in cognet
    cognet_dir = Path('/home/maa/Projects/cognet')
    if cognet_dir.exists():
        for rust_file in cognet_dir.rglob('*.rs'):
            icons = extract_fontawesome_usage(rust_file)
            if icons:
                all_icons.extend(icons)
    
    # Search in shiori
    shiori_dir = Path('/home/maa/Projects/shiori')
    if shiori_dir.exists():
        for rust_file in shiori_dir.rglob('*.rs'):
            icons = extract_fontawesome_usage(rust_file)
            if icons:
                all_icons.extend(icons)
    
    return all_icons

def deduplicate_icons(icons):
    """Deduplicate icons based on d attribute."""
    unique_icons = {}
    for icon in icons:
        d_path = icon['d']
        if d_path not in unique_icons:
            unique_icons[d_path] = icon
        else:
            # Update with better hint if available
            if icon['hint'] and not unique_icons[d_path]['hint']:
                unique_icons[d_path]['hint'] = icon['hint']
    
    return list(unique_icons.values())

def main():
    print("Finding all FontAwesome icon usages...")
    icons = find_all_icons()
    
    print(f"\nFound {len(icons)} total icon usages")
    
    unique_icons = deduplicate_icons(icons)
    print(f"Found {len(unique_icons)} unique icons")
    
    # Group by viewBox
    by_viewbox = defaultdict(list)
    for icon in unique_icons:
        by_viewbox[icon['view_box']].append(icon)
    
    print("\nIcons grouped by viewBox:")
    for viewbox, icons_list in sorted(by_viewbox.items()):
        print(f"\n{viewbox}: {len(icons_list)} icons")
        for icon in icons_list:
            hint = f" ({icon['hint']})" if icon['hint'] else ""
            file_short = icon['file'].replace('/home/maa/Projects/', '')
            print(f"  - {file_short}:{icon['line']}{hint}")
            print(f"    d: {icon['d'][:60]}...")
    
    # Write unique icons to a file for the codegen script
    output_file = Path('/home/maa/Projects/yew-shortcuts/scripts/found_icons.txt')
    with open(output_file, 'w') as f:
        for icon in unique_icons:
            f.write(f"{icon['view_box']}|{icon['d']}|{icon.get('hint', '')}|{icon['file']}\n")
    
    print(f"\nWrote {len(unique_icons)} unique icons to {output_file}")

if __name__ == "__main__":
    main()