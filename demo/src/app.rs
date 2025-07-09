use stylist::{css, yew::Global};
use yew::prelude::*;
use yew_shortcuts::fontawesome::{self, FontAwesomeSvg, icons};
use crate::all_icons::{SOLID_ICONS, REGULAR_ICONS, BRANDS_ICONS};

const ICONS_PER_PAGE: usize = 100;

#[derive(Clone, Copy, PartialEq)]
enum IconCategory {
    Solid,
    Regular,
    Brands,
}

impl IconCategory {
    fn name(&self) -> &'static str {
        match self {
            IconCategory::Solid => "Solid",
            IconCategory::Regular => "Regular",
            IconCategory::Brands => "Brands",
        }
    }
    
    fn icons(&self) -> &'static [(&'static str, &'static fontawesome::Icon)] {
        match self {
            IconCategory::Solid => SOLID_ICONS,
            IconCategory::Regular => REGULAR_ICONS,
            IconCategory::Brands => BRANDS_ICONS,
        }
    }
    
    fn count(&self) -> usize {
        self.icons().len()
    }
}

#[function_component]
pub fn App() -> Html {
    let search_query = use_state(String::new);
    let copied_icon = use_state(|| None::<String>);
    let current_category = use_state(|| IconCategory::Solid);
    let current_page = use_state(|| 0usize);

    let on_search_input = {
        let search_query = search_query.clone();
        let current_page = current_page.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_query.set(input.value());
            current_page.set(0); // Reset to first page on search
        })
    };

    let copy_to_clipboard = {
        let copied_icon = copied_icon.clone();
        Callback::from(move |(icon_name, code): (String, String)| {
            let window = web_sys::window().unwrap();
            let navigator = window.navigator();
            let clipboard = navigator.clipboard();
            
            let promise = clipboard.write_text(&code);
            
            copied_icon.set(Some(icon_name));
            
            let copied_icon = copied_icon.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                gloo::timers::future::TimeoutFuture::new(3000).await;
                copied_icon.set(None);
            });
        })
    };

    let query = search_query.to_lowercase();
    
    // Filter icons based on search
    let filtered_icons: Vec<(&str, &fontawesome::Icon)> = current_category
        .icons()
        .iter()
        .filter(|(name, _)| query.is_empty() || name.to_lowercase().contains(&query))
        .cloned()
        .collect();
    
    let total_pages = (filtered_icons.len() + ICONS_PER_PAGE - 1) / ICONS_PER_PAGE;
    let current_page_num = *current_page;
    
    // Get icons for current page
    let start_idx = current_page_num * ICONS_PER_PAGE;
    let end_idx = (start_idx + ICONS_PER_PAGE).min(filtered_icons.len());
    let page_icons = &filtered_icons[start_idx..end_idx];

    let on_category_change = {
        let current_category = current_category.clone();
        let current_page = current_page.clone();
        Callback::from(move |category: IconCategory| {
            current_category.set(category);
            current_page.set(0); // Reset to first page on category change
        })
    };

    let on_page_change = {
        let current_page = current_page.clone();
        Callback::from(move |page: usize| {
            current_page.set(page);
            // Scroll to top
            web_sys::window().unwrap().scroll_to_with_x_and_y(0.0, 0.0);
        })
    };

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
                    background-color: #f8f9fa;
                    margin: 0;
                    padding: 0;
                }
                
                .header {
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    color: white;
                    padding: 3rem 0;
                    text-align: center;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                }
                
                .header h1 {
                    margin: 0;
                    font-size: 2.5rem;
                    font-weight: 700;
                }
                
                .header p {
                    margin: 0.5rem 0 0 0;
                    font-size: 1.2rem;
                    opacity: 0.95;
                }
                
                .header a {
                    color: white;
                    text-decoration: none;
                    border-bottom: 2px solid rgba(255,255,255,0.5);
                    transition: border-color 0.3s;
                }
                
                .header a:hover {
                    border-color: white;
                }
                
                .container {
                    max-width: 1400px;
                    margin: 0 auto;
                    padding: 2rem;
                }
                
                .info-cards {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                    gap: 1.5rem;
                    margin: 2rem auto;
                    max-width: 1000px;
                    padding: 0 2rem;
                }
                
                .info-card {
                    background: white;
                    padding: 1.5rem;
                    border-radius: 12px;
                    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
                    text-align: center;
                }
                
                .info-card h3 {
                    margin: 0 0 0.5rem 0;
                    color: #667eea;
                    font-size: 1.5rem;
                }
                
                .info-card p {
                    margin: 0;
                    color: #666;
                }
                
                .controls {
                    background: white;
                    padding: 2rem;
                    border-radius: 12px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
                    margin-bottom: 2rem;
                }
                
                .category-tabs {
                    display: flex;
                    gap: 1rem;
                    margin-bottom: 2rem;
                    flex-wrap: wrap;
                }
                
                .category-tab {
                    padding: 0.75rem 1.5rem;
                    border: 2px solid #e2e8f0;
                    background: white;
                    border-radius: 8px;
                    cursor: pointer;
                    transition: all 0.3s;
                    font-weight: 500;
                }
                
                .category-tab:hover {
                    border-color: #667eea;
                    background: #f8f9ff;
                }
                
                .category-tab.active {
                    background: #667eea;
                    color: white;
                    border-color: #667eea;
                }
                
                .search-container {
                    position: relative;
                }
                
                .search-input {
                    width: 100%;
                    padding: 1rem 1rem 1rem 3rem;
                    font-size: 1.1rem;
                    border: 2px solid #e2e8f0;
                    border-radius: 8px;
                    transition: border-color 0.3s;
                }
                
                .search-input:focus {
                    outline: none;
                    border-color: #667eea;
                }
                
                .search-icon {
                    position: absolute;
                    left: 1rem;
                    top: 50%;
                    transform: translateY(-50%);
                    color: #94a3b8;
                    pointer-events: none;
                    width: 1.2rem;
                    height: 1.2rem;
                }
                
                .results-info {
                    margin: 1rem 0;
                    color: #666;
                    text-align: center;
                }
                
                h2 {
                    color: #2c3e50;
                    margin: 2rem 0 1rem 0;
                }
                
                .icon-grid {
                    display: grid;
                    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
                    gap: 1rem;
                    margin-bottom: 3rem;
                }
                
                .icon-card {
                    background: white;
                    border-radius: 12px;
                    padding: 1.5rem;
                    text-align: center;
                    cursor: pointer;
                    transition: all 0.3s;
                    position: relative;
                    border: 2px solid transparent;
                    min-height: 120px;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                }
                
                .icon-card:hover {
                    transform: translateY(-4px);
                    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
                    border-color: #667eea;
                }
                
                .icon-name {
                    font-size: 0.75rem;
                    color: #64748b;
                    word-break: break-word;
                    margin-top: 0.75rem;
                    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
                }
                
                .copied-badge {
                    position: fixed;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    background: #10b981;
                    color: white;
                    padding: 1.5rem 2rem;
                    border-radius: 12px;
                    font-size: 0.9rem;
                    font-weight: 500;
                    box-shadow: 0 10px 25px rgba(0,0,0,0.2);
                    z-index: 1000;
                    max-width: 90%;
                    text-align: center;
                }
                
                .copied-badge code {
                    display: block;
                    margin-top: 0.5rem;
                    background: rgba(255,255,255,0.2);
                    padding: 0.5rem;
                    border-radius: 6px;
                    font-size: 0.85rem;
                    white-space: pre-wrap;
                    color: white;
                }
                
                .pagination {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    gap: 0.5rem;
                    margin-top: 3rem;
                    flex-wrap: wrap;
                }
                
                .pagination button {
                    padding: 0.5rem 1rem;
                    border: 2px solid #e2e8f0;
                    background: white;
                    border-radius: 6px;
                    cursor: pointer;
                    transition: all 0.3s;
                    font-weight: 500;
                }
                
                .pagination button:hover:not(:disabled) {
                    border-color: #667eea;
                    background: #f8f9ff;
                }
                
                .pagination button:disabled {
                    opacity: 0.5;
                    cursor: not-allowed;
                }
                
                .pagination button.active {
                    background: #667eea;
                    color: white;
                    border-color: #667eea;
                }
                
                .pagination .page-info {
                    margin: 0 1rem;
                    color: #666;
                }
                
                code {
                    background: #f1f5f9;
                    padding: 0.2rem 0.4rem;
                    border-radius: 4px;
                    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
                    font-size: 0.9em;
                    color: #0f172a;
                }
                
                pre {
                    background: #f8fafc;
                    padding: 1rem;
                    border-radius: 8px;
                    overflow-x: auto;
                    font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
                    font-size: 0.9rem;
                    line-height: 1.5;
                    margin: 1rem 0;
                    border: 1px solid #e2e8f0;
                }
                
                .how-it-works {
                    background: white;
                    padding: 2rem;
                    border-radius: 12px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
                    margin: 2rem 0;
                }
                
                .how-it-works h2 {
                    margin-top: 0;
                }
                "#
            )} />
            
            <header class="header">
                <h1>{"yew-shortcuts FontAwesome Icons"}</h1>
                <p>{"Compile-time SVG icons for Yew - Zero runtime overhead!"}</p>
                <p style="margin-top: 1rem;">
                    <a href="https://github.com/Madoshakalaka/yew-shortcuts" target="_blank">{"View on GitHub"}</a>
                </p>
            </header>
            
            <div class="info-cards">
                <div class="info-card">
                    <h3>{"🚀 Zero Overhead"}</h3>
                    <p>{"All icons are compile-time constants"}</p>
                </div>
                <div class="info-card">
                    <h3>{"📦 No WASM Bloat"}</h3>
                    <p>{"Only used icons are included"}</p>
                </div>
                <div class="info-card">
                    <h3>{"⚡ 2,060 Icons"}</h3>
                    <p>{"Available at compile time"}</p>
                </div>
            </div>
            
            <div class="container">
                <div class="how-it-works">
                    <h2>{"How it works"}</h2>
                    <p>{"Each icon is defined as a "}<code>{"const"}</code>{" with its SVG path data:"}</p>
                    <pre>
{r#"pub const HOUSE: &Icon = &Icon {
    view_box: "0 0 576 512",
    d: "M575.8 255.5c0 18-15 32.1-32 32.1l-32 0 .7 160.2c..."
};"#}
                    </pre>
                    <p>{"The Rust compiler's dead code elimination ensures that "}<strong>{"only the icons you import and use"}</strong>{" are included in the final WASM binary. Unused icons are completely eliminated at compile time!"}</p>
                </div>
                
                <div class="controls">
                    <div class="category-tabs">
                        {[IconCategory::Solid, IconCategory::Regular, IconCategory::Brands].into_iter().map(|category| {
                            let is_active = *current_category == category;
                            let onclick = {
                                let on_category_change = on_category_change.clone();
                                Callback::from(move |_| on_category_change.emit(category))
                            };
                            
                            html! {
                                <button
                                    class={classes!("category-tab", is_active.then(|| "active"))}
                                    {onclick}
                                >
                                    {format!("{} ({})", category.name(), category.count())}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                    
                    <div class="search-container">
                        <FontAwesomeSvg 
                            icon={&icons::solid::MAGNIFYING_GLASS} 
                            classes={classes!("search-icon")}
                        />
                        <input
                            type="text"
                            class="search-input"
                            placeholder={format!("Search {} icons...", current_category.name().to_lowercase())}
                            value={(*search_query).clone()}
                            oninput={on_search_input}
                        />
                    </div>
                    
                    <div class="results-info">
                        {if !query.is_empty() {
                            html! {
                                <p>{format!("Found {} icons matching \"{}\"", filtered_icons.len(), query)}</p>
                            }
                        } else {
                            html! {
                                <p>{format!("Showing {} of {} icons", 
                                    page_icons.len(), 
                                    filtered_icons.len()
                                )}</p>
                            }
                        }}
                    </div>
                </div>
                
                <div class="icon-grid">
                    {page_icons.iter().map(|(name, icon)| {
                        let full_icon_name = format!("{}::{}", current_category.name().to_lowercase(), name);
                        let code = format!("use yew_shortcuts::fontawesome::icons::{};\n\n<FontAwesomeSvg icon={{&icons::{}::{}}} />", 
                            current_category.name().to_lowercase(), 
                            current_category.name().to_lowercase(), 
                            name
                        );
                        
                        let onclick = {
                            let full_icon_name = full_icon_name.clone();
                            let code = code.clone();
                            let copy_to_clipboard = copy_to_clipboard.clone();
                            Callback::from(move |e: MouseEvent| {
                                e.stop_propagation();
                                copy_to_clipboard.emit((full_icon_name.clone(), code.clone()))
                            })
                        };
                        
                        
                        html! {
                            <div class="icon-card" onclick={onclick.clone()} title={format!("Click to copy {}", name)}>
                                <FontAwesomeSvg icon={icon} style="font-size: 2rem;" onclick={onclick.clone()} />
                                <div class="icon-name" onclick={onclick}>{name}</div>
                            </div>
                        }
                    }).collect::<Html>()}
                </div>
                
                {if let Some(copied_name) = &*copied_icon {
                    let code = if copied_name.starts_with("solid::") {
                        format!("use yew_shortcuts::fontawesome::icons::solid;\n\n<FontAwesomeSvg icon={{&solid::{}}} />", 
                            copied_name.strip_prefix("solid::").unwrap()
                        )
                    } else if copied_name.starts_with("regular::") {
                        format!("use yew_shortcuts::fontawesome::icons::regular;\n\n<FontAwesomeSvg icon={{&regular::{}}} />", 
                            copied_name.strip_prefix("regular::").unwrap()
                        )
                    } else if copied_name.starts_with("brands::") {
                        format!("use yew_shortcuts::fontawesome::icons::brands;\n\n<FontAwesomeSvg icon={{&brands::{}}} />", 
                            copied_name.strip_prefix("brands::").unwrap()
                        )
                    } else {
                        String::new()
                    };
                    
                    html! {
                        <div class="copied-badge">
                            <div>{"✓ Copied to clipboard!"}</div>
                            <code>{code}</code>
                        </div>
                    }
                } else {
                    html! {}
                }}
                
                {if total_pages > 1 {
                    html! {
                        <div class="pagination">
                            <button 
                                onclick={
                                    let on_page_change = on_page_change.clone();
                                    Callback::from(move |_| on_page_change.emit(0))
                                }
                                disabled={current_page_num == 0}
                            >
                                {"First"}
                            </button>
                            <button 
                                onclick={
                                    let on_page_change = on_page_change.clone();
                                    Callback::from(move |_| {
                                        if current_page_num > 0 {
                                            on_page_change.emit(current_page_num - 1)
                                        }
                                    })
                                }
                                disabled={current_page_num == 0}
                            >
                                {"Previous"}
                            </button>
                            
                            <span class="page-info">
                                {format!("Page {} of {}", current_page_num + 1, total_pages)}
                            </span>
                            
                            <button 
                                onclick={
                                    let on_page_change = on_page_change.clone();
                                    let total_pages = total_pages;
                                    Callback::from(move |_| {
                                        if current_page_num < total_pages - 1 {
                                            on_page_change.emit(current_page_num + 1)
                                        }
                                    })
                                }
                                disabled={current_page_num >= total_pages - 1}
                            >
                                {"Next"}
                            </button>
                            <button 
                                onclick={
                                    let on_page_change = on_page_change.clone();
                                    let total_pages = total_pages;
                                    Callback::from(move |_| on_page_change.emit(total_pages - 1))
                                }
                                disabled={current_page_num >= total_pages - 1}
                            >
                                {"Last"}
                            </button>
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </>
    }
}
