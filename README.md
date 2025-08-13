# yew-shortcuts

[![Demo](https://img.shields.io/badge/demo-live-brightgreen)](https://madoshakalaka.github.io/yew-shortcuts/)

Productivity macros for Yew applications. Stop typing the same boilerplate over and over!

## Features

- `cs!` - Clone multiple variables at once for closures
- `#[comp]` - Combines `#[yew_autoprops::autoprops]` and `#[yew::function_component]`
- **FontAwesome Icons** - 2806 compile-time SVG icons with zero runtime overhead!

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
yew-shortcuts = { git = "https://github.com/Madoshakalaka/yew-shortcuts" }
```

## Usage

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

### FontAwesome Icons

yew-shortcuts includes all 2,806 FontAwesome Free 7.0 icons as compile-time constants. **Only the icons you actually use are included in your final WASM binary** - unused icons are eliminated by the Rust compiler's dead code elimination.

```rust
use yew_shortcuts::fontawesome::{icons, FontAwesomeSvg};

// Only this icon will be included in your final binary!
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

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

FontAwesome comes with a very permissive license that allows them to use it in commercial applications as long an attribution in the svg data is preserved. This crate does it by preserving the attribution in the `data-fa-license` attribute of the svg element.
