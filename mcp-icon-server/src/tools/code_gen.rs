use anyhow::{anyhow, Result};

use crate::icon_index::IconIndex;
use crate::types::{GetIconCodeParams, GetIconCodeResult, PropInfo};

pub fn get_icon_code(index: &IconIndex, params: GetIconCodeParams) -> Result<GetIconCodeResult> {
    // Find the icon
    let icon = index
        .find_icon(&params.name, &params.category)
        .ok_or_else(|| {
            anyhow!(
                "Icon '{}' not found in category '{}'",
                params.name,
                params.category
            )
        })?;

    // Generate import statement
    let import_statement = format!(
        "use yew_shortcuts::{{FontAwesomeSvg, {}}};",
        icon.import_path
    );

    // Generate the Yew component code showing both variants
    let code = format!(
        r#"// Default cropped version (always available):
html! {{
    <FontAwesomeSvg icon={{&{}}} />
}}

// Full SVG version (requires "full-svg" feature in Cargo.toml):
html! {{
    <FontAwesomeSvg icon={{&{}}} full=true />
}}"#,
        icon.import_path, icon.import_path
    );

    // Generate props list
    let props = vec![
        PropInfo {
            name: "icon".to_string(),
            prop_type: "&'static Icon".to_string(),
            required: true,
            default_value: None,
            description: "The icon to render".to_string(),
        },
        PropInfo {
            name: "full".to_string(),
            prop_type: "bool".to_string(),
            required: false,
            default_value: Some("false".to_string()),
            description: "Use full SVG with original viewBox (requires 'full-svg' feature)".to_string(),
        },
        PropInfo {
            name: "classes".to_string(),
            prop_type: "Classes".to_string(),
            required: false,
            default_value: Some("Classes::new()".to_string()),
            description: "CSS classes to apply to the SVG element".to_string(),
        },
        PropInfo {
            name: "style".to_string(),
            prop_type: "Option<String>".to_string(),
            required: false,
            default_value: Some("None".to_string()),
            description: "Inline style to apply to the SVG element".to_string(),
        },
        PropInfo {
            name: "onclick".to_string(),
            prop_type: "Option<Callback<MouseEvent>>".to_string(),
            required: false,
            default_value: Some("None".to_string()),
            description: "Click event handler".to_string(),
        },
    ];

    Ok(GetIconCodeResult {
        code,
        import_statement,
        viewbox_default: icon.view_box.clone(),
        viewbox_full: "0 0 640 640".to_string(), // Fixed value for full SVG
        props,
    })
}