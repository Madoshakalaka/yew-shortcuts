use crate::fontawesome::{Icon, FONTAWESOME_LICENSE};
use yew::prelude::*;

#[cfg(feature = "full-svg")]
use crate::fontawesome::FULL_VIEW_BOX;

/// Properties for FontAwesomeSvg component
#[derive(Properties, PartialEq)]
pub struct FontAwesomeSvgProps {
    /// The icon to render
    pub icon: &'static Icon,

    /// Whether to use full SVG mode (when feature enabled)
    #[cfg(feature = "full-svg")]
    #[prop_or(false)]
    pub full: bool,

    /// Optional CSS classes to apply to the SVG element
    #[prop_or_default]
    pub classes: Classes,

    /// Optional inline style to apply to the SVG element
    #[prop_or_default]
    pub style: Option<String>,

    /// Optional onclick handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

/// FontAwesome SVG component for rendering icons
#[function_component]
pub fn FontAwesomeSvg(props: &FontAwesomeSvgProps) -> Html {
    #[cfg(not(feature = "full-svg"))]
    {
        let view_box = props.icon.cropped.view_box;
        let path_d = props.icon.cropped.d;

        html! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox={view_box}
                class={props.classes.clone()}
                style={props.style.clone()}
                onclick={props.onclick.clone()}
                data-fa-license={FONTAWESOME_LICENSE}
            >
                <path fill="currentColor" d={path_d} onclick={props.onclick.clone()} />
            </svg>
        }
    }

    #[cfg(feature = "full-svg")]
    {
        let (view_box, path_d) = if props.full {
            (FULL_VIEW_BOX, props.icon.full.d)
        } else {
            (props.icon.cropped.view_box, props.icon.cropped.d)
        };

        html! {
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox={view_box}
                class={props.classes.clone()}
                style={props.style.clone()}
                onclick={props.onclick.clone()}
                data-fa-license={FONTAWESOME_LICENSE}
            >
                <path fill="currentColor" d={path_d} onclick={props.onclick.clone()} />
            </svg>
        }
    }
}

