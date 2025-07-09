#!/usr/bin/env python3
"""Add missing icons to fontawesome.rs"""

MISSING_ICONS = {
    # Facebook icon
    ("BRAND_FACEBOOK", "0 0 512 512", "M512 256C512 114.6 397.4 0 256 0S0 114.6 0 256C0 376 82.7 476.8 194.2 504.5V334.2H141.4V256h52.8V222.3c0-87.1 39.4-127.5 125-127.5c16.2 0 44.2 3.2 55.7 6.4V172c-6-.6-16.5-1-29.6-1c-42 0-58.2 15.9-58.2 57.2V256h83.6l-14.4 78.2H287V510.1C413.8 494.8 512 386.9 512 256h0z"),
    
    # LinkedIn icon
    ("BRAND_LINKEDIN", "0 0 448 512", "M416 32H31.9C14.3 32 0 46.5 0 64.3v383.4C0 465.5 14.3 480 31.9 480H416c17.6 0 32-14.5 32-32.3V64.3c0-17.8-14.4-32.3-32-32.3zM135.4 416H69V202.2h66.5V416zm-33.2-243c-21.3 0-38.5-17.3-38.5-38.5S80.9 96 102.2 96c21.2 0 38.5 17.3 38.5 38.5 0 21.3-17.2 38.5-38.5 38.5zm282.1 243h-66.4V312c0-24.8-.5-56.7-34.5-56.7-34.6 0-39.9 27-39.9 54.9V416h-66.4V202.2h63.7v29.2h.9c8.9-16.8 30.6-34.5 62.9-34.5 67.2 0 79.7 44.3 79.7 101.9V416z"),
    
    # Copy icon
    ("SOLID_COPY", "0 0 448 512", "M208 0L332.1 0c12.7 0 24.9 5.1 33.9 14.1l67.9 67.9c9 9 14.1 21.2 14.1 33.9L448 336c0 26.5-21.5 48-48 48l-192 0c-26.5 0-48-21.5-48-48l0-288c0-26.5 21.5-48 48-48zM48 128l80 0 0 64-64 0 0 256 192 0 0-32 64 0 0 48c0 26.5-21.5 48-48 48L48 512c-26.5 0-48-21.5-48-48L0 176c0-26.5 21.5-48 48-48z"),
    
    # Clipboard icon
    ("REGULAR_CLIPBOARD", "0 0 384 512", "M280 64l40 0c35.3 0 64 28.7 64 64l0 320c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 128C0 92.7 28.7 64 64 64l40 0 9.6 0C121 27.5 153.3 0 192 0s71 27.5 78.4 64l9.6 0zM64 112c-8.8 0-16 7.2-16 16l0 320c0 8.8 7.2 16 16 16l256 0c8.8 0 16-7.2 16-16l0-320c0-8.8-7.2-16-16-16l-16 0 0 24c0 13.3-10.7 24-24 24l-88 0-88 0c-13.3 0-24-10.7-24-24l0-24-16 0zm128-8a24 24 0 1 0 0-48 24 24 0 1 0 0 48z"),
}

def main():
    # Read current file
    with open('/home/maa/Projects/yew-shortcuts/yew-shortcuts/src/fontawesome.rs', 'r') as f:
        content = f.read()
    
    # Find where to insert new icons (before the closing brace of the icons module)
    icons_end = content.rfind('}')
    
    # Generate new icon code
    new_icons = ""
    for name, view_box, d_path in MISSING_ICONS:
        d_escaped = d_path.replace('"', r'\"')
        new_icons += f'\n    pub const {name}: &Icon = &Icon {{\n'
        new_icons += f'        view_box: "{view_box}",\n'
        new_icons += f'        d: "{d_escaped}",\n'
        new_icons += f'    }};\n'
    
    # Insert new icons
    new_content = content[:icons_end] + new_icons + content[icons_end:]
    
    # Write back
    with open('/home/maa/Projects/yew-shortcuts/yew-shortcuts/src/fontawesome.rs', 'w') as f:
        f.write(new_content)
    
    print(f"Added {len(MISSING_ICONS)} missing icons")

if __name__ == "__main__":
    main()