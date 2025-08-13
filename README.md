# yew-shortcuts

[![Demo](https://img.shields.io/badge/demo-live-brightgreen)](https://madoshakalaka.github.io/yew-shortcuts/)

Productivity macros for Yew applications. Stop typing the same boilerplate over and over!

## Features

- **FontAwesome Icons** - 2806 compile-time SVG icons with zero runtime overhead!
- `cs!` - Clone multiple variables at once for closures
- `#[comp]` - Combines `#[yew_autoprops::autoprops]` and `#[yew::function_component]`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
yew-shortcuts = { git = "https://github.com/Madoshakalaka/yew-shortcuts" }
```

## Usage

### FontAwesome Icons

yew-shortcuts includes all 2,806 FontAwesome Free 7.0 icons as compile-time constants. Only the icons you actually use are included in your final WASM binary - unused icons are eliminated by the Rust compiler's dead code elimination.

```rust
use yew_shortcuts::fontawesome::{icons, FontAwesomeSvg};

// Only this icon will be included in your final binary
html! {
    <FontAwesomeSvg icon={&icons::solid::HOUSE} />
}
```

#### Cropped vs Full SVG Modes

Icons support two rendering modes:

- **Cropped (default)** - Tight viewBox that fits exactly to the icon content.
- **Full** - Standard 640×640 viewBox with padding. Useful when you need consistent icon boundaries.

To use full SVG mode, enable the feature and use the `full` prop:

```toml
# Cargo.toml
[dependencies]
yew-shortcuts = { git = "https://github.com/Madoshakalaka/yew-shortcuts", features = ["full-svg"] }
```

```rust
// Use full 640×640 viewBox
html! {
    <FontAwesomeSvg icon={&icons::solid::HOUSE} full=true />
}
```

Use the [live demo](https://madoshakalaka.github.io/yew-shortcuts/) to browse and search for all available icons!


#### MCP Icon Server for yew-shortcuts

An MCP (Model Context Protocol) server that provides FontAwesome icon search and code generation capabilities for yew-shortcuts. Designed for use with Claude Code and other MCP-compatible clients.

features include:

- **Icon Search**: Fuzzy search across all free FontAwesome icons
- **Code Generation**: Generate ready-to-use Yew component code
- **Icon Details**: Get complete icon information including dimensions and SVG data
- **Category Listing**: Browse icons by category (solid, regular, brands)

to install run the installation script from the mcp-icon-server directory:

```bash
cd mcp-icon-server
./install.sh
```

This will:
1. Build the MCP server in release mode
2. Install the binary to `/usr/local/bin`
3. Display the command to add the server to Claude Code

Then you can prompt the agent with:

```
search for house related yew-shortcuts icons
```

the mcp will return the component props and svg viewbox to the agent so you can prompt with:

```
use the regular house icon and style it with rem proportially to the viewbox
```

### The `cs!` macro

Instead of writing:
```rust
let state = state.clone();
let onclick = onclick.clone();
let name = name.clone();
```

Just write:
```rust
use yew_shortcuts::cs;

cs!(state, onclick, name);
```

### The `#[comp]` attribute

Instead of:
```rust
#[yew_autoprops::autoprops]
#[yew::function_component]
fn MyComponent(name: &str) -> Html {
    html! { <div>{name}</div> }
}
```

Just write:
```rust
use yew_shortcuts::comp;

#[comp]
fn MyComponent(name: &str) -> Html {
    html! { <div>{name}</div> }
}
```


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

FontAwesome comes with a very permissive license that allows them to use it in commercial applications as long an attribution in the svg data is preserved. This crate does it by preserving the attribution in the `data-fa-license` attribute of the svg element.
