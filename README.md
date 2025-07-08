# yew-shortcuts

Productivity macros for Yew applications. Stop typing the same boilerplate over and over!

## Features

- `cs!` - Clone multiple variables at once for closures
- `#[comp]` - Combines `#[yew_autoprops::autoprops]` and `#[yew::function_component]`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
yew-shortcuts = { git = "https://github.com/yourusername/yew-shortcuts" }
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

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.