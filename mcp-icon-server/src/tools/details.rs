use anyhow::{anyhow, Result};

use crate::icon_index::IconIndex;
use crate::types::{GetIconDetailsParams, GetIconDetailsResult};

pub fn get_icon_details(
    index: &IconIndex,
    params: GetIconDetailsParams,
) -> Result<GetIconDetailsResult> {
    // Find the icon
    let icon = index
        .find_icon(&params.name, &params.category)
        .ok_or_else(|| {
            anyhow!(
                "Icon '{}' not found in category '{}'",
                params.name,
                params.category
            )
        })?
        .clone();

    // Generate a complete Yew component example
    let yew_component_example = format!(
        r#"use yew::prelude::*;
use yew_shortcuts::{{FontAwesomeSvg, {}}};

#[function_component]
pub fn MyIconComponent() -> Html {{
    html! {{
        <div class="icon-container">
            <FontAwesomeSvg 
                icon={{&{}}}
                classes={{classes!("text-blue-500", "w-6", "h-6")}}
                style={{"color: currentColor"}}
            />
        </div>
    }}
}}"#,
        icon.import_path, icon.import_path
    );

    // Generate raw SVG code
    let raw_svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{}">
    <path fill="currentColor" d="{}" />
</svg>"#,
        icon.view_box, icon.path_data
    );

    Ok(GetIconDetailsResult {
        icon,
        yew_component_example,
        raw_svg,
    })
}