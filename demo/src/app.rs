use stylist::{yew::{styled_component, Global}, css};
use yew::prelude::*;
use yew_html_ext::html;
use yew_shortcuts::fontawesome::{FontAwesomeSvg, icons};

#[styled_component]
pub fn App() -> Html {
    let search_query = use_state(|| String::new());
    
    let icon_section_style = css!(
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
        gap: 1.5rem;
        margin: 2rem 0;
    );
    
    let icon_card_style = css!(
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.5rem;
        padding: 1rem;
        border: 1px solid ${"#e0e0e0"};
        border-radius: 8px;
        transition: all 0.2s ease;
        cursor: pointer;
        :hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 12px ${"rgba(0, 0, 0, 0.1)"};
            border-color: ${"#4a90e2"};
        }
    );
    
    let icon_name_style = css!(
        font-size: 0.75rem;
        color: ${"#666"};
        text-align: center;
        word-break: break-word;
    );
    
    let header_style = css!(
        background: linear-gradient(135deg, ${"#667eea 0%, #764ba2 100%"});
        color: white;
        padding: 3rem 2rem;
        text-align: center;
        margin-bottom: 2rem;
    );
    
    let container_style = css!(
        max-width: 1200px;
        margin: 0 auto;
        padding: 0 2rem;
    );
    
    let search_style = css!(
        width: 100%;
        max-width: 500px;
        margin: 2rem auto;
        display: flex;
        gap: 1rem;
        align-items: center;
    );
    
    let search_input_style = css!(
        flex: 1;
        padding: 0.75rem 1rem;
        border: 2px solid ${"#e0e0e0"};
        border-radius: 8px;
        font-size: 1rem;
        outline: none;
        transition: border-color 0.2s;
        :focus {
            border-color: ${"#4a90e2"};
        }
    );
    
    let on_search_input = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            search_query.set(input.value());
        })
    };
    
    let filter = (*search_query).to_lowercase();
    
    // Sample icons to showcase - mix of solid, regular, and brands
    let showcase_icons = vec![
        ("solid::HOUSE", icons::solid::HOUSE),
        ("solid::USER", icons::solid::USER),
        ("solid::HEART", icons::solid::HEART),
        ("solid::STAR", icons::solid::STAR),
        ("solid::MAGNIFYING_GLASS", icons::solid::MAGNIFYING_GLASS),
        ("solid::GEAR", icons::solid::GEAR),
        ("solid::ENVELOPE", icons::solid::ENVELOPE),
        ("solid::BELL", icons::solid::BELL),
        ("solid::BOOKMARK", icons::solid::BOOKMARK),
        ("solid::CALENDAR", icons::solid::CALENDAR),
        ("solid::CAMERA", icons::solid::CAMERA),
        ("solid::CART_SHOPPING", icons::solid::CART_SHOPPING),
        ("solid::CHECK", icons::solid::CHECK),
        ("solid::CIRCLE_INFO", icons::solid::CIRCLE_INFO),
        ("solid::CLOUD", icons::solid::CLOUD),
        ("solid::DOWNLOAD", icons::solid::DOWNLOAD),
        ("solid::FILE", icons::solid::FILE),
        ("solid::FOLDER", icons::solid::FOLDER),
        ("solid::IMAGE", icons::solid::IMAGE),
        ("solid::LOCK", icons::solid::LOCK),
        ("solid::MUSIC", icons::solid::MUSIC),
        ("solid::PAPER_PLANE", icons::solid::PAPER_PLANE),
        ("solid::PHONE", icons::solid::PHONE),
        ("solid::PLAY", icons::solid::PLAY),
        ("solid::PLUS", icons::solid::PLUS),
        ("solid::TRASH", icons::solid::TRASH),
        ("solid::VIDEO", icons::solid::VIDEO),
        ("solid::WIFI", icons::solid::WIFI),
        ("regular::HEART", icons::regular::HEART),
        ("regular::STAR", icons::regular::STAR),
        ("regular::ENVELOPE", icons::regular::ENVELOPE),
        ("regular::BELL", icons::regular::BELL),
        ("regular::BOOKMARK", icons::regular::BOOKMARK),
        ("regular::CALENDAR", icons::regular::CALENDAR),
        ("regular::CIRCLE_QUESTION", icons::regular::CIRCLE_QUESTION),
        ("regular::COMMENT", icons::regular::COMMENT),
        ("regular::EYE", icons::regular::EYE),
        ("regular::EYE_SLASH", icons::regular::EYE_SLASH),
        ("regular::FILE", icons::regular::FILE),
        ("regular::FOLDER", icons::regular::FOLDER),
        ("regular::IMAGE", icons::regular::IMAGE),
        ("regular::THUMBS_UP", icons::regular::THUMBS_UP),
        ("regular::THUMBS_DOWN", icons::regular::THUMBS_DOWN),
        ("brands::GITHUB", icons::brands::GITHUB),
        ("brands::TWITTER", icons::brands::TWITTER),
        ("brands::FACEBOOK", icons::brands::FACEBOOK),
        ("brands::LINKEDIN", icons::brands::LINKEDIN),
        ("brands::YOUTUBE", icons::brands::YOUTUBE),
        ("brands::INSTAGRAM", icons::brands::INSTAGRAM),
        ("brands::DISCORD", icons::brands::DISCORD),
        ("brands::REDDIT", icons::brands::REDDIT),
        ("brands::CHROME", icons::brands::CHROME),
        ("brands::FIREFOX", icons::brands::FIREFOX),
        ("brands::APPLE", icons::brands::APPLE),
        ("brands::WINDOWS", icons::brands::WINDOWS),
        ("brands::LINUX", icons::brands::LINUX),
        ("brands::REACT", icons::brands::REACT),
        ("brands::RUST", icons::brands::RUST),
    ];
    
    let filtered_icons: Vec<_> = showcase_icons
        .into_iter()
        .filter(|(name, _)| filter.is_empty() || name.to_lowercase().contains(&filter))
        .collect();
    
    html! {
        <>
            <Global css={css!(
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }
                
                body {
                    font-family: ${"-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif"};
                    line-height: 1.6;
                    color: ${"#333"};
                    background-color: ${"#f5f5f5"};
                }
                
                a {
                    text-decoration: none;
                }
            )} />
            <div class={classes!(header_style)}>
                <h1 style="font-size: 3rem; margin: 0 0 1rem 0;">{ "yew-shortcuts" }</h1>
                <p style="font-size: 1.25rem; opacity: 0.9;">{ "FontAwesome Icons for Yew" }</p>
                <p style="font-size: 1rem; opacity: 0.8;">{ format!("Featuring {} FontAwesome 6.7.2 Free icons", 2060) }</p>
            </div>
            
            <div class={classes!(container_style)}>
                <div class={classes!(search_style)}>
                    <FontAwesomeSvg icon={icons::solid::MAGNIFYING_GLASS} height="1.5rem" style="color: #666;" />
                    <input 
                        type="text"
                        placeholder="Search icons..."
                        class={classes!(search_input_style)}
                        oninput={on_search_input}
                        value={(*search_query).clone()}
                    />
                </div>
                
                <p style="text-align: center; color: #666; margin-bottom: 2rem;">
                    { format!("Showing {} of {} sampled icons", filtered_icons.len(), 60) }
                </p>
                
                <div class={classes!(icon_section_style)}>
                    { for filtered_icons.iter().map(|(name, icon)| {
                        let icon_name = name.to_string();
                        let onclick = {
                            let icon_name = icon_name.clone();
                            Callback::from(move |_| {
                                let window = web_sys::window().unwrap();
                                let navigator = window.navigator();
                                let clipboard = navigator.clipboard();
                                let icon_import = format!("icons::{}", icon_name);
                                wasm_bindgen_futures::spawn_local({
                                    async move {
                                        let promise = clipboard.write_text(&icon_import);
                                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                                    }
                                });
                            })
                        };
                        
                        html! {
                            <div class={classes!(icon_card_style.clone())} {onclick} title="Click to copy import path">
                                <FontAwesomeSvg {icon} height="2rem" width="2rem" />
                                <span class={classes!(icon_name_style.clone())}>{ *name }</span>
                            </div>
                        }
                    })}
                </div>
                
                <div style="text-align: center; padding: 3rem 0; color: #666;">
                    <p>{ "Click any icon to copy its import path" }</p>
                    <p style="margin-top: 1rem;">
                        { "Full documentation and all " }
                        <strong>{ "2,060 icons" }</strong>
                        { " available at " }
                        <a href="https://github.com/Madoshakalaka/yew-shortcuts" style="color: #4a90e2;">
                            { "GitHub" }
                        </a>
                    </p>
                </div>
            </div>
        </>
    }
}