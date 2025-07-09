use stylist::{css, yew::Global};
use yew::prelude::*;
use yew_shortcuts::fontawesome::{self, FontAwesomeSvg, icons};

#[function_component]
pub fn App() -> Html {
    let search_query = use_state(String::new);
    let copied_icon = use_state(|| None::<String>);

    let on_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
        })
    };

    let copy_to_clipboard = {
        let copied_icon = copied_icon.clone();
        Callback::from(move |icon_name: String| {
            let window = web_sys::window().unwrap();
            let navigator = window.navigator();
            let clipboard = navigator.clipboard();
            
            let code = format!("use yew_shortcuts::fontawesome::icons::{};\n\n<FontAwesomeSvg icon={{&{}}} />", &icon_name, &icon_name);
            
            let promise = clipboard.write_text(&code);
            
            copied_icon.set(Some(icon_name));
            
            let copied_icon = copied_icon.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                gloo::timers::future::TimeoutFuture::new(2000).await;
                copied_icon.set(None);
            });
        })
    };

    let query = search_query.to_lowercase();
    
    let solid_icons: Vec<(&str, &fontawesome::Icon)> = vec![
        ("solid::HOUSE", &icons::solid::HOUSE),
        ("solid::USER", &icons::solid::USER),
        ("solid::HEART", &icons::solid::HEART),
        ("solid::STAR", &icons::solid::STAR),
        ("solid::MAGNIFYING_GLASS", &icons::solid::MAGNIFYING_GLASS),
        ("solid::GEAR", &icons::solid::GEAR),
        ("solid::BELL", &icons::solid::BELL),
        ("solid::ENVELOPE", &icons::solid::ENVELOPE),
        ("solid::CALENDAR", &icons::solid::CALENDAR),
        ("solid::CHECK", &icons::solid::CHECK),
        ("solid::XMARK", &icons::solid::XMARK),
        ("solid::TRASH", &icons::solid::TRASH),
        ("solid::PEN", &icons::solid::PEN),
        ("solid::DOWNLOAD", &icons::solid::DOWNLOAD),
        ("solid::UPLOAD", &icons::solid::UPLOAD),
        ("solid::SHARE", &icons::solid::SHARE),
        ("solid::BARS", &icons::solid::BARS),
        ("solid::PLUS", &icons::solid::PLUS),
        ("solid::MINUS", &icons::solid::MINUS),
        ("solid::ARROW_LEFT", &icons::solid::ARROW_LEFT),
    ];

    let regular_icons: Vec<(&str, &fontawesome::Icon)> = vec![
        ("regular::HEART", &icons::regular::HEART),
        ("regular::STAR", &icons::regular::STAR),
        ("regular::ENVELOPE", &icons::regular::ENVELOPE),
        ("regular::CALENDAR", &icons::regular::CALENDAR),
        ("regular::BELL", &icons::regular::BELL),
        ("regular::USER", &icons::regular::USER),
        ("regular::CIRCLE", &icons::regular::CIRCLE),
        ("regular::SQUARE", &icons::regular::SQUARE),
        ("regular::EYE", &icons::regular::EYE),
        ("regular::EYE_SLASH", &icons::regular::EYE_SLASH),
    ];

    let brands_icons: Vec<(&str, &fontawesome::Icon)> = vec![
        ("brands::GITHUB", &icons::brands::GITHUB),
        ("brands::TWITTER", &icons::brands::TWITTER),
        ("brands::FACEBOOK", &icons::brands::FACEBOOK),
        ("brands::GOOGLE", &icons::brands::GOOGLE),
        ("brands::LINKEDIN", &icons::brands::LINKEDIN),
        ("brands::YOUTUBE", &icons::brands::YOUTUBE),
        ("brands::INSTAGRAM", &icons::brands::INSTAGRAM),
        ("brands::DISCORD", &icons::brands::DISCORD),
        ("brands::RUST", &icons::brands::RUST),
        ("brands::REACT", &icons::brands::REACT),
    ];

    html! {
        <>
            <Global css={css!(
                r#"
                * {
                    box-sizing: border-box;
                }
                
                body {
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
                    line-height: 1.6;
                    color: #333;
                    background-color: #f5f5f5;
                    margin: 0;
                    padding: 0;
                }
                
                .container {
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 2rem;
                }
                
                h1, h2 {
                    color: #2c3e50;
                }
                
                .search-container {
                    margin: 2rem 0;
                }
                
                .search-input {
                    width: 100%;
                    padding: 1rem;
                    font-size: 1.1rem;
                    border: 2px solid #ddd;
                    border-radius: 8px;
                    transition: border-color 0.3s;
                }
                
                .search-input:focus {
                    outline: none;
                    border-color: #3498db;
                }
                
                .icon-section {
                    margin: 3rem 0;
                }
                
                .icon-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
                    gap: 1.5rem;
                    margin-top: 1.5rem;
                }
                
                .icon-card {
                    background: white;
                    border-radius: 8px;
                    padding: 1.5rem;
                    text-align: center;
                    cursor: pointer;
                    transition: all 0.3s;
                    position: relative;
                    border: 2px solid transparent;
                }
                
                .icon-card:hover {
                    transform: translateY(-4px);
                    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
                    border-color: #3498db;
                }
                
                .icon-name {
                    font-size: 0.85rem;
                    color: #666;
                    word-break: break-word;
                    margin-top: 0.5rem;
                }
                
                .copied-badge {
                    position: absolute;
                    top: -10px;
                    right: -10px;
                    background: #27ae60;
                    color: white;
                    padding: 0.25rem 0.5rem;
                    border-radius: 4px;
                    font-size: 0.75rem;
                    font-weight: bold;
                }
                
                .info-box {
                    background: #e3f2fd;
                    border-left: 4px solid #2196f3;
                    padding: 1rem;
                    margin: 2rem 0;
                    border-radius: 4px;
                }
                
                code {
                    background: #f5f5f5;
                    padding: 0.2rem 0.4rem;
                    border-radius: 3px;
                    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
                    font-size: 0.9em;
                }
                
                pre {
                    background: #f5f5f5;
                    padding: 1rem;
                    border-radius: 4px;
                    overflow-x: auto;
                    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
                    font-size: 0.9rem;
                    line-height: 1.4;
                    margin: 1rem 0;
                }
                "#
            )} />
            
            <div class="container">
                <h1>{"yew-shortcuts FontAwesome Icons"}</h1>
                <p style="font-size: 1.2rem; color: #666; margin-top: -0.5rem;">{"Compile-time SVG icons for Yew - Zero runtime overhead!"}</p>
                
                <div class="info-box">
                    <p>{"🚀 "}<strong>{"Zero runtime overhead!"}</strong>{" All icons are compile-time constants."}</p>
                    <p>{"📦 "}<strong>{"No WASM bloat!"}</strong>{" Only the icons you actually use are included in your final binary."}</p>
                    <p>{"⚡ "}<strong>{"2,060 icons available"}</strong>{" at compile time, but you pay only for what you use!"}</p>
                    <p>{"💡 Click any icon below to copy its usage code to clipboard."}</p>
                </div>
                
                <h2>{"How it works"}</h2>
                <div style="margin-bottom: 2rem;">
                    <p>{"Each icon is defined as a "}<code>{"const"}</code>{" with its SVG path data:"}</p>
                    <pre style="background: #f5f5f5; padding: 1rem; border-radius: 4px; overflow-x: auto;">
{r#"pub const HOUSE: Icon = Icon {
    view_box: "0 0 576 512",
    d: "M575.8 255.5c0 18-15 32.1-32 32.1l-32 0 .7 160.2c..."
};"#}
                    </pre>
                    <p>{"The Rust compiler's dead code elimination ensures that "}<strong>{"only the icons you import and use"}</strong>{" are included in the final WASM binary. Unused icons are completely eliminated at compile time!"}</p>
                </div>

                <div class="search-container">
                    <input
                        type="text"
                        class="search-input"
                        placeholder="Search icons..."
                        value={(*search_query).clone()}
                        oninput={on_search_input}
                    />
                </div>
                
                <div class="icon-section">
                    <h2>{"Solid Icons"}</h2>
                    <div class="icon-grid">
                        {solid_icons.into_iter()
                            .filter(|(name, _)| query.is_empty() || name.to_lowercase().contains(&query))
                            .map(|(name, icon)| {
                                let icon_name = name.to_string();
                                let is_copied = (*copied_icon).as_ref() == Some(&icon_name);
                                let onclick = {
                                    let icon_name = icon_name.clone();
                                    let copy_to_clipboard = copy_to_clipboard.clone();
                                    Callback::from(move |_| copy_to_clipboard.emit(icon_name.clone()))
                                };
                                
                                html! {
                                    <div class="icon-card" {onclick}>
                                        <FontAwesomeSvg {icon} style="font-size: 2rem; margin-bottom: 0.5rem;" />
                                        <div class="icon-name">{name}</div>
                                        {if is_copied {
                                            html! { <div class="copied-badge">{"Copied!"}</div> }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                }
                            })
                            .collect::<Html>()
                        }
                    </div>
                </div>
                
                <div class="icon-section">
                    <h2>{"Regular Icons"}</h2>
                    <div class="icon-grid">
                        {regular_icons.into_iter()
                            .filter(|(name, _)| query.is_empty() || name.to_lowercase().contains(&query))
                            .map(|(name, icon)| {
                                let icon_name = name.to_string();
                                let is_copied = (*copied_icon).as_ref() == Some(&icon_name);
                                let onclick = {
                                    let icon_name = icon_name.clone();
                                    let copy_to_clipboard = copy_to_clipboard.clone();
                                    Callback::from(move |_| copy_to_clipboard.emit(icon_name.clone()))
                                };
                                
                                html! {
                                    <div class="icon-card" {onclick}>
                                        <FontAwesomeSvg {icon} style="font-size: 2rem; margin-bottom: 0.5rem;" />
                                        <div class="icon-name">{name}</div>
                                        {if is_copied {
                                            html! { <div class="copied-badge">{"Copied!"}</div> }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                }
                            })
                            .collect::<Html>()
                        }
                    </div>
                </div>
                
                <div class="icon-section">
                    <h2>{"Brand Icons"}</h2>
                    <div class="icon-grid">
                        {brands_icons.into_iter()
                            .filter(|(name, _)| query.is_empty() || name.to_lowercase().contains(&query))
                            .map(|(name, icon)| {
                                let icon_name = name.to_string();
                                let is_copied = (*copied_icon).as_ref() == Some(&icon_name);
                                let onclick = {
                                    let icon_name = icon_name.clone();
                                    let copy_to_clipboard = copy_to_clipboard.clone();
                                    Callback::from(move |_| copy_to_clipboard.emit(icon_name.clone()))
                                };
                                
                                html! {
                                    <div class="icon-card" {onclick}>
                                        <FontAwesomeSvg {icon} style="font-size: 2rem; margin-bottom: 0.5rem;" />
                                        <div class="icon-name">{name}</div>
                                        {if is_copied {
                                            html! { <div class="copied-badge">{"Copied!"}</div> }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                }
                            })
                            .collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        </>
    }
}